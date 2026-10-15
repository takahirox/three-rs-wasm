use super::*;
use crate::{
    compute::{BufferAccess, GpuBuffer},
    tsl::{compute::*, *},
};
pub(super) struct Jelly {
    init: BufferCompute,
    update: BufferCompute,
    initialized: bool,
    pending: bool,
}
impl Jelly {
    pub fn render(&mut self, r: &Renderer, params: &[[f32; 4]; 16]) -> Result<()> {
        if !self.initialized {
            self.init.dispatch(r)?;
            self.initialized = true;
        }
        if self.pending {
            self.update.set_uniforms(r, params)?;
            self.update.dispatch(r)?;
            self.pending = false;
        }
        Ok(())
    }
    pub fn step(&mut self) {
        self.pending = true;
    }
}
impl Demo {
    pub(super) async fn jelly(scene: &mut Scene, cam: Object3D, r: &Renderer) -> Result<Self> {
        let aspect = match scene.camera(cam)?.0 {
            Camera::Perspective(c) => c.aspect,
            _ => 1.0,
        };
        scene.get_mut(cam)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 50.0,
            aspect,
            near: 0.1,
            far: 10.0,
            ..Default::default()
        }));
        scene.get_mut(cam)?.position = Vector3::Z;
        scene.look_at(cam, Vector3::ZERO)?;
        let rgb = |hex| {
            let c = Color::from_hex(hex).0;
            vec3(float(c.x as f32), float(c.y as f32), float(c.z as f32))
        };
        let bg = mix(rgb(0x9f87f7), rgb(0xf2cdcd), float(1.0) - uv().y())
            * (float(1.0) - ((uv() - float(0.5)).length() - float(0.3)) / float(0.5))
                .clamp(float(0.0), float(1.0))
            * rgb(0xa78ff6)
            * float(4.0);
        let material = surface::background_material(r, bg).await?;
        let back = scene.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(2.0, 2.0, 1, 1)?),
            Arc::new(Material::Shader(material)),
        )));
        scene.get_mut(back)?.frustum_culled = false;
        scene.get_mut(back)?.render_order = i32::MIN;
        let (asset, buffers, images) =
            super::super::gltf_viewer::load_asset("/web/models/LeePerrySmith.glb").await?;
        let instance =
            crate::gltf::import_animated_decoded(&asset, &buffers, &images)?.instantiate(scene)?;
        let h = instance.meshes[0];
        let geometry = scene.get(h)?.geometry().unwrap();
        let attr = geometry.get_attribute("position").unwrap();
        let count = geometry.vertex_count() as u32;
        let mut base = Vec::new();
        for i in 0..count as usize {
            base.extend([
                attr.get_component(i, 0)? as f32,
                attr.get_component(i, 1)? as f32,
                attr.get_component(i, 2)? as f32,
                0.0,
            ]);
        }
        let base = GpuBuffer::new(r, bytemuck::cast_slice(&base), BufferAccess::Read)?;
        let current = GpuBuffer::zeroed(r, count as u64 * 16, BufferAccess::ReadWrite)?;
        let velocity = GpuBuffer::zeroed(r, count as u64 * 16, BufferAccess::ReadWrite)?;
        let bindings = [
            (&base, Type::Vec3),
            (&current, Type::Vec3),
            (&velocity, Type::Vec3),
        ];
        let index = instance_index();
        let b = storage_element(0, index.clone());
        let p = storage_element(1, index.clone());
        let v = storage_element(2, index.clone());
        let init = BufferCompute::new(
            r,
            count,
            &bindings,
            &[BufferStore {
                binding: 1,
                index: index.clone(),
                value: b.clone(),
            }],
        )
        .await?;
        let brush = uniform(2, Type::Vec4);
        let delta = brush.rgb() - p.clone() * float(0.1);
        let distance = delta.clone().length();
        let direction = delta / distance.clone().max(float(1e-20));
        let power = (uniform(1, Type::Vec4).swizzle("z") - distance).max(float(0.0))
            * uniform(1, Type::Vec4).swizzle("w");
        let displaced = p + brush
            .swizzle("w")
            .greater_than(float(0.5))
            .select(direction * power, splat(float(0.0), Type::Vec3));
        let force = (b.clone() - displaced.clone()).length()
            * uniform(1, Type::Float)
            * (b - displaced.clone());
        let speed = (v + force) * uniform(1, Type::Vec2).y();
        let update = BufferCompute::new(
            r,
            count,
            &bindings,
            &[
                BufferStore {
                    binding: 1,
                    index: index.clone(),
                    value: displaced + speed.clone(),
                },
                BufferStore {
                    binding: 2,
                    index,
                    value: speed,
                },
            ],
        )
        .await?;
        let graph = SurfaceNodes {
            position: Some(storage_element(0, vertex_index())),
            output: Some(WgslFn::new("normal_to_working", "fn normal_to_working(c:vec4<f32>)->vec4<f32>{return vec4(select(pow((c.rgb+0.055)/1.055,vec3(2.4)),c.rgb/12.92,c.rgb<=vec3(0.04045)),c.a);}", &[Type::Vec4],Type::Vec4)?.call(&[output()])),
            ..Default::default()
        };
        let mut m = MeshNormalMaterial::default();
        m.properties.vertex_program = Some(Arc::new(
            graph.build(r, &[(&current, Type::Vec3)], &[]).await?,
        ));
        if let NodeKind::Mesh(mesh) = &mut scene.get_mut(h)?.kind {
            mesh.materials = vec![Arc::new(Material::Normal(m))];
        }
        scene.get_mut(h)?.scale = Vector3::splat(0.1);
        let mut viewer = OrbitViewer::from_camera(Vector3::ZERO, 1.0);
        viewer.fixture(0.0, 0.0, 1.8);
        let mut params = [[0.0; 4]; 16];
        params[1] = [0.4, 0.94, 0.25, 0.22];
        Ok(Self {
            example: 75,
            time: 0.0,
            lights: vec![],
            objects: vec![h],
            params,
            viewer,
            orbit: Vector2::ZERO,
            pan: Vector3::ZERO,
            mixer: None,
            depth: None,
            mrt_sampler: None,
            bloom: None,
            storage: None,
            mask: None,
            jelly: Some(Jelly {
                init,
                update,
                initialized: false,
                pending: true,
            }),
        })
    }
    pub(in crate::browser) fn gpu_pointer(
        &mut self,
        scene: &mut Scene,
        cam: Object3D,
        x: f64,
        y: f64,
    ) -> Result<()> {
        if self.example != 75 {
            return Ok(());
        }
        // OrbitControls applies a drag before the window pointer raycast.
        self.viewer.orbit_pixels(
            self.orbit.x,
            self.orbit.y,
            0.0,
            self.params[0][1] as f64,
            0.7,
            2.0,
        );
        self.viewer.pan_world(self.pan);
        self.orbit = Vector2::ZERO;
        self.pan = Vector3::ZERO;
        self.viewer.update(scene, cam)?;
        scene.update()?;
        let (camera, world) = scene.camera(cam)?;
        let mut ray = crate::raycast::Raycaster::default();
        ray.set_from_camera(Vector2::new(x, y), camera, world)?;
        self.params[2] = ray
            .intersect_objects(scene, &self.objects, false)?
            .first()
            .map_or([0.0; 4], |hit| hit.point.extend(1.0).as_vec4().to_array());
        Ok(())
    }
}
