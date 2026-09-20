use super::*;
use crate::compute::ComputeKernel;
pub(super) struct PointsPass {
    target: RenderTarget,
    kernel: ComputeKernel,
    uniform: GpuBuffer,
    camera: Object3D,
    background: Object3D,
}
impl Demo {
    pub(super) async fn points(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        self.params[..4].copy_from_slice(&[1.0, 6.0, 20.0, 6.0]);
        let position = Vector3::new(-40.0, 0.0, 60.0);
        self.viewer = OrbitViewer::from_camera(Vector3::ZERO, position.length());
        self.viewer.fixture(position.x.atan2(position.z), 0.0, 1.8);
        self.viewer.update(s, c)?;
        #[derive(serde::Deserialize)]
        struct Data {
            positions: Vec<[f32; 4]>,
            colors: Vec<[f32; 4]>,
        }
        let data: Data =
            serde_json::from_slice(&fetch("/web/gallery/assets/hilbert-points.json").await?)
                .map_err(|e| Error::Asset(e.to_string()))?;
        let positions =
            GpuBuffer::new(r, bytemuck::cast_slice(&data.positions), BufferAccess::Read)?;
        let colors = GpuBuffer::new(r, bytemuck::cast_slice(&data.colors), BufferAccess::Read)?;
        let sizes = GpuBuffer::zeroed(r, 256 * 4, BufferAccess::ReadWrite)?;
        let uniforms = GpuBuffer::zeroed(r, 16, BufferAccess::Uniform)?;
        let kernel=ComputeKernel::new(r,"@group(0) @binding(0) var<storage,read_write> sizes:array<f32>;@group(0) @binding(1) var<uniform> params:vec4<f32>;@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id:vec3<u32>){if(id.x>=256u){return;}let f=(sin((params.x+f32(id.x))*params.w)+1.0)*0.5;sizes[id.x]=f*(params.z-params.y)+params.y;}",&[&sizes,&uniforms]).await?;
        let size = storage_element(2, instance_index());
        let alpha = uniform(2, Type::Float)
            .less_than(float(0.5))
            .select(shape_circle(false), shape_circle(true));
        let mut graph = tsl::sprites::SpriteNodeMaterial::new(vec4(
            instanced_attribute(1).rgb() * (size.clone() / uniform(1, Type::Float)),
            alpha,
        ));
        graph.position = instanced_attribute(0).rgb();
        graph.scale = size * uniform(3, Type::Float);
        let mut material = graph
            .build_points(
                r,
                &[
                    (&positions, Type::Vec4),
                    (&colors, Type::Vec4),
                    (&sizes, Type::Float),
                ],
                &[],
            )
            .await?;
        material.properties.transparent = true;
        material.properties.alpha_to_coverage = true;
        let mut geometry = PlaneGeometry::build(1.0, 1.0, 1, 1)?;
        geometry.instance_count = Some(256);
        let h = mesh(s, Arc::new(geometry), Material::Shader(material));
        s.get_mut(h)?.frustum_culled = false;
        self.objects.push(h);
        let background = mesh(
            s,
            Arc::new(PlaneGeometry::build(2.0, 2.0, 1, 1)?),
            Material::Shader(tsl::surface::background_material(r, rgb(0x222222)).await?),
        );
        s.get_mut(background)?.frustum_culled = false;
        s.get_mut(background)?.render_order = -100;
        s.get_mut(background)?.visible = false;
        let camera = s.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 40.0,
            aspect: 1.0,
            near: 1.0,
            far: 1000.0,
            ..Default::default()
        })));
        let target = RenderTarget::with_options(
            &r.device,
            1,
            1,
            RenderTargetOptions {
                samples: 4,
                format: wgpu::TextureFormat::Rgba16Float,
                ..Default::default()
            },
        )?;
        self.points = Some(PointsPass {
            target,
            kernel,
            uniform: uniforms,
            camera,
            background,
        });
        Ok(())
    }
}
impl PointsPass {
    pub(super) fn output(&self) -> &RenderTarget {
        &self.target
    }
    pub(super) fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        output: &RenderTarget,
        time: f64,
        params: &[f32; 16],
    ) -> Result<()> {
        if (self.target.width, self.target.height) != (output.width, output.height) {
            self.target
                .set_size(&r.device, output.width, output.height)?;
        }
        self.uniform.write(
            r,
            0,
            bytemuck::cast_slice(&[time as f32, params[1], params[2], params[3]]),
        )?;
        self.kernel.dispatch(r, [4, 1, 1])?;
        s.get_mut(self.background)?.visible = false;
        self.target.options.load_color = false;
        self.target.viewport = [0, 0, output.width, output.height];
        self.target.scissor = None;
        r.render(s, c, &self.target)?;
        let position = s.get(c)?.position;
        let rotation = s.get(c)?.quaternion;
        s.get_mut(self.camera)?.position = position;
        s.get_mut(self.camera)?.quaternion = rotation;
        s.get_mut(self.background)?.visible = true;
        self.target.options.load_color = true;
        let size = (output.height / 4).max(1);
        let inset = (20.0 * web_sys::window().map_or(1.0, |w| w.device_pixel_ratio())) as u32;
        let size = size
            .min(output.width.saturating_sub(inset))
            .min(output.height.saturating_sub(inset));
        if size == 0 {
            return Ok(());
        }
        self.target.viewport = [
            inset,
            output.height.saturating_sub(size + inset),
            size,
            size,
        ];
        self.target.scissor = Some(self.target.viewport);
        r.render(s, self.camera, &self.target)?;
        s.get_mut(self.background)?.visible = false;
        Ok(())
    }
}
