//! webgl_multiple_elements_text: six views placed in the page's article, each
//! a Points scene of a lattice or a random cloud of molecules displaced by a
//! plane, cylindrical or spherical wave, drawn into its element's viewport on
//! the fixed canvas behind the text, with one OrbitControls per view. The
//! original displaces every point on the CPU each frame; here the positions
//! stay resident and the wave is evaluated in the vertex stage.
use super::controls_attributes::{CameraState, Controls, camera_state};
use crate::compute::{BufferAccess, GpuBuffer};
use crate::shader::ShaderProgram;
use crate::tsl::{NodeMaterial, Type, WgslFn};
use crate::{
    Error, Result, camera::*, geometry::*, material::*, math::*, render_target::*, renderer::*,
    scene::*,
};
use std::f64::consts::PI;
use std::sync::Arc;
use wasm_bindgen::JsCast;

const BALLS: i32 = 20;
const COLORS: [&str; 8] = [
    "rgb(0,127,255)",
    "rgb(255,0,0)",
    "rgb(0,255,0)",
    "rgb(0,255,255)",
    "rgb(255,0,255)",
    "rgb(255,0,127)",
    "rgb(255,255,0)",
    "rgb(0,255,127)",
];
/// PointsMaterial( { size: 0.25, sizeAttenuation } ) squares at the displaced
/// resident position: gl_PointSize = size × scale / −mvPosition.z, scale half
/// the canvas height (u.custom[0].y), in pixels of the viewport (custom[0].zw).
/// custom[1] is (wave, t / 5). The waves are the page's displacement functions.
const POINTS: &str = "fn wave(p:vec3<f32>,kind:f32,t:f32)->vec3<f32>{if kind==0.0 {return vec3(sin(p.x-t),0.0,0.0);}if kind==1.0 {if p.x*p.x+p.y*p.y<0.01 {return vec3(0.0);}let rho=sqrt(p.x*p.x+p.y*p.y);let phi=atan2(p.y,p.x);let s=sin(rho-t)/sqrt(rho);return vec3(1.5*cos(phi)*s,1.5*sin(phi)*s,0.0);}if dot(p,p)<0.01 {return vec3(0.0);}let r=length(p);let theta=acos(p.z/r);let phi=atan2(p.y,p.x);let s=sin(r-t)/r;return vec3(3.0*cos(phi)*sin(theta)*s,3.0*sin(phi)*sin(theta)*s,3.0*cos(theta)*s);}fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;let base=tsl_attribute_0[surface.instance_index].xyz;let p=base+wave(base,u.custom[1].x,u.custom[1].y);let mv=u.view*u.model*vec4(p,1.0);let size=max(u.custom[0].x*(u.custom[0].y/-mv.z),1.0);out.clip=u.projection*mv;out.clip=vec4(out.clip.xy+position.xy*size*2.0/u.custom[0].zw*out.clip.w,out.clip.zw);out.uv=position.xy+0.5;return out;}";
/// The map at gl_PointCoord ( x, 1 − y ); alphaTest 0.1.
const SPRITE: &str = "fn elements_sprite()->vec4<f32>{let c=textureSample(tsl_texture_0,tsl_sampler_0,fragment_surface.uv);if c.a<0.1 {discard;}return c;}";
struct View {
    element: web_sys::Element,
    scene: Scene,
    camera: Object3D,
    points: Object3D,
    wave: f32,
    controls: Controls,
    _positions: GpuBuffer,
    _texture: wgpu::Texture,
}
pub(super) struct Demo {
    views: Vec<View>,
    target: Option<RenderTarget>,
    white: Scene,
    white_camera: Object3D,
    time: f64,
    /// The view being dragged and its last pointer position and button.
    drag: Option<(usize, Vector2, u32)>,
}
/// The 128 × 128 canvas circle of one color, as a CanvasTexture (sRGB, flipY,
/// trilinear mipmaps).
fn circle(r: &Renderer, color: &str) -> Result<(wgpu::Texture, wgpu::Sampler)> {
    let fail = |_| Error::Invalid("sprite canvas");
    let canvas = web_sys::OffscreenCanvas::new(128, 128).map_err(fail)?;
    let ctx: web_sys::OffscreenCanvasRenderingContext2d = canvas
        .get_context("2d")
        .map_err(fail)?
        .ok_or(Error::Invalid("sprite context"))?
        .dyn_into()
        .map_err(|_| Error::Invalid("sprite context"))?;
    ctx.arc(64., 64., 64., 0., 2. * PI).map_err(fail)?;
    ctx.set_fill_style_str(color);
    ctx.fill();
    let data = ctx
        .get_image_data(0., 0., 128., 128.)
        .map_err(fail)?
        .data()
        .0;
    // flipY: rows from the bottom.
    let flipped: Vec<u8> = data.chunks(128 * 4).rev().flatten().copied().collect();
    let size = wgpu::Extent3d {
        width: 128,
        height: 128,
        depth_or_array_layers: 1,
    };
    let texture = r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("sprite circle"),
        size,
        mip_level_count: 8,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::COPY_DST
            | wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    r.queue.write_texture(
        texture.as_image_copy(),
        &flipped,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(128 * 4),
            rows_per_image: Some(128),
        },
        size,
    );
    crate::mipmap::MipGenerator::new(&r.device, &texture)?.update(&r.device, &r.queue);
    let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Linear,
        ..Default::default()
    });
    Ok((texture, sampler))
}
impl Demo {
    pub async fn create(_s: &mut Scene, _c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let document = web_sys::window()
            .and_then(|w| w.document())
            .ok_or(Error::Invalid("document"))?;
        let elements = document
            .query_selector_all(".view")
            .map_err(|_| Error::Invalid("views"))?;
        let mut seed = 186u32;
        let mut random = move || {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            seed as f64 / 4294967296.
        };
        let quad = Arc::new({
            let mut g = PlaneGeometry::build(1., 1., 1, 1)?;
            g.instance_count = Some(0);
            g
        });
        let mut views = vec![];
        for n in 0..elements.length() {
            let element: web_sys::Element = elements
                .item(n)
                .ok_or(Error::Invalid("view"))?
                .dyn_into()
                .map_err(|_| Error::Invalid("view"))?;
            let lattice = n % 2 == 0;
            let mut positions = vec![];
            if lattice {
                let range = BALLS / 2;
                for i in -range..=range {
                    for j in -range..=range {
                        for k in -range..=range {
                            positions.extend([i as f32, j as f32, k as f32, 0.]);
                        }
                    }
                }
            } else {
                for _ in 0..BALLS.pow(3) {
                    let b = BALLS as f64;
                    let i = b * random() - b / 2.;
                    let j = b * random() - b / 2.;
                    let k = b * random() - b / 2.;
                    positions.extend([i as f32, j as f32, k as f32, 0.]);
                }
            }
            let count = positions.len() as u32 / 4;
            let index = (COLORS.len() as f64 * random()).floor() as usize;
            let (texture, sampler) = circle(r, COLORS[index])?;
            let view = texture.create_view(&Default::default());
            let buffer = GpuBuffer::new(r, bytemuck::cast_slice(&positions), BufferAccess::Read)?;
            let node = WgslFn::new("elements_sprite", SPRITE, &[], Type::Vec4)?.call(&[]);
            let mut m = ShaderMaterial::new(Arc::new(
                ShaderProgram::with_projection(
                    r,
                    &NodeMaterial::new(node).wgsl_with_storage(1, &[Type::Vec4])?,
                    &[&buffer],
                    &[(&view, &sampler)],
                    POINTS,
                )
                .await?,
            ));
            m.properties.transparent = true;
            let mut scene = Scene::new();
            scene.background = Color::WHITE;
            let mut g = (*quad).clone();
            g.instance_count = Some(count);
            let points = scene.insert(NodeKind::Mesh(Mesh::new(
                Arc::new(g),
                Arc::new(Material::Shader(m)),
            )));
            scene.get_mut(points)?.frustum_culled = false;
            let camera = scene.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
                fov: 75.,
                aspect: 1.,
                near: 0.1,
                far: 100.,
                ..Default::default()
            })));
            scene.get_mut(camera)?.position = Vector3::new(0., 0., 1.2 * BALLS as f64);
            let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
            controls.update(&mut scene, camera)?;
            views.push(View {
                element,
                scene,
                camera,
                points,
                wave: (n / 2) as f32,
                controls,
                _positions: buffer,
                _texture: texture,
            });
        }
        let mut white = Scene::new();
        white.background = Color::WHITE;
        let white_camera = white.insert(NodeKind::Camera(Camera::Perspective(
            PerspectiveCamera::default(),
        )));
        Ok(Self {
            views,
            target: None,
            white,
            white_camera,
            time: 0.,
            drag: None,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    pub fn prepare(&mut self, _s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        Ok(())
    }
    /// animate(): the white clear, then every view not entirely offscreen in
    /// its viewport and scissor (WebGL's rounded device pixels, clipped to the
    /// canvas with a matching view offset).
    pub fn render(
        &mut self,
        r: &Renderer,
        _s: &mut Scene,
        _c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        if self
            .target
            .as_ref()
            .is_none_or(|t| t.width != out.width || t.height != out.height)
        {
            self.target = Some(RenderTarget::with_options(
                &r.device,
                out.width,
                out.height,
                out.options.clone(),
            )?);
        }
        let target = self
            .target
            .as_mut()
            .ok_or(Error::Invalid("elements target"))?;
        target.viewport = [0, 0, target.width, target.height];
        target.scissor = None;
        target.set_load_color(false);
        r.render(&mut self.white, self.white_camera, target)?;
        let dpr = web_sys::window()
            .ok_or(Error::Invalid("window"))?
            .device_pixel_ratio();
        let (tw, th) = (target.width as f64, target.height as f64);
        let (cw, ch) = (tw / dpr, th / dpr);
        // t += delta × 60: the waves take t / 5.
        let t = (self.time * 60. / 5.) as f32;
        target.set_load_color(true);
        for view in &mut self.views {
            let rect = view.element.get_bounding_client_rect();
            if rect.bottom() < 0. || rect.top() > ch || rect.right() < 0. || rect.left() > cw {
                continue;
            }
            let (w, h) = (rect.width(), rect.height());
            let bottom = ch - rect.bottom();
            let x0 = (rect.left() * dpr).round();
            let y0 = th - ((bottom * dpr).round() + (h * dpr).round());
            let (pw, ph) = ((w * dpr).round(), (h * dpr).round());
            let (ix0, iy0) = (x0.max(0.), y0.max(0.));
            let (ix1, iy1) = ((x0 + pw).min(tw), (y0 + ph).min(th));
            if ix1 <= ix0 || iy1 <= iy0 {
                continue;
            }
            if let NodeKind::Camera(Camera::Perspective(p)) =
                &mut view.scene.get_mut(view.camera)?.kind
            {
                p.aspect = 1.;
                p.view =
                    ((ix0, iy0, ix1, iy1) != (x0, y0, x0 + pw, y0 + ph)).then_some(ViewOffset {
                        full_width: pw,
                        full_height: ph,
                        offset_x: ix0 - x0,
                        offset_y: iy0 - y0,
                        width: ix1 - ix0,
                        height: iy1 - iy0,
                    });
            }
            if let NodeKind::Mesh(m) = &mut view.scene.get_mut(view.points)?.kind
                && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
            {
                m.uniforms[0] = [
                    0.25,
                    (th * 0.5) as f32,
                    (ix1 - ix0) as f32,
                    (iy1 - iy0) as f32,
                ];
                m.uniforms[1] = [view.wave, t, 0., 0.];
            }
            let area = [
                ix0 as u32,
                iy0 as u32,
                (ix1 - ix0) as u32,
                (iy1 - iy0) as u32,
            ];
            target.viewport = area;
            target.scissor = Some(area);
            r.render(&mut view.scene, view.camera, target)?;
        }
        target.viewport = [0, 0, target.width, target.height];
        target.scissor = None;
        target.set_load_color(false);
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.target.as_ref()
    }
    /// A view's OrbitControls, from the page's pointer events: kind is
    /// 1000 × ( view + 1 ) plus 10 + button (down), 0 (move), 20 (up) or 30
    /// (wheel, x = deltaY).
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        let (index, event) = ((kind / 1000) as usize, kind % 1000);
        let Some(i) = index.checked_sub(1) else {
            return;
        };
        let Some(view) = self.views.get_mut(i) else {
            return;
        };
        let height = view.element.client_height() as f64;
        let camera: Result<CameraState> = camera_state(&view.scene, view.camera);
        match event {
            10..=19 => self.drag = Some((i, Vector2::new(x, y), event - 10)),
            20..=29 => self.drag = None,
            30 => {
                if let Ok(camera) = camera {
                    view.controls.dolly(x, &camera, Vector2::ZERO);
                }
                let _ = view.controls.update(&mut view.scene, view.camera);
            }
            _ => {
                if let Some((d, last, button)) = self.drag
                    && d == i
                {
                    let (dx, dy) = (x - last.x, y - last.y);
                    self.drag = Some((d, Vector2::new(x, y), button));
                    if button == 2 {
                        if let Ok(camera) = camera {
                            view.controls.pan(&camera, dx, dy, height);
                        }
                    } else {
                        view.controls.rotate(dx, dy, height);
                    }
                    let _ = view.controls.update(&mut view.scene, view.camera);
                }
            }
        }
    }
    pub fn key(&mut self, _code: u32, _down: bool) {}
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        _s: &mut Scene,
        _c: Object3D,
        _dx: f64,
        _dy: f64,
        _wheel: f64,
        _pan: bool,
        _height: f64,
    ) -> Result<()> {
        Ok(())
    }
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Err(Error::Invalid("elements parameter"))
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
