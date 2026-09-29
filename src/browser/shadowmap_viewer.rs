//! webgl_shadowmap_viewer: a spot and a directional light's BasicShadowMap
//! shadows on a spinning torus knot and cube, their CameraHelpers, and two
//! ShadowMapViewer HUDs drawn over the frame. Each HUD reads its light's
//! layer of the resident shadow atlas and shows it as the depth material's
//! 8-bit `1 - depth` would, with the light's name as a canvas label.
use super::controls_attributes::{
    CameraState, Controls, camera_helper, camera_state, update_camera_helper, viewport_css,
};
use crate::shader::ShaderProgram;
use crate::shadow::ShadowFilter;
use crate::tsl::{NodeMaterial, Type, WgslFn};
use crate::{
    Error, Result, camera::*, geometry::*, material::*, math::*, render_target::*, renderer::*,
    scene::*,
};
use std::f64::consts::PI;
use std::sync::Arc;
use wasm_bindgen::JsCast;

/// One ShadowMapViewer: its HUD quad and label.
struct Viewer {
    quad: Object3D,
    label: Object3D,
}
pub(super) struct Demo {
    time: f64,
    last: f64,
    /// The spin applied at the next frame: render() draws, then turns.
    spin: f64,
    torus: Object3D,
    cube: Object3D,
    shadow_cameras: [(Object3D, Object3D); 2],
    hud: Scene,
    hud_camera: Object3D,
    /// Directional first, then spot, as the example positions them.
    viewers: [Viewer; 2],
    output: Option<RenderTarget>,
    controls: Controls,
}
/// The light's name in 'Bold 20px Arial', as the viewer's label canvas.
fn label_texture(name: &str) -> Result<(Texture, f64)> {
    let fail = |_| Error::Invalid("label canvas");
    let canvas: web_sys::HtmlCanvasElement = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.create_element("canvas").ok())
        .and_then(|c| c.dyn_into().ok())
        .ok_or(Error::Invalid("label canvas"))?;
    let context: web_sys::CanvasRenderingContext2d = canvas
        .get_context("2d")
        .ok()
        .flatten()
        .and_then(|c| c.dyn_into().ok())
        .ok_or(Error::Invalid("2d context"))?;
    context.set_font("Bold 20px Arial");
    let width = context.measure_text(name).map_err(fail)?.width();
    canvas.set_width(width as u32);
    canvas.set_height(25);
    context.set_font("Bold 20px Arial");
    context.set_fill_style_str("rgba( 255, 0, 0, 1 )");
    context.fill_text(name, 0., 20.).map_err(fail)?;
    let (w, h) = (canvas.width(), canvas.height());
    let data = context
        .get_image_data(0., 0., w as f64, h as f64)
        .map_err(fail)?
        .data()
        .0;
    // CanvasTexture: no color space, mipmapped, flipped as flipY.
    let mut t = Texture::from_rgba(w, h, data, false)?;
    t.mipmap_filter = Some(Filter::Linear);
    Ok((t, width))
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.,
            near: 1.,
            far: 1000.,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(0., 15., 35.);
        n.quaternion = Quaternion::IDENTITY;
        s.background = Color::BLACK;
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0x404040),
            intensity: 3.,
        }));
        let angle = PI / 5.;
        let spot = s.insert(NodeKind::Light(Light::Spot {
            color: Color::WHITE,
            intensity: 500.,
            target: Vector3::ZERO,
            distance: 0.,
            decay: 2.,
            angle,
            penumbra: 0.3,
        }));
        let n = s.get_mut(spot)?;
        n.position = Vector3::new(10., 10., 5.);
        n.cast_shadow = true;
        n.shadow.near = 8.;
        n.shadow.far = 30.;
        n.shadow.map_size = Some(1024);
        n.shadow.filter = ShadowFilter::Basic;
        let dir = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 3.,
            target: Vector3::ZERO,
        }));
        let n = s.get_mut(dir)?;
        n.position = Vector3::new(0., 10., 0.);
        n.cast_shadow = true;
        n.shadow.near = 1.;
        n.shadow.far = 10.;
        n.shadow.extent = 15.;
        n.shadow.map_size = Some(1024);
        n.shadow.filter = ShadowFilter::Basic;
        // The shadow cameras, as the lights' shadow.camera, and their helpers.
        // CameraHelper follows the camera's world matrix but keeps the
        // projection it read when constructed: the example's near, far and
        // frustum edits, with the spot camera's default fov of 50 (the shadow
        // pass sets 2 × angle later, and the helper never updates).
        let (geometry, program) = camera_helper(r).await?;
        let mut shadow_cameras = vec![];
        for (camera, position) in [
            (
                Camera::Perspective(PerspectiveCamera {
                    fov: 50.,
                    aspect: 1.,
                    near: 8.,
                    far: 30.,
                    ..Default::default()
                }),
                Vector3::new(10., 10., 5.),
            ),
            (
                Camera::Orthographic(OrthographicCamera {
                    left: -15.,
                    right: 15.,
                    top: 15.,
                    bottom: -15.,
                    near: 1.,
                    far: 10.,
                    zoom: 1.,
                    ..Default::default()
                }),
                Vector3::new(0., 10., 0.),
            ),
        ] {
            let h = s.insert(NodeKind::Camera(camera));
            s.get_mut(h)?.position = position;
            s.look_at(h, Vector3::ZERO)?;
            let mut m = ShaderMaterial::new(program.clone());
            m.properties.tone_mapped = false;
            m.properties.fog = false;
            let helper = s.insert(NodeKind::Line(Line {
                geometry: geometry.clone(),
                material: Arc::new(Material::Shader(m)),
                segments: true,
            }));
            s.get_mut(helper)?.frustum_culled = false;
            update_camera_helper(s, h, helper)?;
            shadow_cameras.push((h, helper));
        }
        let phong = |hex: u32, specular: u32| {
            let mut m = MeshPhongMaterial::default();
            m.properties.color = Color::from_hex(hex);
            m.shininess = 150.;
            m.specular = Color::from_hex(specular);
            Arc::new(Material::Phong(m))
        };
        let red = phong(0xff0000, 0x222222);
        let torus = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(TorusKnotGeometry::build(25., 8., 75, 20, 2, 3)?),
            red.clone(),
        )));
        let n = s.get_mut(torus)?;
        n.scale = Vector3::splat(1. / 18.);
        n.position.y = 3.;
        n.cast_shadow = true;
        n.receive_shadow = true;
        let cube = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(3., 3., 3.)?),
            red,
        )));
        let n = s.get_mut(cube)?;
        n.position = Vector3::new(8., 3., 8.);
        n.cast_shadow = true;
        n.receive_shadow = true;
        let ground = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(10., 0.15, 10.)?),
            phong(0xa0adaf, 0x111111),
        )));
        let n = s.get_mut(ground)?;
        n.scale = Vector3::splat(3.);
        n.receive_shadow = true;
        // One shadow pass allocates the resident atlas the HUDs read: layer 0
        // is the spot light's, layer 1 the directional light's.
        let probe = RenderTarget::new(&r.device, 1, 1)?;
        r.render(s, c, &probe)?;
        let atlas = r.shadow_atlas().ok_or(Error::Invalid("shadow atlas"))?;
        let nearest = r.device.create_sampler(&wgpu::SamplerDescriptor::default());
        let mut hud = Scene::new();
        let (w, h, _) = viewport_css();
        let hud_camera = hud.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
            left: -w / 2.,
            right: w / 2.,
            top: h / 2.,
            bottom: -h / 2.,
            near: 1.,
            far: 10.,
            zoom: 1.,
            ..Default::default()
        })));
        hud.get_mut(hud_camera)?.position.z = 2.;
        let mut viewers = vec![];
        for (layer, name) in [(1u32, "Dir. Light"), (0, "Spot Light")] {
            let view = atlas.create_view(&wgpu::TextureViewDescriptor {
                dimension: Some(wgpu::TextureViewDimension::D2),
                aspect: wgpu::TextureAspect::DepthOnly,
                base_array_layer: layer,
                array_layer_count: Some(1),
                ..Default::default()
            });
            // The shadow map's RGBA8 color holds 1 − depth (cleared white);
            // the HUD samples it linearly and shows 1 − r, written raw.
            let color = WgslFn::new(
                "shadow_map",
                "fn shadow_map()->vec4<f32>{let size=vec2<f32>(textureDimensions(tsl_texture_0));let uv=fragment_surface.uv;let p=vec2(uv.x,1.0-uv.y)*size-0.5;let i=floor(p);let f=p-i;var s=array<f32,4>();for(var k=0;k<4;k++){let o=vec2(f32(k%2),f32(k/2));let c=vec2<i32>(clamp(i+o,vec2(0.0),size-1.0));let d=textureLoad(tsl_texture_0,c,0).r;s[k]=select(round((1.0-d)*255.0)/255.0,1.0,d>=1.0);}let r=mix(mix(s[0],s[1],f.x),mix(s[2],s[3],f.x),f.y);let v=vec3(1.0-r);return vec4(select(v,srgb_input(v),ENCODE_SRGB),1.0);}",
                &[],
                Type::Vec4,
            )?
            .call(&[]);
            let program = ShaderProgram::with_projection_and_sample_types(
                r,
                &NodeMaterial::new(color).wgsl_with_texture_types(&[Type::Texture], &[])?,
                &[(&view, &nearest)],
                &[wgpu::TextureViewDimension::D2],
                &[wgpu::TextureSampleType::Float { filterable: false }],
                "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}",
            )
            .await?;
            let mut m = ShaderMaterial::new(Arc::new(program));
            m.properties.tone_mapped = false;
            let quad = hud.insert(NodeKind::Mesh(Mesh::new(
                Arc::new(PlaneGeometry::build(256., 256., 1, 1)?),
                Arc::new(Material::Shader(m)),
            )));
            let (texture, label_width) = label_texture(name)?;
            let mut basic = MeshBasicMaterial::default();
            basic.properties.map = Some(Arc::new(texture));
            basic.properties.side = Side::Double;
            basic.properties.transparent = true;
            let label = hud.insert(NodeKind::Mesh(Mesh::new(
                Arc::new(PlaneGeometry::build(label_width.trunc(), 25., 1, 1)?),
                Arc::new(Material::Basic(basic)),
            )));
            viewers.push(Viewer { quad, label });
        }
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.set_target(Vector3::new(0., 2., 0.));
        controls.update(s, c)?;
        let mut viewers = viewers.into_iter();
        let (dir_viewer, spot_viewer) = (
            viewers.next().ok_or(Error::Invalid("viewer"))?,
            viewers.next().ok_or(Error::Invalid("viewer"))?,
        );
        let mut d = Self {
            time: 0.,
            last: 0.,
            spin: 0.,
            torus,
            cube,
            shadow_cameras: [shadow_cameras[0], shadow_cameras[1]],
            hud,
            hud_camera,
            viewers: [dir_viewer, spot_viewer],
            output: None,
            controls,
        };
        d.layout()?;
        Ok(d)
    }
    /// resizeShadowMapViewers() and updateForWindowResize(): the HUD frustum
    /// in CSS pixels, the viewers at 15% of the width, the spot light's to
    /// the right of the directional light's.
    fn layout(&mut self) -> Result<()> {
        let (w, h, _) = viewport_css();
        if let NodeKind::Camera(Camera::Orthographic(o)) =
            &mut self.hud.get_mut(self.hud_camera)?.kind
        {
            o.left = -w / 2.;
            o.right = w / 2.;
            o.top = h / 2.;
            o.bottom = -h / 2.;
        }
        let size = w * 0.15;
        for (viewer, x) in self.viewers.iter().zip([10., size + 20.]) {
            let n = self.hud.get_mut(viewer.quad)?;
            n.scale = Vector3::new(size / 256., size / 256., 1.);
            n.position = Vector3::new(-w / 2. + size / 2. + x, h / 2. - size / 2. - 10., 0.);
            let position = n.position;
            self.hud.get_mut(viewer.label)?.position =
                Vector3::new(position.x, position.y - size / 2. + 25. / 2., 0.);
        }
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// render(): the scene and HUDs draw with the spin so far; the objects
    /// then turn by the frame's delta for the next frame.
    pub fn prepare(&mut self, s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        let delta = self.time - self.last;
        self.last = self.time;
        let t = self.spin;
        let q = Euler {
            angles: Vector3::new(0.25 * t, 2. * t, t),
            order: EulerOrder::XYZ,
        }
        .quaternion();
        s.get_mut(self.torus)?.quaternion = q;
        s.get_mut(self.cube)?.quaternion = q;
        self.spin += delta;
        for (camera, helper) in self.shadow_cameras {
            update_camera_helper(s, camera, helper)?;
        }
        self.layout()
    }
    /// renderScene(), then each viewer's render(): no clear but depth.
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        if self.output.as_ref().is_none_or(|t| {
            t.width != out.width
                || t.height != out.height
                || t.options.samples != out.options.samples
        }) {
            let mut options = out.options.clone();
            options.store_multisampled_color_buffer = true;
            self.output = Some(RenderTarget::with_options(
                &r.device, out.width, out.height, options,
            )?);
        }
        let target = self
            .output
            .as_mut()
            .ok_or(Error::Invalid("viewer target"))?;
        target.set_load_color(false);
        r.render(s, c, target)?;
        target.set_load_color(true);
        r.render(&mut self.hud, self.hud_camera, target)?;
        target.set_load_color(false);
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.output.as_ref()
    }
    pub fn draw(&mut self, _kind: u32, _x: f64, _y: f64) {}
    pub fn key(&mut self, _code: u32, _down: bool) {}
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        s: &mut Scene,
        c: Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Result<()> {
        let camera: CameraState = camera_state(s, c)?;
        if wheel != 0. {
            self.controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if pan {
            self.controls.pan(&camera, dx, dy, height);
        } else {
            self.controls.rotate(dx, dy, height);
        }
        self.controls.update(s, c)
    }
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Err(Error::Invalid("shadow map viewer parameter"))
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
