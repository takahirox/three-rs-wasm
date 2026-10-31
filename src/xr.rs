//! WebXR presentation through the WebXR/WebGPU binding, as three r187's
//! XRManager drives it for a WebGPU backend ( `renderers/common/XRManager.js` ):
//! an `XRGPUBinding` projection layer with one array layer per view, the
//! views' cameras ( an ArrayCamera whose transform and projection are the
//! first view's, as r187 sets them ), and
//! WebXRController's target-ray and grip spaces. The browser's XR session
//! itself stays in the page's JavaScript ( `web/examples/xr.js` ), which hands
//! each frame's views, layer texture and input poses to this module.
//!
//! Rendering keeps three's WebGPU structure: each view is drawn into its own
//! layer of the session's texture, multisampled and resolved there, sorted
//! with the XR camera ( the first view's ), with one shadow update per frame. Each view culls
//! with its own frustum; three draws what either view's frustum holds into
//! both layers, and the draws outside a view's frustum leave no pixels.
//! There is no WebGL fallback: a session without the 'webgpu' feature is an
//! error, as in XRManager.
use crate::{
    Error, Result,
    camera::{Camera, PerspectiveCamera},
    math::*,
    render_target::{RenderTarget, RenderTargetOptions},
    renderer::Renderer,
    scene::*,
};

/// One XRView: its pose in the reference space ( `view.transform.matrix` ),
/// its projection ( `view.projectionMatrix` ) and its sub-image viewport.
#[derive(Clone, Copy, Debug)]
pub struct XrView {
    pub transform: Matrix4,
    pub projection: Matrix4,
    pub viewport: [u32; 4],
}

impl XrView {
    /// Views from column-major `transform, projection` pairs ( 32 floats per
    /// view ) and `x, y, width, height` viewports.
    pub fn from_arrays(matrices: &[f32], viewports: &[u32]) -> Result<Vec<Self>> {
        if !matrices.len().is_multiple_of(32) || viewports.len() * 8 != matrices.len() {
            return Err(Error::Invalid("XR views"));
        }
        Ok(matrices
            .chunks(32)
            .zip(viewports.chunks(4))
            .map(|(m, v)| {
                let mat =
                    |o: usize| Matrix4::from_cols_array(&std::array::from_fn(|i| m[o + i] as f64));
                XrView {
                    transform: mat(0),
                    projection: mat(16),
                    viewport: [v[0], v[1], v[2], v[3]],
                }
            })
            .collect())
    }
}

/// An input source's state as WebXRController reads it in a frame.
#[derive(Clone, Debug, Default)]
pub struct XrInputPose {
    /// `frame.getPose( inputSource.targetRaySpace, referenceSpace )`.
    pub target_ray: Option<Matrix4>,
    /// `frame.getPose( inputSource.gripSpace, referenceSpace )`.
    pub grip: Option<Matrix4>,
}

/// The connected input source's description ( the `connected` event's data ).
#[derive(Clone, Debug, Default)]
pub struct XrInputSource {
    pub handedness: String,
    pub target_ray_mode: String,
    pub profiles: Vec<String>,
    pub has_gamepad: bool,
    pub has_hand: bool,
}

/// `inputSource.gamepad`: buttons as ( pressed, touched, value ), and axes.
#[derive(Clone, Debug, Default)]
pub struct XrGamepad {
    pub buttons: Vec<(bool, bool, f64)>,
    pub axes: Vec<f64>,
}

/// A WebXRController event delivered to the page's listeners on the
/// controller's target-ray and grip spaces, in dispatch order.
#[derive(Clone, Debug)]
pub enum XrControllerEvent {
    Connected(XrInputSource),
    Disconnected,
    SelectStart,
    SelectEnd,
    Select,
    SqueezeStart,
    SqueezeEnd,
    Squeeze,
}

/// WebXRController: the target-ray and grip spaces ( Groups with
/// matrixAutoUpdate off, hidden until tracked ).
pub struct XrController {
    pub target_ray: Object3D,
    pub grip: Object3D,
    /// `targetRay.userData.isSelecting`, which the examples keep.
    pub is_selecting: bool,
    pub source: Option<XrInputSource>,
    pub gamepad: XrGamepad,
    pub events: Vec<XrControllerEvent>,
}

impl XrController {
    fn space(scene: &mut Scene) -> Object3D {
        let h = scene.insert(NodeKind::Group);
        let n = scene.get_mut(h).expect("controller space");
        n.matrix_auto_update = false;
        n.visible = false;
        h
    }
    fn new(scene: &mut Scene) -> Self {
        XrController {
            target_ray: Self::space(scene),
            grip: Self::space(scene),
            is_selecting: false,
            source: None,
            gamepad: XrGamepad::default(),
            events: Vec::new(),
        }
    }
    /// WebXRController.update: poses into the spaces' matrices and visibility.
    pub fn update(&mut self, scene: &mut Scene, pose: &XrInputPose) -> Result<()> {
        let set = |scene: &mut Scene, h: Object3D, m: Option<Matrix4>| -> Result<()> {
            let n = scene.get_mut(h)?;
            if let Some(m) = m {
                n.matrix = m;
                (n.scale, n.quaternion, n.position) = m.to_scale_rotation_translation();
                n.matrix_world_needs_update = true;
            }
            n.visible = m.is_some();
            Ok(())
        };
        if self.source.is_none() {
            scene.get_mut(self.target_ray)?.visible = false;
            scene.get_mut(self.grip)?.visible = false;
            return Ok(());
        }
        set(scene, self.grip, pose.grip)?;
        // A target ray without its own pose follows the grip.
        set(scene, self.target_ray, pose.target_ray.or(pose.grip))
    }
}

/// XRManager's presentation state.
pub struct XrManager {
    pub presenting: bool,
    /// The left and right view cameras ( unparented; their world matrices are
    /// set from the user camera's parent each frame ).
    pub cameras: Vec<Object3D>,
    /// The ArrayCamera's projection and world matrix: the first view's.
    xr_projection: Matrix4,
    xr_world: Matrix4,
    pub controllers: Vec<XrController>,
    /// The render state's depthNear/depthFar the session was last given.
    depth_range: Option<(f64, f64)>,
    /// The user camera's fov and zoom while presenting, restored at the end.
    saved_camera: Option<(Object3D, f64, f64)>,
    /// The layer texture this frame draws into, and the view targets.
    target: Option<RenderTarget>,
    pub reference_space_type: String,
}

impl XrManager {
    pub fn new() -> Self {
        XrManager {
            presenting: false,
            cameras: Vec::new(),
            xr_projection: Matrix4::IDENTITY,
            xr_world: Matrix4::IDENTITY,
            controllers: Vec::new(),
            depth_range: None,
            saved_camera: None,
            target: None,
            reference_space_type: "local-floor".into(),
        }
    }
    /// `renderer.xr.getController( index )`'s target-ray space ( and the
    /// controller behind it, created on first use ).
    pub fn controller(&mut self, scene: &mut Scene, index: usize) -> Object3D {
        while self.controllers.len() <= index {
            self.controllers.push(XrController::new(scene));
        }
        self.controllers[index].target_ray
    }
    /// `renderer.xr.getControllerGrip( index )`.
    pub fn controller_grip(&mut self, scene: &mut Scene, index: usize) -> Object3D {
        self.controller(scene, index);
        self.controllers[index].grip
    }
    /// setSession's WebGPU layer: a two-layer target over the projection
    /// layer's texture, multisampled as the renderer is, with the output
    /// conversion of `encode_srgb` ( DirectRenderPipeline's context output ) or
    /// without it ( a frame-buffer target of the renderer's output type ).
    pub fn start(
        &mut self,
        renderer: &Renderer,
        scene: &mut Scene,
        width: u32,
        height: u32,
        options: RenderTargetOptions,
    ) -> Result<()> {
        if width == 0 || height == 0 {
            return Err(Error::Invalid("XR projection layer"));
        }
        self.target = Some(RenderTarget::with_options(
            &renderer.device,
            width,
            height,
            RenderTargetOptions {
                depth: 2,
                use_array_depth_texture: true,
                store_multisampled_color_buffer: false,
                store_multisampled_depth_buffer: false,
                store_multisampled_stencil_buffer: false,
                ..options
            },
        )?);
        while self.cameras.len() < 2 {
            let h = scene.insert(NodeKind::Camera(Camera::Perspective(
                PerspectiveCamera::default(),
            )));
            scene.get_mut(h)?.matrix_auto_update = false;
            self.cameras.push(h);
        }
        self.depth_range = None;
        self.presenting = true;
        Ok(())
    }
    /// onSessionEnd: controllers disconnect; the user camera's fov and zoom
    /// return.
    pub fn end(&mut self, scene: &mut Scene) -> Result<()> {
        for c in &mut self.controllers {
            if c.source.take().is_some() {
                c.events.push(XrControllerEvent::Disconnected);
                scene.get_mut(c.target_ray)?.visible = false;
                scene.get_mut(c.grip)?.visible = false;
            }
        }
        if let Some((camera, fov, zoom)) = self.saved_camera.take()
            && let NodeKind::Camera(Camera::Perspective(p)) = &mut scene.get_mut(camera)?.kind
        {
            p.fov = fov;
            p.zoom = zoom;
            p.projection_override = None;
        }
        self.target = None;
        self.depth_range = None;
        self.presenting = false;
        Ok(())
    }
    /// The depthNear/depthFar to hand to `session.updateRenderState`, once
    /// they change ( XRManager.updateCamera ).
    pub fn depth_range(&self) -> Option<(f64, f64)> {
        self.depth_range
    }
    /// onAnimationFrame: the views' cameras.
    pub fn set_views(&mut self, scene: &mut Scene, views: &[XrView]) -> Result<()> {
        if views.len() != 2 || self.cameras.len() != 2 {
            return Err(Error::Invalid("XR views: two views expected"));
        }
        for (i, view) in views.iter().enumerate() {
            let n = scene.get_mut(self.cameras[i])?;
            n.matrix = view.transform;
            (n.scale, n.quaternion, n.position) = view.transform.to_scale_rotation_translation();
            if let NodeKind::Camera(Camera::Perspective(p)) = &mut n.kind {
                p.projection_override = Some(view.projection);
            }
        }
        Ok(())
    }
    /// XRManager.updateCamera: the views' world matrices under the user
    /// camera's parent, the XR camera copying the first view's projection,
    /// and the user camera following it.
    pub fn update_camera(&mut self, scene: &mut Scene, camera: Object3D) -> Result<()> {
        scene.update()?;
        let (near, far, parent, mask) = {
            let n = scene.get(camera)?;
            let NodeKind::Camera(Camera::Perspective(p)) = &n.kind else {
                return Err(Error::Invalid("XR user camera"));
            };
            (p.near, p.far, n.parent(), n.layers.mask)
        };
        // cameraXR sees layers 1 and 2 besides the camera's; the left view
        // leaves out 2 and the right view 1.
        let xr_mask = mask | 0b110;
        for (i, &h) in self.cameras.iter().enumerate() {
            scene.get_mut(h)?.layers.mask = xr_mask & !(if i == 0 { 0b100 } else { 0b010 });
        }
        if self.depth_range != Some((near, far)) {
            self.depth_range = Some((near, far));
        }
        let parent_world = parent
            .map(|p| scene.get(p).map(|n| n.matrix_world))
            .transpose()?
            .unwrap_or(Matrix4::IDENTITY);
        let mut worlds = [Matrix4::IDENTITY; 2];
        let mut projections = [Matrix4::IDENTITY; 2];
        for (i, &h) in self.cameras.iter().enumerate() {
            let n = scene.get_mut(h)?;
            n.matrix_world = parent_world * n.matrix;
            n.matrix_world_needs_update = false;
            n.matrix_world_auto_update = false;
            worlds[i] = n.matrix_world;
            if let NodeKind::Camera(Camera::Perspective(p)) = &mut n.kind {
                p.near = near;
                p.far = far;
                projections[i] = p.projection_override.unwrap_or(Matrix4::IDENTITY);
            }
        }
        // cameraXR.matrix is the first view's, so its world is the first
        // view's world; cameraXR.projectionMatrix.copy( cameraL.projectionMatrix ).
        let (world, projection) = (worlds[0], projections[0]);
        self.xr_world = world;
        self.xr_projection = projection;
        // updateUserCamera.
        if self.saved_camera.is_none()
            && let NodeKind::Camera(Camera::Perspective(p)) = &scene.get(camera)?.kind
        {
            self.saved_camera = Some((camera, p.fov, p.zoom));
        }
        let local = parent_world.inverse() * world;
        let n = scene.get_mut(camera)?;
        n.matrix = local;
        (n.scale, n.quaternion, n.position) = local.to_scale_rotation_translation();
        n.matrix_world_needs_update = true;
        if let NodeKind::Camera(Camera::Perspective(p)) = &mut n.kind {
            p.projection_override = Some(projection);
            p.fov = (2. * (1. / projection.y_axis.y).atan()).to_degrees();
            p.zoom = 1.;
        }
        scene.update_world_matrix(camera, false, true)?;
        Ok(())
    }
    /// Draws the frame's views into the layer texture: one pass per view,
    /// sorted with the XR camera, shadows updated once. `draw`
    /// renders the scene for a view into the given target ( a direct render,
    /// or a scene render and an output pass ).
    pub fn render(
        &mut self,
        renderer: &Renderer,
        scene: &mut Scene,
        color: wgpu::Texture,
        mut draw: impl FnMut(&Renderer, &mut Scene, Object3D, &mut RenderTarget) -> Result<()>,
    ) -> Result<()> {
        let target = self
            .target
            .as_mut()
            .ok_or(Error::Invalid("XR session target"))?;
        target.set_texture(0, color)?;
        renderer
            .array_camera_view_projection
            .set(Some(self.xr_projection * self.xr_world.inverse()));
        let shadow = scene.shadow_auto_update;
        let mut result = Ok(());
        for (i, &camera) in self.cameras.iter().enumerate() {
            if let Err(e) = target.set_layer(i as u32) {
                result = Err(e);
                break;
            }
            scene.shadow_auto_update = shadow && i == 0;
            result = draw(renderer, scene, camera, target);
            if result.is_err() {
                break;
            }
        }
        scene.shadow_auto_update = shadow;
        renderer.array_camera_view_projection.set(None);
        let _ = target.set_layer(0);
        result
    }
    pub fn target(&self) -> Option<&RenderTarget> {
        self.target.as_ref()
    }
}

impl Default for XrManager {
    fn default() -> Self {
        Self::new()
    }
}

/// `raycaster.setFromXRController( controller )`: the target ray's world
/// origin and -Z direction.
pub fn controller_ray(scene: &Scene, target_ray: Object3D) -> Result<(Vector3, Vector3)> {
    let world = scene.get(target_ray)?.matrix_world;
    let origin = world.w_axis.truncate();
    // tempMatrix.identity().extractRotation( matrixWorld ); direction ( 0, 0, -1 ).
    let rotation = Matrix4::from_cols(
        world.x_axis.truncate().normalize().extend(0.),
        world.y_axis.truncate().normalize().extend(0.),
        world.z_axis.truncate().normalize().extend(0.),
        Vector4::W,
    );
    let direction = rotation.transform_vector3(Vector3::NEG_Z).normalize();
    Ok((origin, direction))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn projection(l: f64, r: f64, t: f64, b: f64, n: f64, f: f64) -> Matrix4 {
        let mut m = [0.; 16];
        m[0] = 2. / (r - l);
        m[5] = 2. / (t - b);
        m[8] = (r + l) / (r - l);
        m[9] = (t + b) / (t - b);
        m[10] = f / (n - f);
        m[11] = -1.;
        m[14] = n * f / (n - f);
        Matrix4::from_cols_array(&m)
    }

    #[test]
    fn xr_camera_is_the_first_view() -> Result<()> {
        // r187's XRManager.updateCamera: cameraXR takes the left view's
        // matrix and projection ( setProjectionFromUnion is gone ), and the
        // user camera follows it under its parent. The fake device's views
        // ( tests/browser/fake-xr.js ), near 0.1 and far 10.
        let mut scene = Scene::new();
        let parent = scene.insert(NodeKind::Group);
        scene.get_mut(parent)?.position = Vector3::new(0., 0., 2.);
        let user = scene.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            near: 0.1,
            far: 10.,
            ..Default::default()
        })));
        scene.add(parent, user)?;
        let mut xr = XrManager::new();
        for _ in 0..2 {
            let h = scene.insert(NodeKind::Camera(Camera::Perspective(
                PerspectiveCamera::default(),
            )));
            scene.get_mut(h)?.matrix_auto_update = false;
            xr.cameras.push(h);
        }
        let views = [
            XrView {
                transform: Matrix4::from_translation(Vector3::new(-0.032, 1.6, 0.)),
                projection: projection(-1., 0.9, 0.95, -1.05, 0.1, 10.),
                viewport: [0, 0, 1, 1],
            },
            XrView {
                transform: Matrix4::from_translation(Vector3::new(0.032, 1.6, 0.)),
                projection: projection(-0.9, 1., 0.95, -1.05, 0.1, 10.),
                viewport: [1, 0, 1, 1],
            },
        ];
        xr.set_views(&mut scene, &views)?;
        xr.update_camera(&mut scene, user)?;
        let left_world = Matrix4::from_translation(Vector3::new(0., 0., 2.)) * views[0].transform;
        assert!(xr.xr_world.abs_diff_eq(left_world, 1e-12));
        assert_eq!(xr.xr_projection, views[0].projection);
        let n = scene.get(user)?;
        assert!(n.matrix_world.abs_diff_eq(left_world, 1e-12));
        assert!((n.position - Vector3::new(-0.032, 1.6, 0.)).length() < 1e-12);
        let Camera::Perspective(p) = scene.camera(user)?.0 else {
            panic!("perspective user camera");
        };
        assert_eq!(p.projection_override, Some(views[0].projection));
        assert!(
            (p.fov - (2. * (1. / views[0].projection.y_axis.y).atan()).to_degrees()).abs() < 1e-12
        );
        Ok(())
    }
}
