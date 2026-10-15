//! webgl_video_kinect: 640 × 480 points whose depth is read from the
//! Kinect recording in the vertex stage, drawn as additive 0.2-alpha
//! squares. Each new video frame is copied to the GPU once; the points
//! themselves stay resident and the camera eases toward the pointer.
use super::controls_attributes::{additive, viewport_css};
use crate::shader::ShaderProgram;
use crate::tsl::{NodeMaterial, Type, WgslFn};
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
/// The original vertex shader. Its position attribute is (j % width,
/// floor(j / width), 0) for point j, so the instance index gives it.
/// `texture2D` in a vertex stage reads level 0, which magnifies (linear).
/// gl_PointSize squares: u.custom[0] is (pointSize, 0, drawing-buffer
/// width, height); u.custom[1] is (nearClipping, farClipping, zOffset);
/// u.custom[2..6] is modelViewMatrix, composed in f64 as three does.
const KINECT_POINTS: &str = "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;let j=surface.instance_index;let p=vec2(f32(j%640u),f32(j/640u));let size=vec2(640.0,480.0);let uv=p/size;let color=textureSampleLevel(tsl_texture_0,tsl_sampler_0,uv,0.0);let depth=(color.r+color.g+color.b)/3.0;let z=(1.0-depth)*(u.custom[1].y-u.custom[1].x)+u.custom[1].x;let pos=vec4((uv.x-0.5)*z*1.11146,(uv.y-0.5)*z*0.83359,-z+u.custom[1].z,1.0);out.clip=u.projection*(mat4x4<f32>(u.custom[2],u.custom[3],u.custom[4],u.custom[5])*pos);out.clip=vec4(out.clip.xy+position.xy*u.custom[0].x*2.0/u.custom[0].zw*out.clip.w,out.clip.zw);out.uv=uv;return out;}";
/// The current video frame as an ImageBitmap without color space
/// conversion: WebGL uploads a NoColorSpace VideoTexture with
/// UNPACK_COLORSPACE_CONVERSION_WEBGL = NONE, and a direct video copy to
/// WebGPU would convert the untagged frame to sRGB.
async fn frame_bitmap(video: &web_sys::HtmlVideoElement) -> Result<web_sys::ImageBitmap> {
    use wasm_bindgen::JsCast;
    let options = web_sys::ImageBitmapOptions::new();
    options.set_color_space_conversion(web_sys::ColorSpaceConversion::None);
    let promise = web_sys::window()
        .ok_or(Error::Invalid("window"))?
        .create_image_bitmap_with_html_video_element_and_image_bitmap_options(video, &options)
        .map_err(|_| Error::Invalid("video frame bitmap"))?;
    wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .map_err(|_| Error::Invalid("video frame bitmap"))?
        .dyn_into()
        .map_err(|_| Error::Invalid("video frame bitmap"))
}
fn upload(r: &Renderer, texture: &wgpu::Texture, bitmap: web_sys::ImageBitmap) {
    r.queue.copy_external_image_to_texture(
        &wgpu::CopyExternalImageSourceInfo {
            source: wgpu::ExternalImageSource::ImageBitmap(bitmap.clone()),
            origin: wgpu::Origin2d::ZERO,
            flip_y: true,
        },
        wgpu::CopyExternalImageDestInfo {
            texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
            color_space: wgpu::PredefinedColorSpace::Srgb,
            premultiplied_alpha: false,
        },
        texture.size(),
    );
    bitmap.close();
}
pub(super) struct Demo {
    time: f64,
    last: f64,
    points: Object3D,
    video: web_sys::HtmlVideoElement,
    texture: wgpu::Texture,
    /// The video time whose frame was last requested, and the decoded frame
    /// waiting for upload.
    requested: Option<f64>,
    ready: Rc<RefCell<Option<web_sys::ImageBitmap>>>,
    /// nearClipping, farClipping, pointSize, zOffset.
    params: [f64; 4],
    /// The document mousemove position, once the pointer has moved.
    pointer: Option<Vector2>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        use wasm_bindgen::JsCast;
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
        s.get_mut(c)?.position = Vector3::new(0., 0., 500.);
        s.look_at(c, Vector3::new(0., 0., -1000.))?;
        s.background = Color::BLACK;
        let video: web_sys::HtmlVideoElement = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("video"))
            .ok_or(Error::Invalid("video element"))?
            .dyn_into()
            .map_err(|_| Error::Invalid("video element"))?;
        while video.ready_state() < 2 {
            let promise = js_sys::Promise::new(&mut |resolve, _| {
                let _ = web_sys::window()
                    .map(|w| w.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 20));
            });
            wasm_bindgen_futures::JsFuture::from(promise)
                .await
                .map_err(|_| Error::Invalid("video wait"))?;
        }
        // VideoTexture (NoColorSpace): the frame's bytes as stored, flipped
        // on upload.
        let texture = r.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("kinect video"),
            size: wgpu::Extent3d {
                width: video.video_width(),
                height: video.video_height(),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        upload(r, &texture, frame_bitmap(&video).await?);
        let requested = Some(video.current_time());
        let view = texture.create_view(&Default::default());
        // minFilter: NearestFilter, the default linear magFilter, no mipmaps.
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        // `gl_FragColor = vec4( color.rgb, 0.2 )` at the vertex's vUv.
        let color = WgslFn::new(
            "kinect_color",
            "fn kinect_color()->vec4<f32>{return vec4(textureSampleLevel(tsl_texture_0,tsl_sampler_0,fragment_surface.uv,0.0).rgb,0.2);}",
            &[],
            Type::Vec4,
        )?
        .call(&[]);
        let program = ShaderProgram::with_projection_and_dimensions(
            r,
            &NodeMaterial::new(color).wgsl_with_texture_types(&[Type::Texture], &[])?,
            &[(&view, &sampler)],
            &[wgpu::TextureViewDimension::D2],
            KINECT_POINTS,
        )
        .await?;
        let mut m = ShaderMaterial::new(Arc::new(program));
        m.properties.transparent = true;
        m.properties.blending = Some(additive());
        m.properties.depth_test = false;
        m.properties.depth_write = false;
        let mut g = PlaneGeometry::build(1., 1., 1, 1)?;
        g.instance_count = Some(WIDTH * HEIGHT);
        let points = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(g),
            Arc::new(Material::Shader(m)),
        )));
        s.get_mut(points)?.frustum_culled = false;
        Ok(Self {
            time: 0.,
            last: 0.,
            points,
            video,
            texture,
            requested,
            ready: Rc::new(RefCell::new(None)),
            params: [850., 4000., 2., 1000.],
            pointer: None,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// animate(): the camera eases toward the mouse (as 60 fps steps) and
    /// looks at the center; a new video frame is copied once.
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        // A new frame decodes asynchronously and uploads on the next frame.
        if let Some(bitmap) = self.ready.borrow_mut().take() {
            upload(r, &self.texture, bitmap);
        }
        let time = self.video.current_time();
        if self.video.ready_state() >= 2 && self.requested != Some(time) {
            self.requested = Some(time);
            let (video, ready) = (self.video.clone(), self.ready.clone());
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(bitmap) = frame_bitmap(&video).await
                    && let Some(stale) = ready.borrow_mut().replace(bitmap)
                {
                    stale.close();
                }
            });
        }
        let steps = ((self.time - self.last) * 60.).round().max(0.) as usize;
        self.last = self.time;
        let (w, h, dpr) = viewport_css();
        // mouse = ( clientX − innerWidth / 2, clientY − innerHeight / 2 ) × 8.
        let mouse = self.pointer.map_or(Vector2::ZERO, |p| {
            Vector2::new((p.x - w / 2.) * 8., (p.y - h / 2.) * 8.)
        });
        let n = s.get_mut(c)?;
        for _ in 0..steps {
            n.position.x += (mouse.x - n.position.x) * 0.05;
            n.position.y += (-mouse.y - n.position.y) * 0.05;
        }
        let position = n.position;
        let center = Vector3::new(0., 0., -1000.);
        s.look_at(c, center)?;
        // Points are frustum culled by their position attribute's bounding
        // sphere: the 640 × 480 grid at z = 0.
        let q = s.get(c)?.quaternion;
        let projection = s.camera(c)?.0.projection_matrix()?;
        let view = Matrix4::from_rotation_translation(q, position).inverse();
        let visible = Frustum::from_projection(projection * view).intersects_sphere(Sphere {
            center: Vector3::new(319.5, 239.5, 0.),
            radius: (319.5f64 * 319.5 + 239.5 * 239.5).sqrt(),
        });
        let [near, far, size, offset] = self.params.map(|v| v as f32);
        let n = s.get_mut(self.points)?;
        n.visible = visible;
        if let NodeKind::Mesh(m) = &mut n.kind
            && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
        {
            m.uniforms[0] = [size, 0., (w * dpr).round() as f32, (h * dpr).round() as f32];
            m.uniforms[1] = [near, far, offset, 0.];
            for (i, column) in view.to_cols_array_2d().iter().enumerate() {
                m.uniforms[2 + i] = column.map(|v| v as f32);
            }
        }
        Ok(())
    }
    /// Absolute CSS-pixel pointer moves (the document's mousemove).
    pub fn draw(&mut self, _kind: u32, x: f64, y: f64) {
        self.pointer = Some(Vector2::new(x, y));
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
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let p = self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("kinect parameter"))?;
        *p = value as f64;
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
