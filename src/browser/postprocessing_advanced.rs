//! webgl_postprocessing_advanced: five EffectComposers over one scene. The
//! scene composer renders the background quad and the Phong head into an
//! 8-bit linear target, then blurs everything outside the head (the inverse
//! MaskPass); four half-size composers copy that result and chain the
//! gamma, film, vignette, dot screen, colorify (masked by the head and its
//! inverse), sepia, bloom and bleach bypass shaders into the four quadrants
//! of the canvas. Each ShaderPass is one full-screen draw into a GPU target,
//! as in the original. The stencil masks are the head rendered into a mask
//! target at the scene and half resolutions: a masked pass writes its effect
//! where the mask passes and copies the read buffer elsewhere, as
//! EffectComposer's copy pass does under the stencil test.
use super::composer_passes::{Draw, Kit};
use super::gltf_viewer::{decode_texture_image, fetch, load_asset};
use crate::{
    Error, Result, camera::*, geometry::*, material::*, math::*, render_target::*, renderer::*,
    scene::*,
};
use std::sync::Arc;

const ASSETS: &str = "/web/gallery/assets";
// The composer targets, by index.
const SCENE: usize = 0;
const FULL_A: usize = 1;
const FULL_B: usize = 2;
const HALF_A: usize = 3;
const HALF_B: usize = 4;
const BLOOM_X: usize = 5;
const BLOOM_Y: usize = 6;
const MASK_FULL: usize = 7;
const MASK_HALF: usize = 8;
/// Every target at the current size.
struct Targets {
    /// The scene composer's read buffer, with the depth the head needs.
    scene: RenderTarget,
    full: [RenderTarget; 2],
    /// One pair of half-size buffers, reused by each composer in turn.
    half: [RenderTarget; 2],
    /// BloomPass's half-float convolution targets.
    bloom: [RenderTarget; 2],
    mask_full: RenderTarget,
    mask_half: RenderTarget,
    /// (horizontal, vertical) blur steps, fixed at the page's first size.
    blur: [f32; 2],
    /// The quadrant viewports in device pixels (GL: from the bottom left).
    half_viewport: [f32; 2],
}
impl Targets {
    fn all(&self) -> [&RenderTarget; 9] {
        [
            &self.scene,
            &self.full[0],
            &self.full[1],
            &self.half[0],
            &self.half[1],
            &self.bloom[0],
            &self.bloom[1],
            &self.mask_full,
            &self.mask_half,
        ]
    }
}
pub(super) struct Demo {
    head: Object3D,
    background: Scene,
    background_camera: Object3D,
    background_quad: Object3D,
    mask: Scene,
    mask_camera: Object3D,
    mask_head: Object3D,
    time: f64,
    /// FilmPass times: the grayscale pass and the pass composers 3 and 4 share
    /// (after composer 3's and composer 4's render).
    film: [f64; 3],
    pending: bool,
    /// The page's first CSS size: its targets and blur steps until a resize.
    first: Option<(u32, u32, f64, f64)>,
    targets: Option<Targets>,
    screen: Option<RenderTarget>,
    kit: Kit,
}
async fn texture(path: &str, srgb: bool) -> Result<Arc<Texture>> {
    let mut t = decode_texture_image(&fetch(&format!("{ASSETS}/{path}")).await?).await?;
    t.srgb = srgb;
    t.mipmap_filter = Some(Filter::Linear);
    Ok(Arc::new(t))
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 50.,
            near: 1.,
            far: 10000.,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(0., 0., 900.);
        n.quaternion = Quaternion::IDENTITY;
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 3.,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position = Vector3::new(0., -0.1, 1.).normalize();
        let (asset, buffers, images) = load_asset("/web/models/LeePerrySmith.glb").await?;
        let mut temp = Scene::default();
        let meshes =
            crate::gltf::import_decoded(&asset, &buffers, &images)?.instantiate(&mut temp)?;
        let geometry = match &temp.get(meshes[0])?.kind {
            NodeKind::Mesh(m) => m.geometry.clone(),
            _ => return Err(Error::Invalid("LeePerrySmith mesh")),
        };
        let map = texture("shadow-rtt/Map-COL.jpg", true).await?;
        let mut phong = MeshPhongMaterial {
            specular: Color::from_hex(0x080808),
            shininess: 20.,
            normal_map: Some(
                texture(
                    "LeePerrySmith/Infinite-Level_02_Tangent_SmoothUV.jpg",
                    false,
                )
                .await?,
            ),
            normal_scale: Vector2::splat(0.75),
            ..Default::default()
        };
        phong.properties.color = Color::from_hex(0xcbcbcb);
        phong.properties.map = Some(map);
        let place = |s: &mut Scene, material: Material| -> Result<Object3D> {
            let head = s.insert(NodeKind::Mesh(Mesh::new(
                geometry.clone(),
                Arc::new(material),
            )));
            let n = s.get_mut(head)?;
            n.position = Vector3::new(0., -50., 0.);
            n.scale = Vector3::splat(100.);
            Ok(head)
        };
        let head = place(s, Material::Phong(phong))?;
        // The MaskPass render: the head's coverage, white on black.
        let mut mask = Scene::new();
        mask.background = Color::BLACK;
        let mask_camera = mask.insert(s.get(c)?.kind.clone());
        let mask_head = place(&mut mask, Material::Basic(MeshBasicMaterial::default()))?;
        // sceneBG: the castle face on a quad filling the orthographic view.
        let mut background = Scene::new();
        background.background = Color::BLACK;
        let background_camera =
            background.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
                near: 0.,
                far: 20000.,
                zoom: 1.,
                ..Default::default()
            })));
        // OrthographicCamera( ..., -10000, 10000 ) at z 100: the same slab with
        // the near plane at the camera (the quad neither tests nor writes depth).
        background.get_mut(background_camera)?.position = Vector3::new(0., 0., 10100.);
        let face = texture(
            "environment-materials/textures/cube/SwedishRoyalCastle/pz.jpg",
            true,
        )
        .await?;
        let mut basic = MeshBasicMaterial::default();
        basic.properties.map = Some(face);
        basic.properties.depth_test = false;
        basic.properties.depth_write = false;
        let background_quad = background.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(1., 1., 1, 1)?),
            Arc::new(Material::Basic(basic)),
        )));
        background.get_mut(background_quad)?.position = Vector3::new(0., 0., -500.);
        let mut demo = Self {
            head,
            background,
            background_camera,
            background_quad,
            mask,
            mask_camera,
            mask_head,
            time: 0.,
            film: [0.; 3],
            pending: false,
            first: None,
            targets: None,
            screen: None,
            kit: Kit::new(r),
        };
        demo.step();
        Ok(demo)
    }
    /// One render(): FilmPass adds the composer delta (0.01) per render.
    fn step(&mut self) {
        self.film[0] += 0.01;
        self.film[1] = self.film[2] + 0.01;
        self.film[2] = self.film[1] + 0.01;
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
            self.step();
        }
        Ok(())
    }
    /// render(): the head turns with Date.now() * 0.0004.
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        if std::mem::take(&mut self.pending) {
            self.step();
        }
        let q = Quaternion::from_rotation_y(-self.time * 0.4);
        s.get_mut(self.head)?.quaternion = q;
        self.mask.get_mut(self.mask_head)?.quaternion = q;
        let n = s.get(c)?.clone();
        let m = self.mask.get_mut(self.mask_camera)?;
        m.kind = n.kind;
        m.position = n.position;
        m.quaternion = n.quaternion;
        Ok(())
    }
    /// The targets at a size: the scene composer's (full) and the others'
    /// (half), in device pixels.
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let dpr = web_sys::window().map_or(1., |w| w.device_pixel_ratio());
        let css = (out.width as f64 / dpr, out.height as f64 / dpr);
        let first = *self
            .first
            .get_or_insert((out.width, out.height, css.0, css.1));
        // The page creates its targets at the CSS size; onWindowResize sets
        // them to the device size.
        let (full, half) = if (first.0, first.1) == (out.width, out.height) {
            ((css.0, css.1), (css.0 / 2., css.1 / 2.))
        } else {
            let (w, h) = (css.0 / 2., css.1 / 2.);
            ((w * 2. * dpr, h * 2. * dpr), (w * dpr, h * dpr))
        };
        let size = |v: (f64, f64)| ((v.0 as u32).max(1), (v.1 as u32).max(1));
        let (full, half) = (size(full), size(half));
        let make = |(w, h): (u32, u32), format, depth| {
            RenderTarget::with_options(
                &r.device,
                w,
                h,
                RenderTargetOptions {
                    format,
                    depth_buffer: depth,
                    ..Default::default()
                },
            )
        };
        let rgba8 = wgpu::TextureFormat::Rgba8Unorm;
        let half_float = wgpu::TextureFormat::Rgba16Float;
        self.targets = Some(Targets {
            scene: make(full, rgba8, true)?,
            full: [make(full, rgba8, false)?, make(full, rgba8, false)?],
            half: [make(half, rgba8, false)?, make(half, rgba8, false)?],
            bloom: [
                make(half, half_float, false)?,
                make(half, half_float, false)?,
            ],
            mask_full: make(full, rgba8, true)?,
            mask_half: make(half, rgba8, true)?,
            // effectHBlur.h = 2 / ( width / 2 ), from the first CSS size.
            blur: [(2. / (first.2 / 2.)) as f32, (2. / (first.3 / 2.)) as f32],
            half_viewport: [
                (css.0 / 2. * dpr).round() as f32,
                (css.1 / 2. * dpr).round() as f32,
            ],
        });
        self.kit.reset();
        // onWindowResize: the background quad and orthographic camera follow
        // the window.
        self.background.get_mut(self.background_quad)?.scale = Vector3::new(css.0, css.1, 1.);
        if let NodeKind::Camera(Camera::Orthographic(o)) =
            &mut self.background.get_mut(self.background_camera)?.kind
        {
            o.left = -css.0 / 2.;
            o.right = css.0 / 2.;
            o.top = css.1 / 2.;
            o.bottom = -css.1 / 2.;
        }
        Ok(())
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        if self
            .screen
            .as_ref()
            .is_none_or(|t| t.width != out.width || t.height != out.height)
        {
            self.screen = Some(RenderTarget::with_options(
                &r.device,
                out.width,
                out.height,
                RenderTargetOptions {
                    samples: 0,
                    depth_buffer: false,
                    ..out.options.clone()
                },
            )?);
            self.resize(r, out)?;
        }
        let screen_format = out.options.format;
        let Some(mut t) = self.targets.take() else {
            return Err(Error::Invalid("advanced targets"));
        };
        let result = self.passes(r, s, c, &mut t, screen_format);
        self.targets = Some(t);
        result?;
        Ok(true)
    }
    fn passes(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        t: &mut Targets,
        screen_format: wgpu::TextureFormat,
    ) -> Result<()> {
        // composerScene: RenderPass( sceneBG ), RenderPass( sceneModel ) over
        // it, and the MaskPass renders of the head.
        r.render(&mut self.background, self.background_camera, &t.scene)?;
        t.scene.set_load_color(true);
        let result = r.render(s, c, &t.scene);
        t.scene.set_load_color(false);
        result?;
        r.render(&mut self.mask, self.mask_camera, &t.mask_full)?;
        r.render(&mut self.mask, self.mask_camera, &t.mask_half)?;
        let size = |target: &RenderTarget| [0., 0., target.width as f32, target.height as f32];
        let (full, half) = (size(&t.full[0]), size(&t.half[0]));
        let [hw, hh] = t.half_viewport;
        let screen = self
            .screen
            .as_ref()
            .ok_or(Error::Invalid("advanced screen"))?;
        let screen_height = screen.height as f32;
        // setViewport( x, y, halfWidth, halfHeight ) from the bottom left.
        let quadrant = |x: f32, y: f32| [x, screen_height - y - hh, hw, hh];
        let none = [0.; 4];
        let copy = |output| Draw::new("fs_copy", FULL_B, Some(output), half, none);
        let gamma = |input, output| Draw::new("fs_gamma", input, Some(output), half, none);
        let vignette =
            |input, viewport| Draw::new("fs_vignette", input, None, viewport, [1.6, 0.95, 0., 0.]);
        let film = |input, output, time: f64, grayscale: f32| {
            Draw::new(
                "fs_film",
                input,
                Some(output),
                half,
                [time as f32, 0.35, grayscale, 0.],
            )
        };
        let masked = |mut d: Draw, mask, mode| {
            d.mask = mask;
            d.mode = mode;
            d
        };
        let [bw, film3, film4] = self.film;
        let (a, b) = (HALF_A, HALF_B);
        let mut dot = Draw::new("fs_dot", b, Some(a), half, [half[2], half[3], 0., 0.]);
        dot.uniforms[1] = [0.5, 0.8, 0., 0.];
        let mut combine = Draw::new("fs_combine", BLOOM_Y, Some(b), half, [0.5, 0., 0., 0.]);
        combine.load = true;
        let mut blur = masked(
            Draw::new(
                "fs_blur",
                SCENE,
                Some(FULL_A),
                full,
                [t.blur[0], 0., 0., 0.],
            ),
            MASK_FULL,
            2.,
        );
        blur.flip = true;
        let draws = [
            // composerScene: the blurs outside the head (the inverse mask).
            blur,
            masked(
                Draw::new(
                    "fs_blur",
                    FULL_A,
                    Some(FULL_B),
                    full,
                    [0., t.blur[1], 0., 0.],
                ),
                MASK_FULL,
                2.,
            ),
            // composer1: gamma, grayscale film, vignette (bottom left).
            copy(a),
            gamma(a, b),
            film(b, a, bw, 1.),
            vignette(a, quadrant(0., 0.)),
            // composer2: dot screen, colorify inside and outside the head.
            copy(a),
            gamma(a, b),
            dot,
            masked(
                Draw::new("fs_colorify", a, Some(b), half, [1., 0.8, 0.8, 0.]),
                MASK_HALF,
                1.,
            ),
            masked(
                Draw::new("fs_colorify", b, Some(a), half, [1., 0.75, 0.5, 0.]),
                MASK_HALF,
                2.,
            ),
            vignette(a, quadrant(hw, 0.)),
            // composer3: sepia, film, vignette (top left).
            copy(a),
            gamma(a, b),
            Draw::new("fs_sepia", b, Some(a), half, [0.9, 0., 0., 0.]),
            film(a, b, film3, 0.),
            vignette(b, quadrant(0., hh)),
            // composer4: bloom, film, bleach bypass, vignette (top right).
            copy(a),
            gamma(a, b),
            Draw::new(
                "fs_convolution",
                b,
                Some(BLOOM_X),
                half,
                [0.001953125, 0., 0., 0.],
            ),
            Draw::new(
                "fs_convolution",
                BLOOM_X,
                Some(BLOOM_Y),
                half,
                [0., 0.001953125, 0., 0.],
            ),
            combine,
            film(b, a, film4, 0.),
            Draw::new("fs_bleach", a, Some(b), half, [0.95, 0., 0., 0.]),
            vignette(b, quadrant(hw, hh)),
        ];
        let targets = t.all();
        let views = targets.map(|t| &t.view);
        let formats = targets.map(|t| t.options.format);
        self.kit
            .run(r, &draws, &views, &formats, (&screen.view, screen_format));
        Ok(())
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.screen.as_ref()
    }
    pub fn draw(&mut self, _kind: u32, _x: f64, _y: f64) {}
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
        Err(Error::Invalid("postprocessing advanced parameter"))
    }
    /// A still frame: one render() of the page at the example time.
    pub fn seek(&mut self, t: f64) {
        self.time = t;
        self.pending = true;
    }
}
