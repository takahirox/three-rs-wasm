//! webgl_texture2darray, webgl_texture2darray_compressed and
//! webgl_rendertarget_texture2darray: a plane showing one layer of a 2D
//! texture array, the layer stepping each frame. The render-target example
//! renders the current layer of the head volume into an array render target
//! on the GPU first, then samples that target.
use super::super::controls_attributes::set_uniform;
use super::super::gltf_viewer::fetch;
use crate::shader::ShaderProgram;
use crate::texture_gpu::GpuTexture;
use crate::tsl::{NodeMaterial, Type, WgslFn, uniform};
use crate::{
    Error, Result, camera::*, geometry::*, material::*, math::*, render_target::*, renderer::*,
    scene::*,
};
use std::sync::Arc;

const ASSETS: &str = "/web/gallery/assets";
const HEAD: [u32; 3] = [256, 256, 109];
/// The plane's vertex stage: `vUv = position.xy / size + 0.5`, flipped in v.
fn plane_projection(width: f64, height: f64) -> String {
    format!(
        "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{{var out=surface;let size=vec2({width:?},{height:?});out.clip=u.projection*u.view*u.model*vec4(position,1.0);var uv=position.xy/size+0.5;uv.y=1.0-uv.y;out.uv=uv;return out;}}"
    )
}
/// The fragment: the layer from uniform 0's x (the `int depth` uniform).
async fn layer_program(
    r: &Renderer,
    view: &wgpu::TextureView,
    sampler: &wgpu::Sampler,
    body: &str,
    projection: &str,
) -> Result<Arc<ShaderProgram>> {
    let color = WgslFn::new(
        "array_layer",
        &format!(
            "fn array_layer(p:vec4<f32>)->vec4<f32>{{let color=textureSample(tsl_texture_0,tsl_sampler_0,fragment_surface.uv,i32(p.x));{body}}}"
        ),
        &[Type::Vec4],
        Type::Vec4,
    )?
    .call(&[uniform(0, Type::Vec4)]);
    Ok(Arc::new(
        ShaderProgram::with_projection_and_dimensions(
            r,
            &NodeMaterial::new(color).wgsl_with_texture_types(&[Type::TextureArray], &[])?,
            &[(view, sampler)],
            &[wgpu::TextureViewDimension::D2Array],
            projection,
        )
        .await?,
    ))
}
fn array_view(texture: &wgpu::Texture) -> wgpu::TextureView {
    texture.create_view(&wgpu::TextureViewDescriptor {
        dimension: Some(wgpu::TextureViewDimension::D2Array),
        ..Default::default()
    })
}
/// The 256×256×109 head, unzipped once into a DataArrayTexture (RedFormat,
/// nearest filtering).
fn head_texture(r: &Renderer, data: &[u8]) -> Result<wgpu::Texture> {
    let [w, h, d] = HEAD;
    if data.len() < (w * h * d) as usize {
        return Err(Error::Invalid("head volume length"));
    }
    let texture = r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("head data array"),
        size: wgpu::Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: d,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::R8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    r.queue.write_texture(
        texture.as_image_copy(),
        &data[..(w * h * d) as usize],
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(w),
            rows_per_image: Some(h),
        },
        wgpu::Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: d,
        },
    );
    Ok(texture)
}
fn sampler(r: &Renderer, filter: wgpu::FilterMode) -> wgpu::Sampler {
    r.device.create_sampler(&wgpu::SamplerDescriptor {
        mag_filter: filter,
        min_filter: filter,
        ..Default::default()
    })
}
/// The post-processing pass: its scene, orthographic camera, quad and the
/// 256×256×109 red array render target.
struct Post {
    scene: Scene,
    camera: Object3D,
    quad: Object3D,
    target: RenderTarget,
}
pub(super) struct Arrays {
    id: u32,
    mesh: Object3D,
    /// The `depth` uniform's value and step (324 and 326).
    value: f64,
    step: f64,
    time: f64,
    last: f64,
    intensity: f64,
    post: Option<Post>,
    /// Keeps the compressed source alive for its bind group.
    _source: Option<GpuTexture>,
}
impl Arrays {
    pub(super) async fn new(s: &mut Scene, c: Object3D, r: &Renderer, id: u32) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.,
            near: 0.1,
            far: 2000.,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(0., 0., 70.);
        n.quaternion = Quaternion::IDENTITY;
        s.background = Color::BLACK;
        let (width, height) = if id == 325 { (50., 25.) } else { (50., 50.) };
        let projection = plane_projection(width, height);
        let mut source = None;
        let mut post = None;
        let program = match id {
            324 => {
                let head =
                    head_texture(r, &fetch(&format!("{ASSETS}/head256x256x109.raw")).await?)?;
                let view = array_view(&head);
                layer_program(
                    r,
                    &view,
                    &sampler(r, wgpu::FilterMode::Nearest),
                    "return vec4(color.rrr*1.5,1.0);",
                    &projection,
                )
                .await?
            }
            325 => {
                let bytes = fetch(&format!("{ASSETS}/spiritedaway.ktx2")).await?;
                let texture = GpuTexture::from_basis_array(r, &bytes, true)?;
                let view = array_view(&texture.texture);
                let program = layer_program(
                    r,
                    &view,
                    &texture.sampler,
                    "return vec4(color.rgb+0.2,1.0);",
                    &projection,
                )
                .await?;
                source = Some(texture);
                program
            }
            _ => {
                let head =
                    head_texture(r, &fetch(&format!("{ASSETS}/head256x256x109.raw")).await?)?;
                let [w, h, d] = HEAD;
                let target = RenderTarget::with_options(
                    &r.device,
                    w,
                    h,
                    RenderTargetOptions {
                        depth: d,
                        depth_buffer: false,
                        format: wgpu::TextureFormat::R8Unorm,
                        ..Default::default()
                    },
                )?;
                let target_view = array_view(&target.texture);
                // The pass: `gl_FragColor.r = voxel * uIntensity` from the
                // DataArrayTexture's layer uDepth. WebGPU attachments start at
                // the top row, so v flips to keep texel rows where WebGL
                // writes them.
                let color = WgslFn::new(
                    "voxel",
                    "fn voxel(p:vec4<f32>)->vec4<f32>{let uv=vec2(fragment_surface.uv.x,1.0-fragment_surface.uv.y);let v=textureSample(tsl_texture_0,tsl_sampler_0,uv,i32(p.x)).r;return vec4(v*p.y,0.0,0.0,1.0);}",
                    &[Type::Vec4],
                    Type::Vec4,
                )?
                .call(&[uniform(0, Type::Vec4)]);
                let pass = ShaderProgram::with_projection_and_dimensions(
                    r,
                    &NodeMaterial::new(color).wgsl_with_texture_types(&[Type::TextureArray], &[])?,
                    &[(&array_view(&head), &sampler(r, wgpu::FilterMode::Nearest))],
                    &[wgpu::TextureViewDimension::D2Array],
                    "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}",
                )
                .await?;
                let mut scene = Scene::new();
                scene.background = Color::BLACK;
                let camera =
                    scene.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
                        left: -1.,
                        right: 1.,
                        top: 1.,
                        bottom: -1.,
                        near: 0.,
                        far: 1.,
                        zoom: 1.,
                        ..Default::default()
                    })));
                let mut m = ShaderMaterial::new(Arc::new(pass));
                m.uniforms[0] = [55., 1., 0., 0.];
                let quad = scene.insert(NodeKind::Mesh(Mesh::new(
                    Arc::new(PlaneGeometry::build(2., 2., 1, 1)?),
                    Arc::new(Material::Shader(m)),
                )));
                // WebGLArrayRenderTarget's texture is a DataArrayTexture, which
                // keeps its nearest filtering.
                let program = layer_program(
                    r,
                    &target_view,
                    &sampler(r, wgpu::FilterMode::Nearest),
                    "return vec4(color.rrr*1.5,1.0);",
                    &projection,
                )
                .await?;
                post = Some(Post {
                    scene,
                    camera,
                    quad,
                    target,
                });
                program
            }
        };
        let mut m = ShaderMaterial::new(program);
        m.uniforms[0] = [55., 0., 0., 0.];
        let mesh = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(width, height, 1, 1)?),
            Arc::new(Material::Shader(m)),
        )));
        Ok(Self {
            id,
            mesh,
            value: 55.,
            step: if id == 325 { 1. } else { 0.4 },
            time: 0.,
            last: 0.,
            intensity: 1.,
            post,
            _source: source,
        })
    }
    pub(super) fn update(&mut self, dt: f64, animate: bool) {
        if animate {
            self.time += dt;
        }
    }
    pub(super) fn seek(&mut self, t: f64) {
        self.time = t;
    }
    /// animate(): the layer steps (per frame, as 60 fps steps, or from the
    /// timer), then the render-target pass writes the current layer.
    pub(super) fn prepare(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        let steps = ((self.time - self.last) * 60.).round().max(0.) as usize;
        self.last = self.time;
        let layer = if self.id == 325 {
            // depthStep += delta × 10 from 1; the uniform is its value % 5.
            ((1. + self.time * 10.) % 5.).trunc()
        } else {
            // One frame's step per 60 fps step, then the reflection test once
            // per frame, as the fixture's stepped frame does.
            let mut value = self.value + self.step * steps as f64;
            if !(0. ..=109.).contains(&value) {
                if value > 1. {
                    value = 109. * 2. - value;
                }
                if value < 0. {
                    value = -value;
                }
                self.step = -self.step;
            }
            self.value = value;
            // `int depth`: the uniform truncates.
            self.value.trunc()
        };
        set_uniform(s, self.mesh, 0, [layer as f32, 0., 0., 0.])?;
        if let Some(p) = &mut self.post {
            let target_layer = self.value.floor();
            set_uniform(
                &mut p.scene,
                p.quad,
                0,
                [target_layer as f32, self.intensity as f32, 0., 0.],
            )?;
            p.target.set_layer(target_layer as u32)?;
            r.render(&mut p.scene, p.camera, &p.target)?;
        }
        Ok(())
    }
    /// 326's intensity control.
    pub(super) fn parameter(&mut self, index: usize, value: f64) -> Result<()> {
        match (self.id, index) {
            (326, 0) => self.intensity = value,
            _ => return Err(Error::Invalid("texture array parameter")),
        }
        Ok(())
    }
}
