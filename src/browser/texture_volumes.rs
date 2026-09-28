//! The compressed texture array's layer updates and the NRRD volume rendered
//! by VolumeRenderShader1, and the NRRD loader's volume slices.
mod slices;
use super::controls_attributes::{Controls, viewport_css};
use super::gltf_viewer::{decode_texture_image, fetch};
use crate::shader::ShaderProgram;
use crate::texture_gpu::GpuTexture;
use crate::tsl::{NodeMaterial, Type, WgslFn, uniform};
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::f64::consts::PI;
use std::sync::Arc;

const ASSETS: &str = "/web/gallery/assets";
/// webgl_texture2darray_layerupdate: the three-layer array fed from spirited away.
struct Layers {
    source: GpuTexture,
    array: wgpu::Texture,
    depth: u32,
    /// srcLayer, destLayer.
    params: [f64; 2],
    transfer: bool,
}
impl Layers {
    /// Copy one source layer into a destination layer on the GPU (the original
    /// uploads the same compressed layer bytes).
    fn copy(&self, r: &Renderer, from: u32, to: u32) {
        let size = self.array.size();
        let mut encoder = r
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("texture array layer update"),
            });
        encoder.copy_texture_to_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.source.texture,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: 0,
                    y: 0,
                    z: from,
                },
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyTextureInfo {
                texture: &self.array,
                mip_level: 0,
                origin: wgpu::Origin3d { x: 0, y: 0, z: to },
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::Extent3d {
                width: size.width,
                height: size.height,
                depth_or_array_layers: 1,
            },
        );
        r.queue.submit([encoder.finish()]);
    }
}
/// webgl_texture3d: the volume material and its settings.
struct Volume {
    mesh: Object3D,
    /// Per colormap (gray, viridis): the program bound to it.
    programs: [Arc<ShaderProgram>; 2],
    /// clim1, clim2, colormap, renderstyle, isothreshold.
    params: [f64; 5],
}
pub(super) struct Demo {
    id: u32,
    time: f64,
    controls: Option<Controls>,
    layers: Option<Layers>,
    volume: Option<Volume>,
    slices: Option<slices::Slices>,
}
/// A parsed NRRD: sizes, the samples as numbers, and the header's space.
pub(super) struct Nrrd {
    pub(super) sizes: [usize; 3],
    pub(super) data: Vec<f32>,
    /// `space directions`, one vector per axis.
    pub(super) vectors: Option<[[f64; 3]; 3]>,
    pub(super) space: String,
}
/// NRRDLoader: the text header, then gzip-encoded little-endian floats or shorts.
fn parse_nrrd(bytes: &[u8]) -> Result<Nrrd> {
    let end = bytes
        .windows(2)
        .position(|w| w == b"\n\n")
        .ok_or(Error::Invalid("NRRD header"))?;
    let header = std::str::from_utf8(&bytes[..end]).map_err(|_| Error::Invalid("NRRD header"))?;
    let mut sizes = [0usize; 3];
    let mut gzip = false;
    let mut short = false;
    let mut vectors = None;
    let mut space = String::new();
    for line in header.lines() {
        if let Some(v) = line.strip_prefix("sizes:") {
            for (k, n) in v.split_whitespace().enumerate().take(3) {
                sizes[k] = n.parse().map_err(|_| Error::Invalid("NRRD sizes"))?;
            }
        } else if let Some(v) = line.strip_prefix("encoding:") {
            gzip = matches!(v.trim(), "gzip" | "gz");
        } else if let Some(v) = line.strip_prefix("type:") {
            short = match v.trim() {
                "float" => false,
                "short" | "short int" | "signed short" | "signed short int" | "int16"
                | "int16_t" => true,
                _ => return Err(Error::Invalid("NRRD type")),
            };
        } else if let Some(v) = line.strip_prefix("space directions:") {
            let mut out = [[0.; 3]; 3];
            for (k, vector) in v.split_whitespace().enumerate().take(3) {
                for (m, n) in vector
                    .trim_matches(|c| c == '(' || c == ')')
                    .split(',')
                    .enumerate()
                    .take(3)
                {
                    out[k][m] = n
                        .parse()
                        .map_err(|_| Error::Invalid("NRRD space directions"))?;
                }
            }
            vectors = Some(out);
        } else if let Some(v) = line.strip_prefix("space:") {
            space = v.trim().to_string();
        }
    }
    let data = &bytes[end + 2..];
    let raw = if gzip {
        // The gzip member header: magic, method, flags, mtime, xfl, os, then
        // optional extra, name, comment and header CRC.
        if data.len() < 18 || data[0] != 0x1f || data[1] != 0x8b {
            return Err(Error::Invalid("NRRD gzip"));
        }
        let flags = data[3];
        let mut at = 10;
        if flags & 4 != 0 {
            at += 2 + u16::from_le_bytes([data[at], data[at + 1]]) as usize;
        }
        for bit in [8, 16] {
            if flags & bit != 0 {
                at += data[at..]
                    .iter()
                    .position(|&b| b == 0)
                    .ok_or(Error::Invalid("NRRD gzip"))?
                    + 1;
            }
        }
        if flags & 2 != 0 {
            at += 2;
        }
        miniz_oxide::inflate::decompress_to_vec(&data[at..])
            .map_err(|_| Error::Invalid("NRRD inflate"))?
    } else {
        data.to_vec()
    };
    let count = sizes.iter().product::<usize>();
    let width = if short { 2 } else { 4 };
    if raw.len() < count * width {
        return Err(Error::Invalid("NRRD data length"));
    }
    let data = if short {
        raw[..count * 2]
            .as_chunks::<2>()
            .0
            .iter()
            .map(|b| f32::from(i16::from_le_bytes(*b)))
            .collect()
    } else {
        raw[..count * 4]
            .as_chunks::<4>()
            .0
            .iter()
            .map(|b| f32::from_le_bytes(*b))
            .collect()
    };
    Ok(Nrrd {
        sizes,
        data,
        vectors,
        space,
    })
}
/// VolumeRenderShader1's fragment stage, with the colormap lookup moved after
/// the ray loops so that it samples in uniform control flow.
const VOLUME: &str = r#"fn volume_sample(t:vec3<f32>)->f32{return textureSampleLevel(tsl_texture_0,tsl_sampler_0,t,0.0).r;}
fn volume_render(p0:vec4<f32>,p1:vec4<f32>,p2:vec4<f32>)->vec4<f32>{
 let size=p0.xyz;let style=p0.w;let threshold=p1.x;let clim=p1.yz;let n=p1.w;
 let pos=fragment_surface.local_position;
 let view_ray=normalize(-p2.xyz);
 let t1=(vec3(-0.5)-pos)/view_ray;let t2=(size-vec3(0.5)-pos)/view_ray;
 let tmax=max(t1,t2);let dist=min(min(tmax.x,tmax.y),tmax.z);
 let nsteps=i32(dist/1.0+0.5);
 if nsteps<1 {discard;}
 let front=pos+view_ray*dist;
 let stp=((pos-front)/size)/f32(nsteps);
 let start=front/size;
 var val=0.0;var shade=1.0;var alpha_mask=1.0;
 if style<0.5 {
  var max_val=-1e6;var max_i=100;var loc=start;
  for(var iter=0;iter<887;iter++){
   if iter>=nsteps {break;}
   let v=volume_sample(loc);
   if v>max_val {max_val=v;max_i=iter;}
   loc+=stp;
  }
  var iloc=start+stp*(f32(max_i)-0.5);let istep=stp/4.0;
  for(var i=0;i<4;i++){max_val=max(max_val,volume_sample(iloc));iloc+=istep;}
  val=max_val;
 } else {
  alpha_mask=0.0;
  let dstep=1.5/size;var loc=start;
  let low=threshold-0.02*(clim.y-clim.x);
  var found=false;
  for(var iter=0;iter<887;iter++){
   if iter>=nsteps || found {break;}
   var v=volume_sample(loc);
   if v>low {
    var iloc=loc-0.5*stp;let istep=stp/4.0;
    for(var i=0;i<4;i++){
     v=volume_sample(iloc);
     if v>threshold {
      let vv=normalize(view_ray);
      var nn=vec3(0.0);
      var a=volume_sample(iloc+vec3(-dstep.x,0.0,0.0));var b=volume_sample(iloc+vec3(dstep.x,0.0,0.0));
      nn.x=a-b;v=max(max(a,b),v);
      a=volume_sample(iloc+vec3(0.0,-dstep.y,0.0));b=volume_sample(iloc+vec3(0.0,dstep.y,0.0));
      nn.y=a-b;v=max(max(a,b),v);
      a=volume_sample(iloc+vec3(0.0,0.0,-dstep.z));b=volume_sample(iloc+vec3(0.0,0.0,dstep.z));
      nn.z=a-b;v=max(max(a,b),v);
      nn=normalize(nn);
      let sel=select(0.0,1.0,dot(nn,vv)>0.0);
      nn=(2.0*sel-1.0)*nn;
      let l=normalize(view_ray);
      shade=clamp(dot(nn,l),0.0,1.0);
      val=v;alpha_mask=1.0;found=true;break;
     }
     iloc+=istep;
    }
   }
   loc+=stp;
  }
 }
 let cm=(val-clim.x)/(clim.y-clim.x);
 let c=textureSample(tsl_texture_1,tsl_sampler_1,vec2((cm*(n-1.0)+0.5)/n,0.5));
 let color=vec4(c.rgb*shade,c.a)*alpha_mask;
 if color.a<0.05 {discard;}
 return color;
}"#;
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        let mut d = Self {
            id,
            time: 0.,
            controls: None,
            layers: None,
            volume: None,
            slices: None,
        };
        s.background = Color::BLACK;
        match id {
            314 => d.layer_scene(s, c, r).await?,
            321 => {
                let nrrd = parse_nrrd(&fetch(&format!("{ASSETS}/nrrd/I.nrrd")).await?)?;
                d.slices = Some(slices::Slices::new(s, c, r, nrrd).await?);
            }
            _ => d.volume_scene(s, c, r).await?,
        }
        Ok(d)
    }
    async fn layer_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
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
        let bytes = fetch(&format!("{ASSETS}/spiritedaway.ktx2")).await?;
        let depth = basisu::Transcoder::new(&bytes)
            .map_err(|e| Error::Asset(format!("Basis: {e:?}")))?
            .layer_count();
        let source = GpuTexture::from_basis_array(r, &bytes, true)?;
        let size = source.texture.size();
        // CompressedArrayTexture( [ level 0 ], width, height, 3 ).
        let array = r.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("layer update texture array"),
            size: wgpu::Extent3d {
                width: size.width,
                height: size.height,
                depth_or_array_layers: 3,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            // A new CompressedArrayTexture has no color space: no sRGB decode.
            format: source.texture.format().remove_srgb_suffix(),
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let view = array.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        });
        let (width, height) = (20., 10.);
        // The instanced plane: translation by the instance index, flipped v.
        let projection = format!(
            "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{{var out=surface;let size=vec2({width:?},{height:?});let t=vec3(0.0,f32(surface.instance_index)*size.y-size.y,0.0);out.clip=u.projection*u.view*u.model*vec4(position+t,1.0);var uv=position.xy/size+0.5;uv.y=1.0-uv.y;out.uv=uv;return out;}}"
        );
        let color = WgslFn::new(
            "array_layer",
            "fn array_layer()->vec4<f32>{return textureSample(tsl_texture_0,tsl_sampler_0,fragment_surface.uv,i32(fragment_surface.instance_index));}",
            &[],
            Type::Vec4,
        )?
        .call(&[]);
        let program = ShaderProgram::with_projection_and_dimensions(
            r,
            &NodeMaterial::new(color).wgsl_with_texture_types(&[Type::TextureArray], &[])?,
            &[(&view, &source.sampler)],
            &[wgpu::TextureViewDimension::D2Array],
            &projection,
        )
        .await?;
        let mut g = PlaneGeometry::build(width, height, 1, 1)?;
        g.instance_count = Some(3);
        let mesh = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(g),
            Arc::new(Material::Shader(ShaderMaterial::new(Arc::new(program)))),
        )));
        s.get_mut(mesh)?.frustum_culled = false;
        let layers = Layers {
            source,
            array,
            depth,
            params: [0., 0.],
            transfer: false,
        };
        // The array starts with the first three frames.
        for layer in 0..3 {
            layers.copy(r, layer % depth, layer);
        }
        self.layers = Some(layers);
        Ok(())
    }
    async fn volume_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        let (w, h, _) = viewport_css();
        let aspect = w / h;
        let frustum = 512.;
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
            left: -frustum * aspect / 2.,
            right: frustum * aspect / 2.,
            top: frustum / 2.,
            bottom: -frustum / 2.,
            near: 1.,
            far: 1000.,
            zoom: 1.,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(-64., -64., 128.);
        n.up = Vector3::Z;
        let Nrrd { sizes, data, .. } =
            parse_nrrd(&fetch(&format!("{ASSETS}/texture-volumes/stent.nrrd")).await?)?;
        // Data3DTexture( RedFormat, FloatType ), linear filtering.
        let texture = r.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("NRRD volume"),
            size: wgpu::Extent3d {
                width: sizes[0] as u32,
                height: sizes[1] as u32,
                depth_or_array_layers: sizes[2] as u32,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D3,
            format: wgpu::TextureFormat::R32Float,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        r.queue.write_texture(
            texture.as_image_copy(),
            bytemuck::cast_slice(&data),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(sizes[0] as u32 * 4),
                rows_per_image: Some(sizes[1] as u32),
            },
            texture.size(),
        );
        let view = texture.create_view(&Default::default());
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let color = WgslFn::new(
            "volume_render",
            VOLUME,
            &[Type::Vec4, Type::Vec4, Type::Vec4],
            Type::Vec4,
        )?
        .call(&[
            uniform(0, Type::Vec4),
            uniform(1, Type::Vec4),
            uniform(2, Type::Vec4),
        ]);
        let source = NodeMaterial::new(color)
            .wgsl_with_texture_types(&[Type::Texture3D, Type::Texture], &[])?;
        let mut programs = vec![];
        for name in ["cm_gray.png", "cm_viridis.png"] {
            let mut map =
                decode_texture_image(&fetch(&format!("{ASSETS}/texture-volumes/{name}")).await?)
                    .await?;
            map.srgb = false;
            map.mipmap_filter = Some(Filter::Linear);
            let map = r.upload_texture(&Arc::new(map))?;
            programs.push(Arc::new(
                ShaderProgram::with_projection_and_dimensions(
                    r,
                    &source,
                    &[(&view, &sampler), (&map.view, &map.sampler)],
                    &[wgpu::TextureViewDimension::D3, wgpu::TextureViewDimension::D2],
                    "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}",
                )
                .await?,
            ));
        }
        let programs: [Arc<ShaderProgram>; 2] = programs
            .try_into()
            .map_err(|_| Error::Invalid("colormaps"))?;
        let mut m = ShaderMaterial::new(programs[1].clone());
        m.properties.side = Side::Back;
        let (x, y, z) = (sizes[0] as f64, sizes[1] as f64, sizes[2] as f64);
        let mut g = BoxGeometry::build(x, y, z)?;
        g.translate(Vector3::new(x / 2. - 0.5, y / 2. - 0.5, z / 2. - 0.5))?;
        let mesh = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(g),
            Arc::new(Material::Shader(m)),
        )));
        if let NodeKind::Mesh(m) = &mut s.get_mut(mesh)?.kind
            && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
        {
            m.uniforms[0] = [x as f32, y as f32, z as f32, 1.];
        }
        // OrbitControls: target ( 64, 64, 128 ), z up, zoom 0.5 to 4, no pan.
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, false);
        controls.up = Vector3::Z;
        controls.set_target(Vector3::new(64., 64., 128.));
        controls.update(s, c)?;
        self.controls = Some(controls);
        self.volume = Some(Volume {
            mesh,
            programs,
            params: [0., 1., 1., 1., 0.15],
        });
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        if let Some(k) = &mut self.slices {
            k.update(dt, animate);
        }
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        if let Some(k) = &mut self.slices {
            return k.prepare(s, r);
        }
        if let Some(k) = &mut self.layers
            && std::mem::take(&mut k.transfer)
        {
            // transfer(): the source layer ( modulo the depth ) into the destination.
            let from = k.params[0] as u32 % k.depth;
            k.copy(r, from, k.params[1] as u32);
        }
        if let Some(k) = &self.volume {
            // onWindowResize: the frustum keeps its height.
            let (w, h, _) = viewport_css();
            let aspect = w / h;
            if let NodeKind::Camera(Camera::Orthographic(o)) = &mut s.get_mut(c)?.kind {
                let height = o.top - o.bottom;
                o.left = -height * aspect / 2.;
                o.right = height * aspect / 2.;
            }
            // v_viewDirInObj: the camera's forward in the (identity) object frame.
            s.update_world_matrix(c, true, false)?;
            let forward = s.get(c)?.world_quaternion() * Vector3::NEG_Z;
            let p = k.params;
            let program = k.programs[p[2] as usize].clone();
            let style = if p[3] > 0.5 { 1. } else { 0. };
            let settings = [p[4] as f32, p[0] as f32, p[1] as f32, 256.];
            let direction = [forward.x as f32, forward.y as f32, forward.z as f32, 0.];
            if let NodeKind::Mesh(m) = &mut s.get_mut(k.mesh)?.kind
                && let Material::Shader(current) = m.materials[0].as_ref()
                && (current.uniforms[0][3] != style
                    || current.uniforms[1] != settings
                    || current.uniforms[2] != direction
                    || !Arc::ptr_eq(&current.program, &program))
                && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
            {
                m.uniforms[0][3] = style;
                m.uniforms[1] = settings;
                m.uniforms[2] = direction;
                m.program = program;
            }
        }
        Ok(())
    }
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        if let Some(k) = &mut self.slices {
            k.draw(kind, x, y);
        }
    }
    pub fn key(&mut self, code: u32, down: bool) {
        if let Some(k) = &mut self.slices {
            k.key(code, down);
        }
    }
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
        if let Some(k) = &mut self.slices {
            if wheel != 0. {
                k.wheel(wheel);
            }
            return Ok(());
        }
        let Some(controls) = &mut self.controls else {
            return Ok(());
        };
        if wheel != 0. {
            controls.wheel_scale(wheel);
        } else if pan {
            return Ok(());
        } else {
            controls.rotate(dx, dy, height);
        }
        // An orthographic camera zooms, clamped to minZoom 0.5 and maxZoom 4.
        let scale = controls.take_scale();
        if let NodeKind::Camera(Camera::Orthographic(o)) = &mut s.get_mut(c)?.kind {
            o.zoom = (o.zoom / scale).clamp(0.5, 4.);
        }
        controls.update(s, c)
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let v = value as f64;
        match (self.id, index) {
            (314, 0 | 1) => self.layers.as_mut().ok_or(Error::Invalid("layers"))?.params[index] = v,
            (314, 2) => {
                self.layers
                    .as_mut()
                    .ok_or(Error::Invalid("layers"))?
                    .transfer = true
            }
            (315, 0..=4) => self.volume.as_mut().ok_or(Error::Invalid("volume"))?.params[index] = v,
            (321, _) => self
                .slices
                .as_mut()
                .ok_or(Error::Invalid("slices"))?
                .parameter(index, v)?,
            _ => return Err(Error::Invalid("texture volume parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
        if let Some(k) = &mut self.slices {
            k.seek(t);
        }
    }
}
