//! webgl_materials_video_webcam: 128 16:9 planes on a Fibonacci sphere of
//! radius 32, each turned to the camera at its centre, sharing one
//! MeshBasicMaterial whose map is the webcam stream. The gallery page opens
//! the camera into `#video` with the page's constraints; each newly presented
//! frame is copied to the GPU texture, as VideoTexture uploads it, and the
//! planes stay black until the first frame arrives ( the texture takes that
//! frame's size; a stream that later changes size keeps it ). OrbitControls
//! rotate only.
use super::controls_attributes::Controls;
use crate::shader::ShaderProgram;
use crate::tsl::{NodeMaterial, uv};
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::f64::consts::PI;
use std::sync::Arc;
use wasm_bindgen::JsCast;

struct Stream {
    texture: wgpu::Texture,
    /// The video time of the last uploaded frame.
    uploaded: Option<f64>,
}

pub struct Demo {
    controls: Controls,
    planes: Vec<Object3D>,
    video: Option<web_sys::HtmlVideoElement>,
    stream: Option<Stream>,
    /// The map shader, compiled at creation and bound to the stream's texture
    /// once its first frame arrives.
    program: Option<ShaderProgram>,
    sampler: wgpu::Sampler,
}

impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 60.,
            near: 0.1,
            far: 100.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 0., 0.01);
        s.background = Color::BLACK;
        let mut geometry = PlaneGeometry::build(16., 9., 1, 1)?;
        geometry.scale(Vector3::splat(0.5))?;
        let geometry = Arc::new(geometry);
        // Until the stream has a frame the map is empty: the planes draw black.
        let mut black = MeshBasicMaterial::default();
        black.properties.color = Color::BLACK;
        let material = Arc::new(Material::Basic(black));
        let count = 128;
        let radius = 32.;
        let camera = s.get(c)?.position;
        let mut planes = Vec::with_capacity(count);
        for i in 1..=count {
            let phi = (-1. + (2 * i) as f64 / count as f64).acos();
            let theta = (count as f64 * PI).sqrt() * phi;
            let h = s.insert(NodeKind::Mesh(Mesh {
                geometry: geometry.clone(),
                materials: vec![material.clone()],
            }));
            s.get_mut(h)?.position = Vector3::new(
                radius * phi.sin() * theta.sin(),
                radius * phi.cos(),
                radius * phi.sin() * theta.cos(),
            );
            s.look_at(h, camera)?;
            planes.push(h);
        }
        let video = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("video"))
            .and_then(|v| v.dyn_into().ok());
        // VideoTexture: linear filtering without mipmaps, clamped, flipped on upload.
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let placeholder = video_texture(r, wgpu::Extent3d::default());
        let program = ShaderProgram::with_textures(
            r,
            &NodeMaterial::new(crate::tsl::Texture::External(0).sample(uv())).wgsl(1)?,
            &[],
            &[(&placeholder.create_view(&Default::default()), &sampler)],
        )
        .await?;
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.update(s, c)?;
        Ok(Self {
            controls,
            planes,
            video,
            stream: None,
            program: Some(program),
            sampler,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, _animate: bool) -> Result<()> {
        Ok(())
    }
    /// The first frame creates the video texture and binds the map material to
    /// it; every frame after that only copies into it.
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        self.controls.update(s, c)?;
        let Some(video) = self.video.clone() else {
            return Ok(());
        };
        if video.ready_state() < 2 || video.video_width() == 0 {
            return Ok(());
        }
        let size = wgpu::Extent3d {
            width: video.video_width(),
            height: video.video_height(),
            depth_or_array_layers: 1,
        };
        if let Some(mut program) = self.program.take() {
            let texture = video_texture(r, size);
            program.rebind(
                r,
                &[],
                &[(&texture.create_view(&Default::default()), &self.sampler)],
            )?;
            let material = Arc::new(Material::Shader(ShaderMaterial::new(Arc::new(program))));
            for &h in &self.planes {
                if let NodeKind::Mesh(m) = &mut s.get_mut(h)?.kind {
                    m.materials[0] = material.clone();
                }
            }
            self.stream = Some(Stream {
                texture,
                uploaded: None,
            });
        }
        let k = self
            .stream
            .as_mut()
            .ok_or(Error::Invalid("webcam stream"))?;
        let time = video.current_time();
        if k.uploaded != Some(time) && k.texture.size() == size {
            r.queue.copy_external_image_to_texture(
                &wgpu::CopyExternalImageSourceInfo {
                    source: wgpu::ExternalImageSource::HTMLVideoElement(video),
                    origin: wgpu::Origin2d::ZERO,
                    flip_y: true,
                },
                wgpu::CopyExternalImageDestInfo {
                    texture: &k.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                    color_space: wgpu::PredefinedColorSpace::Srgb,
                    premultiplied_alpha: false,
                },
                size,
            );
            k.uploaded = Some(time);
        }
        Ok(())
    }
    pub fn draw(&mut self, _kind: u32, _x: f64, _y: f64) {}
    pub fn key(&mut self, _code: u32, _down: bool) {}
    /// enableZoom and enablePan are off: drags only rotate.
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        _s: &mut Scene,
        _c: Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Result<()> {
        if wheel == 0. && !pan {
            self.controls.rotate(dx, dy, height);
        }
        Ok(())
    }
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {}
}

fn video_texture(r: &Renderer, size: wgpu::Extent3d) -> wgpu::Texture {
    r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("webcam texture"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::COPY_DST
            | wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    })
}
