//! WebXR presentation through the WebXR/WebGPU binding, as three r187's
//! XRManager drives it for a WebGPU backend ( `renderers/common/XRManager.js` ):
//! an `XRGPUBinding` projection layer with one array layer per view, the
//! views' cameras ( an ArrayCamera with the union camera of two views: r186's
//! setProjectionFromUnion, which r187 removed, copying the first view's
//! projection instead; the port still builds the union ), and
//! WebXRController's target-ray and grip spaces. The browser's XR session
//! itself stays in the page's JavaScript ( `web/examples/xr.js` ), which hands
//! each frame's views, layer texture and input poses to this module.
//!
//! Rendering keeps three's WebGPU structure: each view is drawn into its own
//! layer of the session's texture, multisampled and resolved there, sorted
//! with the union camera, with one shadow update per frame. Each view culls
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
    /// The ArrayCamera's union view-projection.
    union_projection: Matrix4,
    union_world: Matrix4,
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
            union_projection: Matrix4::IDENTITY,
            union_world: Matrix4::IDENTITY,
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
    /// camera's parent, the union camera, and the user camera following it.
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
        // cameraXR.matrix is the first view's; its world, before the union
        // offset, is the first view's world.
        let (world, projection) = union_camera(worlds, projections);
        self.union_world = world;
        self.union_projection = projection;
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
    /// sorted with the union camera, shadows updated once. `draw`
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
            .set(Some(self.union_projection * self.union_world.inverse()));
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

/// setProjectionFromUnion: a camera behind both views whose frustum covers
/// them. Like three, it reads near/far and the field of view from the left
/// projection with OpenGL's clip conventions.
pub fn union_camera(worlds: [Matrix4; 2], projections: [Matrix4; 2]) -> (Matrix4, Matrix4) {
    let left_pos = worlds[0].w_axis.truncate();
    let right_pos = worlds[1].w_axis.truncate();
    let ipd = left_pos.distance(right_pos);
    let l = projections[0].to_cols_array();
    let r = projections[1].to_cols_array();
    let near = l[14] / (l[10] - 1.);
    let far = l[14] / (l[10] + 1.);
    let top_fov = (l[9] + 1.) / l[5];
    let bottom_fov = (l[9] - 1.) / l[5];
    let left_fov = (l[8] - 1.) / l[0];
    let right_fov = (r[8] + 1.) / r[0];
    let left = near * left_fov;
    let right = near * right_fov;
    let z_offset = ipd / (-left_fov + right_fov);
    let x_offset = z_offset * -left_fov;
    let (scale, rotation, position) = worlds[0].to_scale_rotation_translation();
    // camera.translateX( xOffset ); camera.translateZ( zOffset ).
    let position = position + rotation * Vector3::new(x_offset, 0., z_offset);
    let world = Matrix4::from_scale_rotation_translation(scale, rotation, position);
    if l[10] == -1.0 {
        return (world, projections[0]);
    }
    let near2 = near + z_offset;
    let far2 = far + z_offset;
    let left2 = left - x_offset;
    let right2 = right + (ipd - x_offset);
    let top2 = top_fov * far / far2 * near2;
    let bottom2 = bottom_fov * far / far2 * near2;
    (
        world,
        make_perspective(left2, right2, top2, bottom2, near2, far2),
    )
}

/// Matrix4.makePerspective in its default WebGL coordinate system, as
/// setProjectionFromUnion calls it. The union camera only orders the draws.
fn make_perspective(left: f64, right: f64, top: f64, bottom: f64, near: f64, far: f64) -> Matrix4 {
    let x = 2. * near / (right - left);
    let y = 2. * near / (top - bottom);
    let a = (right + left) / (right - left);
    let b = (top + bottom) / (top - bottom);
    let c = -(far + near) / (far - near);
    let d = -2. * far * near / (far - near);
    Matrix4::from_cols_array(&[x, 0., 0., 0., 0., y, 0., 0., a, b, c, -1., 0., 0., d, 0.])
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
    fn union_camera_matches_three() {
        // Values from three r186's setProjectionFromUnion ( r187 removed it ) for the fake device's
        // views ( tests/browser/fake-xr.js ), near 0.1 and far 10.
        let worlds = [
            Matrix4::from_translation(Vector3::new(-0.032, 1.6, 0.)),
            Matrix4::from_translation(Vector3::new(0.032, 1.6, 0.)),
        ];
        let p = [
            projection(-1., 0.9, 0.95, -1.05, 0.1, 10.),
            projection(-0.9, 1., 0.95, -1.05, 0.1, 10.),
        ];
        let (world, proj) = union_camera(worlds, p);
        let l = p[0].to_cols_array();
        let near = l[14] / (l[10] - 1.);
        let left_fov = (l[8] - 1.) / l[0];
        let right_fov = (p[1].to_cols_array()[8] + 1.) / p[1].to_cols_array()[0];
        let z = 0.064 / (-left_fov + right_fov);
        assert!((world.w_axis.z - z).abs() < 1e-12);
        assert!((world.w_axis.x - (-0.032 + z * -left_fov)).abs() < 1e-12);
        assert!(near.is_finite() && proj.is_finite());
    }
}
