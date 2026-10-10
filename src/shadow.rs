//! Depth atlas shared by directional, spot and point light shadow passes.
use crate::{Error, Result, camera::*, material::*, math::*, scene::*};
use std::sync::Arc;

#[derive(Clone, Copy, Debug, Default, serde::Serialize)]
pub enum ShadowFilter {
    #[default]
    Pcf,
    Basic,
    /// VSMShadowMap: the depth blurred into mean and deviation, read by
    /// Chebyshev's bound. Directional and spot lights only.
    Vsm,
}

#[derive(Clone, Copy, Debug, serde::Serialize)]
pub struct Shadow {
    /// Per-light map width; None inherits Scene::shadow_map_size.
    pub map_size: Option<u32>,
    /// A directional or spot light's map height when it differs from its
    /// width ( LightShadow.mapSize ); the map takes that part of its layer.
    pub map_height: Option<u32>,
    /// WebGLRenderer( { reversedDepthBuffer: true } ): the map is compared
    /// greater-equal against a depth cleared to 0, so receivers beyond the
    /// shadow camera's far plane ( inside its x / y frustum ) read shadowed.
    pub reversed_depth: bool,
    pub near: f64,
    pub far: f64,
    /// Half width/height of a directional light's orthographic shadow camera.
    pub extent: f64,
    /// Offset added to the comparison depth, in normalized depth units.
    pub bias: f64,
    /// Offset the receiving surface along its world-space normal.
    pub normal_bias: f64,
    /// PCF radius in texels; zero selects a single filtered comparison.
    pub radius: f64,
    pub filter: ShadowFilter,
    /// SpotLightShadow.focus: the shadow camera's fov is 2 × angle × focus.
    pub focus: f64,
    /// LightShadow.intensity: received shadows are mixed toward lit by 1 − intensity.
    pub intensity: f64,
    /// LightShadow.blurSamples: the VSM blur's taps per pass.
    pub blur_samples: u32,
}
impl Default for Shadow {
    fn default() -> Self {
        Self {
            map_size: None,
            map_height: None,
            reversed_depth: false,
            near: 0.5,
            far: 500.0,
            extent: 5.0,
            bias: 0.0,
            normal_bias: 0.0,
            radius: 1.0,
            filter: ShadowFilter::Pcf,
            focus: 1.0,
            intensity: 1.0,
            blur_samples: 8,
        }
    }
}
/// SkinnedMesh.computeBoundingSphere(): each skinned vertex added with
/// Sphere.expandByPoint, in the mesh's local space.
fn skinned_sphere(scene: &Scene, handle: Object3D) -> Result<Sphere> {
    let mut center = Vector3::ZERO;
    let mut radius = -1.0f64;
    if let Some(geometry) = crate::deformation::evaluate(scene, handle)? {
        let positions = geometry
            .attributes
            .get("position")
            .ok_or(Error::Invalid("skinned positions"))?;
        for i in 0..positions.count() {
            let p = positions.vector3(i)?;
            if radius < 0.0 {
                center = p;
                radius = 0.0;
                continue;
            }
            let v = p - center;
            let length_sq = v.length_squared();
            if length_sq > radius * radius {
                let length = length_sq.sqrt();
                let delta = (length - radius) * 0.5;
                center += v * (delta / length);
                radius += delta;
            }
        }
    }
    Ok(Sphere { center, radius })
}
pub(crate) struct Atlas {
    pub view: wgpu::TextureView,
    /// The VSM layers' blurred ( mean, deviation ), by shadow layer, when a light
    /// uses VSM. It takes the LTC tables' binding, so a scene with VSM shadows has
    /// no rect area lights.
    pub vsm_view: Option<wgpu::TextureView>,
    pub matrices: [[f32; 16]; 48],
    pub params: [[f32; 4]; 8],
    pub filters: [[f32; 4]; 8],
    pub cascades: [[f32; 4]; 16],
    /// Per light, its map's extent in its layers ( x, y ).
    pub scales: [[f32; 4]; 8],
}
pub(crate) struct ShadowRenderer {
    layout: wgpu::BindGroupLayout,
    pipelines: [wgpu::RenderPipeline; 10],
    pub sampler: wgpu::Sampler,
    empty: wgpu::TextureView,
    white: Arc<Texture>,
    target: std::cell::RefCell<Option<ShadowTarget>>,
    /// Resident draw data per (shadow layer, caster, material group), so reordered
    /// casters keep their bindings.
    slots: std::cell::RefCell<
        std::collections::HashMap<(usize, crate::scene::Object3D, usize), crate::draw_gpu::Slot>,
    >,
    /// The cached target holds maps rendered for the current layout.
    rendered: std::cell::Cell<bool>,
    /// SkinnedMesh.boundingSphere: computed once from the pose at first use
    /// and kept, as three's frustum culling caches it.
    skinned_spheres: std::cell::RefCell<std::collections::HashMap<(usize, Object3D), Sphere>>,
    vsm: std::cell::RefCell<Option<VsmTarget>>,
    pub vsm_sampler: wgpu::Sampler,
    vsm_layout: wgpu::BindGroupLayout,
    vsm_pipelines: [wgpu::RenderPipeline; 2],
}
/// The VSM blur targets: the ( mean, deviation ) layers and the vertical pass.
struct VsmTarget {
    view: wgpu::TextureView,
    layers: Vec<wgpu::TextureView>,
    /// Per shadow layer: the vertical and horizontal passes' uniforms and bindings.
    passes: Vec<[(wgpu::Buffer, wgpu::BindGroup); 2]>,
    pass_view: wgpu::TextureView,
    size: u32,
}
const VSM_WGSL: &str = "
struct V{rect:vec4<f32>,params:vec4<f32>}
@group(0)@binding(0) var<uniform> v:V;
@group(0)@binding(1) var depth_map:texture_depth_2d;
@group(0)@binding(2) var pass_map:texture_2d<f32>;
@group(0)@binding(3) var linear_sampler:sampler;
@vertex fn vs(@builtin(vertex_index) i:u32)->@builtin(position) vec4<f32>{let p=array(vec2(-1.0,-1.0),vec2(3.0,-1.0),vec2(-1.0,3.0));return vec4(p[i],0.0,1.0);}
// vsm.glsl: VSM_SAMPLES taps radius apart across -1..1; the vertical pass reads
// the depth ( nearest ), the horizontal the vertical result ( linear ).
@fragment fn vertical(@builtin(position) p:vec4<f32>)->@location(0) vec4<f32>{
 let samples=v.params.y;var mean=0.0;var squared=0.0;
 let stride=select(2.0/(samples-1.0),0.0,samples<=1.0);let start=select(-1.0,0.0,samples<=1.0);
 for(var i=0.0;i<samples;i+=1.0){
  let q=p.xy+vec2(0.0,start+i*stride)*v.params.x;
  let t=clamp(vec2<i32>(floor(q)),vec2<i32>(v.rect.xy),vec2<i32>(v.rect.xy+v.rect.zw)-1);
  let d=textureLoad(depth_map,t,0);mean+=d;squared+=d*d;
 }
 mean/=samples;squared/=samples;
 return vec4(mean,sqrt(max(0.0,squared-mean*mean)),0.0,1.0);
}
@fragment fn horizontal(@builtin(position) p:vec4<f32>)->@location(0) vec4<f32>{
 let samples=v.params.y;var mean=0.0;var squared=0.0;
 let stride=select(2.0/(samples-1.0),0.0,samples<=1.0);let start=select(-1.0,0.0,samples<=1.0);
 let size=vec2<f32>(textureDimensions(pass_map));
 for(var i=0.0;i<samples;i+=1.0){
  let q=p.xy+vec2(start+i*stride,0.0)*v.params.x;
  let d=textureSampleLevel(pass_map,linear_sampler,q/size,0.0).rg;
  mean+=d.x;squared+=d.y*d.y+d.x*d.x;
 }
 mean/=samples;squared/=samples;
 return vec4(mean,sqrt(max(0.0,squared-mean*mean)),0.0,1.0);
}";
/// The VSM layers take the LTC tables' binding: a scene with VSM shadows can have
/// no rect area lights.
fn rect_area_free(scene: &Scene) -> Result<()> {
    for root in scene.roots() {
        for h in scene.traverse(root, true)? {
            if matches!(scene.get(h)?.kind, NodeKind::Light(Light::RectArea { .. })) {
                return Err(Error::Invalid("VSM shadows with rect area lights"));
            }
        }
    }
    Ok(())
}
fn vsm_texture(device: &wgpu::Device, size: u32, layers: u32) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("vsm shadow map"),
        size: wgpu::Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: layers,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rg16Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    })
}
struct ShadowTarget {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    layers: Vec<wgpu::TextureView>,
}
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniform {
    matrix: [f32; 16],
    alpha: [f32; 4],
    model: [f32; 16],
    clipping: crate::clipping::Clipping,
    uv_transform: [[f32; 4]; 3],
    custom: [[f32; 4]; 16],
}
impl ShadowRenderer {
    /// A SkinnedMesh's bounding sphere: computed from the pose at first use and
    /// cached until `compute_skinned_bounds` recomputes it, as three caches
    /// `boundingSphere` until `computeBoundingSphere()`.
    pub(crate) fn skinned_bounds(&self, scene: &Scene, handle: Object3D) -> Result<Sphere> {
        let key = (Arc::as_ptr(&scene.cache_owner) as usize, handle);
        if let Some(sphere) = self.skinned_spheres.borrow().get(&key) {
            return Ok(*sphere);
        }
        self.compute_skinned_bounds(scene, handle)
    }
    pub(crate) fn compute_skinned_bounds(&self, scene: &Scene, handle: Object3D) -> Result<Sphere> {
        let key = (Arc::as_ptr(&scene.cache_owner) as usize, handle);
        let sphere = skinned_sphere(scene, handle)?;
        self.skinned_spheres.borrow_mut().insert(key, sphere);
        Ok(sphere)
    }
    pub fn new(device: &wgpu::Device) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shadow depth"),
            source: wgpu::ShaderSource::Wgsl(
                concat!(
                    include_str!("shaders/deformation.wgsl"),
                    "\n",
                    include_str!("shaders/shadow.wgsl")
                )
                .into(),
            ),
        });
        let mut entries = vec![
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ];
        entries.extend(crate::deformation_gpu::layout_entries());
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("shadow layout"),
            entries: &entries,
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&layout],
            push_constant_ranges: &[],
        });
        let pipelines = depth_pipelines(device, &pipeline_layout, &shader);
        let empty = depth_texture(device, 1, 1).create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        });
        let texture_entry = |binding, sample_type| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                sample_type,
                view_dimension: wgpu::TextureViewDimension::D2,
                multisampled: false,
            },
            count: None,
        };
        let vsm_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("vsm blur"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                texture_entry(1, wgpu::TextureSampleType::Depth),
                texture_entry(2, wgpu::TextureSampleType::Float { filterable: true }),
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let vsm_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("vsm blur"),
            source: wgpu::ShaderSource::Wgsl(VSM_WGSL.into()),
        });
        let vsm_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("vsm blur"),
            bind_group_layouts: &[&vsm_layout],
            push_constant_ranges: &[],
        });
        let vsm_pipelines = ["vertical", "horizontal"].map(|entry| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("vsm blur"),
                layout: Some(&vsm_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &vsm_module,
                    entry_point: Some("vs"),
                    compilation_options: Default::default(),
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &vsm_module,
                    entry_point: Some(entry),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::TextureFormat::Rg16Float.into())],
                }),
                primitive: Default::default(),
                depth_stencil: None,
                multisample: Default::default(),
                multiview: None,
                cache: None,
            })
        });
        let linear = wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        };
        Self {
            vsm: Default::default(),
            vsm_sampler: device.create_sampler(&linear),
            vsm_layout,
            vsm_pipelines,
            layout,
            pipelines,
            empty,
            target: Default::default(),
            slots: Default::default(),
            rendered: Default::default(),
            skinned_spheres: Default::default(),
            sampler: device.create_sampler(&wgpu::SamplerDescriptor {
                compare: Some(wgpu::CompareFunction::LessEqual),
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            }),
            white: Arc::new(Texture::from_rgba(1, 1, vec![255; 4], false).unwrap()),
        }
    }
    /// The depth atlas of the last shadow pass: one layer per shadow camera,
    /// in shadow-casting light order.
    pub fn atlas_texture(&self) -> Option<wgpu::Texture> {
        self.target.borrow().as_ref().map(|t| t.texture.clone())
    }
    pub fn collect_resources(&self) {
        self.slots.borrow_mut().clear();
        *self.target.borrow_mut() = None;
    }
    #[allow(clippy::too_many_arguments)]
    pub fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        cache: &mut crate::texture_gpu::TextureCache,
        scene: &Scene,
        visible: &[Object3D],
        lights: &[Object3D],
        view_camera: &Camera,
        camera_world: Matrix4,
        geometry_cache: &mut crate::geometry_gpu::Cache,
        deformation_cache: &mut crate::deformation_gpu::Cache,
    ) -> Result<Atlas> {
        let mut slots = self.slots.borrow_mut();
        let mut used = std::collections::HashSet::new();
        let mut atlas = Atlas {
            view: self.empty.clone(),
            vsm_view: None,
            matrices: [[0.0; 16]; 48],
            params: [[0.0; 4]; 8],
            filters: [[0.0; 4]; 8],
            cascades: [[0.; 4]; 16],
            scales: [[0.0; 4]; 8],
        };
        let mut cameras = Vec::new();
        let mut viewports = Vec::new();
        // Per shadow layer: VSMShadowMap's blur ( radius, samples ).
        let mut vsm_layers: Vec<Option<(f64, u32)>> = Vec::new();
        let mut size = 0;
        for &handle in lights {
            let node = scene.get(handle)?;
            if node.cast_shadow {
                let resolution = node.shadow.map_size.unwrap_or(scene.shadow_map_size);
                let height = node.shadow.map_height.unwrap_or(resolution);
                if resolution == 0
                    || height == 0
                    || resolution.max(height) > device.limits().max_texture_dimension_2d
                {
                    return Err(Error::Invalid("shadow map size"));
                }
                size = size.max(resolution).max(height);
            }
        }

        for (i, &handle) in lights.iter().enumerate() {
            let node = scene.get(handle)?;
            if !node.cast_shadow {
                continue;
            }
            let shadow = node.shadow;
            let resolution = shadow.map_size.unwrap_or(scene.shadow_map_size);
            let scale = resolution as f64 / size as f64;
            let scale_y = shadow.map_height.unwrap_or(resolution) as f64 / size as f64;
            atlas.scales[i] = [scale as f32, scale as f32, 0., 0.];
            if ![
                shadow.near,
                shadow.far,
                shadow.extent,
                shadow.bias,
                shadow.normal_bias,
                shadow.radius,
            ]
            .iter()
            .all(|v| v.is_finite())
                || shadow.near <= 0.0
                || shadow.far <= shadow.near
                || shadow.extent <= 0.0
                || shadow.radius < 0.0
            {
                return Err(Error::Invalid("shadow camera or filter"));
            }
            let position = node.world_position();
            if matches!(&node.kind, NodeKind::Light(Light::Sun { .. })) {
                let cascades = crate::sun_shadow::cascades(
                    view_camera,
                    camera_world,
                    position,
                    shadow,
                    resolution,
                )?;
                atlas.params[i] = [
                    (cameras.len() + 1) as f32,
                    shadow.bias as f32,
                    shadow.normal_bias as f32,
                    2.,
                ];
                atlas.filters[i] = [
                    shadow.radius as f32,
                    if matches!(shadow.filter, ShadowFilter::Basic) {
                        1.
                    } else {
                        0.
                    },
                    scale as f32,
                    (1.0 - shadow.intensity) as f32,
                ];
                for (j, c) in cascades.into_iter().enumerate() {
                    atlas.matrices[cameras.len()] = c.projection_view.as_mat4().to_cols_array();
                    atlas.cascades[i * 2 + j] = c.range.extend(c.viewport.x).as_vec4().to_array();
                    viewports.push(c.viewport * scale);
                    cameras.push(c.projection_view);
                    vsm_layers.push(None);
                }
                continue;
            }
            let mut views = Vec::new();
            let projection = match &node.kind {
                NodeKind::Light(Light::Directional { target, .. }) => {
                    views.push(view(position, *target, node.up));
                    OrthographicCamera {
                        left: -shadow.extent,
                        right: shadow.extent,
                        top: shadow.extent,
                        bottom: -shadow.extent,
                        near: shadow.near,
                        far: shadow.far,
                        ..Default::default()
                    }
                    .projection_matrix()?
                }
                NodeKind::Light(Light::Spot { target, angle, .. }) => {
                    views.push(view(position, *target, node.up));
                    PerspectiveCamera {
                        fov: (2.0 * angle * shadow.focus).to_degrees().min(179.9),
                        aspect: 1.0,
                        near: shadow.near,
                        far: shadow.far,
                        ..Default::default()
                    }
                    .projection_matrix()?
                }
                NodeKind::Light(Light::Point { .. }) => {
                    for (axis, up) in [
                        (Vector3::X, Vector3::Y),
                        (-Vector3::X, Vector3::Y),
                        (Vector3::Y, Vector3::Z),
                        (-Vector3::Y, -Vector3::Z),
                        (Vector3::Z, Vector3::Y),
                        (-Vector3::Z, Vector3::Y),
                    ] {
                        views.push(view(position, position + axis, up));
                    }
                    PerspectiveCamera {
                        fov: 90.0,
                        aspect: 1.0,
                        near: shadow.near,
                        far: shadow.far,
                        ..Default::default()
                    }
                    .projection_matrix()?
                }
                _ => continue,
            };
            atlas.params[i] = [
                (cameras.len() + 1) as f32,
                shadow.bias as f32,
                shadow.normal_bias as f32,
                if views.len() == 6 { 1.0 } else { 0.0 },
            ];
            if views.len() == 1 {
                atlas.scales[i][1] = scale_y as f32;
            }
            atlas.scales[i][2] = f32::from(u8::from(shadow.reversed_depth));
            let vsm = matches!(shadow.filter, ShadowFilter::Vsm) && views.len() == 1;
            atlas.filters[i] = [
                shadow.radius as f32,
                match shadow.filter {
                    ShadowFilter::Basic => 1.,
                    ShadowFilter::Vsm if vsm => 2.,
                    _ => 0.,
                },
                scale as f32,
                (1.0 - shadow.intensity) as f32,
            ];
            let cube = views.len() == 6;
            for v in views {
                let matrix = projection * v;
                atlas.matrices[cameras.len()] = matrix.as_mat4().to_cols_array();
                cameras.push(matrix);
                // A point light's cube faces stay square.
                viewports.push(if !cube {
                    Vector4::new(0., 0., scale, scale_y)
                } else {
                    Vector4::new(0., 0., scale, scale)
                });
                vsm_layers.push(vsm.then_some((shadow.radius, shadow.blur_samples)));
            }
        }
        // A pass without shadow casters (e.g. an overlay scene) keeps the
        // resident atlas and caster slots for the next shadowed pass.
        if cameras.is_empty() {
            return Ok(atlas);
        }
        if size == 0 || size > device.limits().max_texture_dimension_2d {
            return Err(Error::Invalid("shadow map size"));
        }
        let mut cached = self.target.borrow_mut();
        // The atlas only grows: a pass with fewer shadow cameras ( a bake's
        // directional light between cascaded frames ) reuses its first layers.
        if cached.as_ref().is_none_or(|t| {
            t.texture.width() != size || t.texture.depth_or_array_layers() < cameras.len() as u32
        }) {
            let texture = depth_texture(device, size, cameras.len() as u32);
            let view = texture.create_view(&wgpu::TextureViewDescriptor {
                dimension: Some(wgpu::TextureViewDimension::D2Array),
                ..Default::default()
            });
            let layers = (0..cameras.len())
                .map(|layer| {
                    texture.create_view(&wgpu::TextureViewDescriptor {
                        dimension: Some(wgpu::TextureViewDimension::D2),
                        base_array_layer: layer as u32,
                        array_layer_count: Some(1),
                        ..Default::default()
                    })
                })
                .collect();
            *cached = Some(ShadowTarget {
                texture,
                view,
                layers,
            });
            self.rendered.set(false);
        }
        let target = cached.as_ref().expect("shadow target");
        let atlas_layers = target.texture.depth_or_array_layers() as usize;
        atlas.view = target.view.clone();
        if let Some(v) = self.vsm.borrow().as_ref()
            && vsm_layers.iter().any(Option::is_some)
        {
            rect_area_free(scene)?;
            atlas.vsm_view = Some(v.view.clone());
        }
        if !scene.shadow_auto_update && self.rendered.get() {
            return Ok(atlas);
        }
        let mut encoder = device.create_command_encoder(&Default::default());
        for (layer, camera) in cameras.iter().enumerate() {
            let mut draws = Vec::new();
            // WebGLShadowMap culls casters against each shadow camera's frustum.
            let frustum = crate::math::Frustum::from_projection(*camera);
            // VSMShadowMap also draws the receivers.
            let vsm = vsm_layers[layer].is_some();
            for &handle in visible {
                let node = scene.get(handle)?;
                if !(node.cast_shadow || (vsm && node.receive_shadow)) {
                    continue;
                }
                // WebGLShadowMap draws Line and LineSegments casters with their own
                // primitives.
                let (geometry, materials, line) = match &node.kind {
                    NodeKind::Mesh(mesh) => (&mesh.geometry, mesh.materials.clone(), None),
                    NodeKind::Line(l) => (&l.geometry, vec![l.material.clone()], Some(l.segments)),
                    _ => continue,
                };
                if node.frustum_culled && node.skin.is_some() && node.instances.is_empty() {
                    let sphere = self.skinned_bounds(scene, handle)?;
                    if !frustum.intersects_sphere(sphere.transformed(node.matrix_world)) {
                        continue;
                    }
                }
                if node.frustum_culled
                    && node.skin.is_none()
                    && node.instances.is_empty()
                    && !frustum.intersects_sphere(
                        geometry_cache
                            .sphere(geometry)?
                            .transformed(node.matrix_world),
                    )
                {
                    continue;
                }
                if materials.iter().any(|m| {
                    (matches!(m.as_ref(), crate::material::Material::Shader(_))
                        || m.properties().vertex_program.is_some())
                        && m.properties().shadow_program.is_none()
                }) {
                    return Err(Error::Invalid(
                        "custom WGSL shadow caster requires a shadow program",
                    ));
                }
                let g = geometry;
                let gpu = geometry_cache.get(device, queue, g, false, false, false, None)?;
                let wire = if line.is_none()
                    && materials.iter().any(|m| m.properties().wireframe)
                    && g.indirect.is_none()
                    && g.gpu_indirect.is_none()
                {
                    Some(geometry_cache.get(device, queue, g, false, false, true, None)?)
                } else {
                    None
                };
                let deformation = deformation_cache.get(device, queue, scene, handle)?;
                let groups = if materials.len() > 1 {
                    g.groups.clone()
                } else {
                    vec![crate::geometry::Group {
                        start: 0,
                        count: g.draw_count(),
                        material_index: 0,
                    }]
                };
                for (group_index, group) in groups.into_iter().enumerate() {
                    let material = materials
                        .get(group.material_index)
                        .ok_or(Error::Invalid("shadow material group"))?
                        .properties();
                    if !material.visible {
                        continue;
                    }
                    let start = group.start.max(g.draw_range.start);
                    let end = group
                        .start
                        .saturating_add(group.count)
                        .min(g.draw_count())
                        .min(
                            g.draw_range
                                .start
                                .saturating_add(g.draw_range.count.unwrap_or(usize::MAX)),
                        );
                    if end <= start {
                        continue;
                    }
                    let map = material.map.as_ref().unwrap_or(&self.white);
                    let image = cache.get(device, queue, map)?;
                    let defaults = [Instance::default()];
                    let instances = if node.instances.is_empty() {
                        &defaults[..]
                    } else {
                        &node.instances
                    };
                    let data = instances
                        .iter()
                        .map(|i| {
                            if !i.matrix.is_finite() || i.matrix.determinant() < 0.0 {
                                return Err(Error::Invalid("instance transform"));
                            }
                            Ok(crate::renderer::InstanceVertex {
                                matrix: i.matrix.as_mat4().to_cols_array(),
                                color: i.color.0.extend(1.0).as_vec4().to_array(),
                            })
                        })
                        .collect::<Result<Vec<_>>>()?;
                    let key = (layer, handle, group_index);
                    used.insert(key);
                    let slot = slots.entry(key).or_default();
                    let instance_buffer =
                        slot.instances(device, queue, bytemuck::cast_slice(&data));
                    let uniform = slot.uniform(
                        device,
                        queue,
                        bytemuck::bytes_of(&Uniform {
                            matrix: (*camera * node.matrix_world).as_mat4().to_cols_array(),
                            model: node.matrix_world.as_mat4().to_cols_array(),
                            clipping: crate::clipping::prepare(scene, material, true)?,
                            uv_transform: std::array::from_fn(|c| {
                                let v = map.uv_matrix().col(c).as_vec3();
                                [
                                    v.x,
                                    v.y,
                                    v.z,
                                    if c == 2 { map.tex_coord as f32 } else { 0.0 },
                                ]
                            }),
                            custom: material.vertex_uniforms,
                            alpha: [
                                material.opacity as f32,
                                material.alpha_test as f32,
                                0.0,
                                0.0,
                            ],
                        }),
                    );
                    let bind = slot.bindings(
                        device,
                        &self.layout,
                        &[
                            wgpu::BindGroupEntry {
                                binding: 21,
                                resource: deformation.data.as_entire_binding(),
                            },
                            wgpu::BindGroupEntry {
                                binding: 22,
                                resource: deformation.pose.as_entire_binding(),
                            },
                            wgpu::BindGroupEntry {
                                binding: 23,
                                resource: deformation.info.as_entire_binding(),
                            },
                            wgpu::BindGroupEntry {
                                binding: 0,
                                resource: uniform.as_entire_binding(),
                            },
                            wgpu::BindGroupEntry {
                                binding: 1,
                                resource: wgpu::BindingResource::TextureView(&image.view),
                            },
                            wgpu::BindGroupEntry {
                                binding: 2,
                                resource: wgpu::BindingResource::Sampler(&image.sampler),
                            },
                        ],
                    );
                    let mirrored = node.matrix_world.determinant() < 0.0;
                    let shadow_side = material.shadow_side.unwrap_or(match material.side {
                        side if vsm => side,
                        Side::Front => Side::Back,
                        Side::Back => Side::Front,
                        Side::Double => Side::Double,
                    });
                    let side = match shadow_side {
                        Side::Double => 2,
                        Side::Front => usize::from(mirrored),
                        Side::Back => usize::from(!mirrored),
                    };
                    let lines = material.wireframe
                        && material.shadow_program.is_none()
                        && start % 3 == 0
                        && end % 3 == 0;
                    if let Some(segments) = line {
                        let instanced = !node.instances.is_empty();
                        draws.push((
                            gpu.clone(),
                            instance_buffer,
                            bind,
                            start as u32..end as u32,
                            node.draw_instance_count(g)?,
                            match (segments, instanced) {
                                (true, true) => 6,
                                (true, false) => 7,
                                (false, true) => 8,
                                (false, false) => 9,
                            },
                            None,
                        ));
                        continue;
                    }
                    match (&wire, lines) {
                        (Some(wire), true) => draws.push((
                            wire.clone(),
                            instance_buffer,
                            bind,
                            start as u32 * 2..end as u32 * 2,
                            node.draw_instance_count(g)?,
                            if node.instances.is_empty() { 7 } else { 6 },
                            None,
                        )),
                        _ => draws.push((
                            gpu.clone(),
                            instance_buffer,
                            bind,
                            start as u32..end as u32,
                            node.draw_instance_count(g)?,
                            side + if node.instances.is_empty() { 3 } else { 0 },
                            material.shadow_program.clone(),
                        )),
                    }
                }
            }
            let attachment = &target.layers[layer];
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("shadow depth"),
                color_attachments: &[],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: attachment,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            let v = viewports[layer] * size as f64;
            pass.set_viewport(v.x as f32, v.y as f32, v.z as f32, v.w as f32, 0., 1.);
            for (gpu, instances, bindings, range, count, side, program) in &draws {
                pass.set_pipeline(
                    program
                        .as_ref()
                        .map_or(&self.pipelines[*side], |p| &p.pipelines[*side]),
                );
                pass.set_bind_group(0, bindings, &[]);
                if let Some(program) = program {
                    pass.set_bind_group(1, &program.bindings, &[]);
                }
                pass.set_vertex_buffer(0, gpu.vertices.slice(..));
                pass.set_vertex_buffer(1, instances.slice(..));
                if let Some(indices) = &gpu.indices {
                    pass.set_index_buffer(indices.slice(..), wgpu::IndexFormat::Uint32);
                    pass.draw_indexed(range.clone(), 0, 0..*count);
                } else {
                    pass.draw(range.clone(), 0..*count);
                }
            }
        }
        if vsm_layers.iter().any(Option::is_some) {
            rect_area_free(scene)?;
            atlas.vsm_view = Some(self.blur_vsm(
                device,
                queue,
                &mut encoder,
                target,
                &vsm_layers,
                &viewports,
                size,
            )?);
        }
        // Keep slots of casters culled this frame (they return as the light moves);
        // drop only removed nodes and layers beyond the current shadow cameras.
        slots.retain(|key, _| {
            used.contains(key) || (key.0 < atlas_layers && scene.get(key.1).is_ok())
        });
        queue.submit([encoder.finish()]);
        self.rendered.set(true);
        Ok(atlas)
    }
}
impl ShadowRenderer {
    /// VSMPass: per VSM layer, the vertical blur of the depth into the pass
    /// target, then the horizontal blur into the layer's ( mean, deviation ).
    #[allow(clippy::too_many_arguments)]
    fn blur_vsm(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        target: &ShadowTarget,
        vsm_layers: &[Option<(f64, u32)>],
        viewports: &[Vector4],
        size: u32,
    ) -> Result<wgpu::TextureView> {
        let mut cached = self.vsm.borrow_mut();
        if cached
            .as_ref()
            .is_none_or(|v| v.size != size || v.layers.len() != vsm_layers.len())
        {
            let texture = vsm_texture(device, size, vsm_layers.len() as u32);
            let view = texture.create_view(&wgpu::TextureViewDescriptor {
                dimension: Some(wgpu::TextureViewDimension::D2Array),
                ..Default::default()
            });
            let layers: Vec<wgpu::TextureView> = (0..vsm_layers.len())
                .map(|layer| {
                    texture.create_view(&wgpu::TextureViewDescriptor {
                        dimension: Some(wgpu::TextureViewDimension::D2),
                        base_array_layer: layer as u32,
                        array_layer_count: Some(1),
                        ..Default::default()
                    })
                })
                .collect();
            let pass_view =
                vsm_texture(device, size, 1).create_view(&wgpu::TextureViewDescriptor {
                    dimension: Some(wgpu::TextureViewDimension::D2),
                    ..Default::default()
                });
            let passes = target
                .layers
                .iter()
                .map(|depth| {
                    std::array::from_fn(|k| {
                        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
                            label: Some("vsm blur"),
                            size: 32,
                            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                            mapped_at_creation: false,
                        });
                        // The vertical pass reads the depth; the horizontal the pass target.
                        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                            label: Some("vsm blur"),
                            layout: &self.vsm_layout,
                            entries: &[
                                wgpu::BindGroupEntry {
                                    binding: 0,
                                    resource: buffer.as_entire_binding(),
                                },
                                wgpu::BindGroupEntry {
                                    binding: 1,
                                    resource: wgpu::BindingResource::TextureView(depth),
                                },
                                wgpu::BindGroupEntry {
                                    binding: 2,
                                    resource: wgpu::BindingResource::TextureView(if k == 0 {
                                        &layers[0]
                                    } else {
                                        &pass_view
                                    }),
                                },
                                wgpu::BindGroupEntry {
                                    binding: 3,
                                    resource: wgpu::BindingResource::Sampler(&self.vsm_sampler),
                                },
                            ],
                        });
                        (buffer, group)
                    })
                })
                .collect();
            *cached = Some(VsmTarget {
                view,
                layers,
                passes,
                pass_view,
                size,
            });
        }
        let v = cached.as_ref().ok_or(Error::Invalid("vsm target"))?;
        for (layer, blur) in vsm_layers.iter().enumerate() {
            let Some((radius, samples)) = blur else {
                continue;
            };
            let rect = viewports[layer] * size as f64;
            let data: [f32; 8] = [
                rect.x as f32,
                rect.y as f32,
                rect.z as f32,
                rect.w as f32,
                *radius as f32,
                *samples as f32,
                0.,
                0.,
            ];
            for (k, output) in [&v.pass_view, &v.layers[layer]].into_iter().enumerate() {
                let (buffer, group) = &v.passes[layer][k];
                queue.write_buffer(buffer, 0, bytemuck::cast_slice(&data));
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("vsm blur"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: output,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    ..Default::default()
                });
                pass.set_viewport(
                    rect.x as f32,
                    rect.y as f32,
                    rect.z as f32,
                    rect.w as f32,
                    0.,
                    1.,
                );
                pass.set_pipeline(&self.vsm_pipelines[k]);
                pass.set_bind_group(0, group, &[]);
                pass.draw(0..3, 0..1);
            }
        }
        Ok(v.view.clone())
    }
}
/// The shadow camera's view: Object3D.lookAt through Matrix4.lookAt, which
/// nudges a forward axis parallel to `up` by 0.0001 before the cross products.
fn view(position: Vector3, target: Vector3, up: Vector3) -> Matrix4 {
    let mut z = position - target;
    if z.length_squared() == 0. {
        z.z = 1.;
    }
    z = z.normalize();
    let mut x = up.cross(z);
    if x.length_squared() == 0. {
        if up.z.abs() == 1. {
            z.x += 0.0001;
        } else {
            z.z += 0.0001;
        }
        z = z.normalize();
        x = up.cross(z);
    }
    x = x.normalize();
    let y = z.cross(x);
    Matrix4::from_cols(
        x.extend(0.),
        y.extend(0.),
        z.extend(0.),
        position.extend(1.),
    )
    .inverse()
}
fn depth_texture(device: &wgpu::Device, size: u32, layers: u32) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("shadow atlas"),
        size: wgpu::Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: layers,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    })
}

fn depth_pipelines(
    device: &wgpu::Device,
    pipeline_layout: &wgpu::PipelineLayout,
    shader: &wgpu::ShaderModule,
) -> [wgpu::RenderPipeline; 10] {
    // 0–5: triangles by side, instanced first; 6–7: wireframe lines, as the
    // depth material inherits `wireframe`, and LineSegments; 8–9: Line strips.
    std::array::from_fn(|i| {
        let lines = i >= 6;
        let strip = i >= 8;
        let instanced = if lines { i == 6 || i == 8 } else { i < 3 };
        let cull_mode = if lines {
            None
        } else {
            [Some(wgpu::Face::Back), Some(wgpu::Face::Front), None][i % 3]
        };
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("shadow depth"),
            layout: Some(pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: Some("vs_main"),
                compilation_options: wgpu::PipelineCompilationOptions {
                    constants: &[("INSTANCED", if instanced { 1.0 } else { 0.0 })],
                    ..Default::default()
                },
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<crate::renderer::Vertex>() as u64,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[
                            wgpu::VertexAttribute {
                                format: wgpu::VertexFormat::Float32x3,
                                offset: 0,
                                shader_location: 0,
                            },
                            wgpu::VertexAttribute {
                                format: wgpu::VertexFormat::Float32x3,
                                offset: 12,
                                shader_location: 3,
                            },
                            wgpu::VertexAttribute {
                                format: wgpu::VertexFormat::Float32x2,
                                offset: 24,
                                shader_location: 1,
                            },
                            wgpu::VertexAttribute {
                                format: wgpu::VertexFormat::Float32x2,
                                offset: 72,
                                shader_location: 2,
                            },
                            wgpu::VertexAttribute {
                                format: wgpu::VertexFormat::Float32x2,
                                offset: 80,
                                shader_location: 4,
                            },
                        ],
                    },
                    crate::renderer::instance_layout(instanced),
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[],
            }),
            primitive: wgpu::PrimitiveState {
                topology: if strip {
                    wgpu::PrimitiveTopology::LineStrip
                } else if lines {
                    wgpu::PrimitiveTopology::LineList
                } else {
                    wgpu::PrimitiveTopology::TriangleList
                },
                cull_mode,
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: Default::default(),
            multiview: None,
            cache: None,
        })
    })
}

/// Resident depth-only program. Shading returns alpha as a binary shadow mask.
#[derive(Debug)]
pub struct ShadowProgram {
    pipelines: [wgpu::RenderPipeline; 10],
    bindings: wgpu::BindGroup,
}
impl ShadowProgram {
    pub async fn new(renderer: &crate::renderer::Renderer, hooks: &str) -> Result<Self> {
        Self::with_buffers(renderer, hooks, &[]).await
    }
    /// Resident read-only storage shared with the visible material and compute.
    /// Hooks use consecutive group 1 bindings, as in `ShaderProgram`.
    pub async fn with_buffers(
        renderer: &crate::renderer::Renderer,
        hooks: &str,
        buffers: &[&crate::compute::GpuBuffer],
    ) -> Result<Self> {
        if buffers
            .iter()
            .any(|b| matches!(b.access, crate::compute::BufferAccess::Uniform))
        {
            return Err(Error::Invalid("shadow binding requires storage buffer"));
        }
        let device = &renderer.device;
        device.push_error_scope(wgpu::ErrorFilter::Validation);
        let base = include_str!("shaders/shadow.wgsl");
        let source=base.replace("@location(1) world:vec3<f32>","@location(1) world:vec3<f32>,@location(2) local:vec3<f32>")
   .replace("@location(0) position:vec3<f32>,","@location(0) position:vec3<f32>,@location(3) normal:vec3<f32>,")
   .replace("var p=vec4(animated.position,1.0);","let local=deform(animated.position,normal,uv);var p=vec4(local,1.0);")
   .replace("out.world=(u.model*p).xyz;return out;","out.world=(u.model*p).xyz;out.local=local;return out;")
   .replace("@fragment fn fs_main(in:Out) {","@fragment fn fs_main(in:Out) {var surface:VertexOut;surface.position=in.world;surface.local_position=in.local;surface.uv=in.uv;surface.clip=in.position;if shade(surface,vec4(1.0)).a<0.5 {discard;}");
        let source = source
            .replace(
                "@builtin(vertex_index) vertex:u32,",
                "@builtin(vertex_index) vertex:u32,@builtin(instance_index) instance:u32,",
            )
            .replace(
                "let animated=",
                "vertex_instance_index=instance;tsl_vertex_index=vertex;let animated=",
            )
            .replace(
                "@location(2) local:vec3<f32>",
                "@location(2) local:vec3<f32>,@location(3) @interpolate(flat) instance:u32",
            )
            .replace("out.local=local;", "out.local=local;out.instance=instance;")
            .replace(
                "surface.position=in.world;",
                "surface.instance_index=in.instance;surface.position=in.world;",
            );
        let source = format!(
            "{}\n{}\n{}\n{}",
            include_str!("shaders/deformation.wgsl"),
            source,
            "var<private> vertex_instance_index:u32;var<private> tsl_vertex_index:u32;struct VertexOut {position:vec3<f32>,local_position:vec3<f32>,uv:vec2<f32>,clip:vec4<f32>,instance_index:u32};",
            hooks
        );
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("TSL shadow"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
        let resource_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("TSL shadow resources"),
            entries: &buffers
                .iter()
                .enumerate()
                .map(|(i, _)| wgpu::BindGroupLayoutEntry {
                    binding: i as u32,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                })
                .collect::<Vec<_>>(),
        });
        let bindings = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("TSL shadow resources"),
            layout: &resource_layout,
            entries: &buffers
                .iter()
                .enumerate()
                .map(|(i, b)| wgpu::BindGroupEntry {
                    binding: i as u32,
                    resource: b.buffer.as_entire_binding(),
                })
                .collect::<Vec<_>>(),
        });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("TSL shadow"),
            bind_group_layouts: &[&renderer.shadows.layout, &resource_layout],
            push_constant_ranges: &[],
        });
        let pipelines = depth_pipelines(device, &layout, &module);
        if let Some(error) = device.pop_error_scope().await {
            return Err(Error::Gpu(error.to_string()));
        }
        Ok(Self {
            pipelines,
            bindings,
        })
    }
}
