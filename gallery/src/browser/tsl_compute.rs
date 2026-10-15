//! Five pinned TSL storage/instancing scenes. Simulation arrays never cross the CPU.
use super::gltf_viewer::{OrbitViewer, decode_image, fetch};
use crate::tsl::Node;
use crate::{
    Error, Result,
    attribute::BufferAttribute,
    camera::*,
    compute::{BufferAccess, GpuBuffer},
    geometry::*,
    material::*,
    math::*,
    renderer::*,
    scene::*,
    tsl::{self, compute::*, sprites::SpriteNodeMaterial, *},
};
use std::sync::Arc;
fn f(x: f32) -> Node {
    float(x)
}
fn v3(x: f32, y: f32, z: f32) -> Node {
    vec3(f(x), f(y), f(z))
}
fn scalar(i: usize) -> Node {
    uniform(i, Type::Float)
}
fn rgb(hex: u32) -> Node {
    let c = Color::from_hex(hex).0;
    v3(c.x as f32, c.y as f32, c.z as f32)
}
fn rand(seed: &mut u32) -> f64 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    *seed as f64 / 4294967296.0
}
fn range(
    r: &Renderer,
    count: usize,
    seed: u32,
    low: [f64; 4],
    high: [f64; 4],
) -> Result<GpuBuffer> {
    let mut seed = seed;
    let data: Vec<[f32; 4]> = (0..count)
        .map(|_| std::array::from_fn(|i| (low[i] + (high[i] - low[i]) * rand(&mut seed)) as f32))
        .collect();
    GpuBuffer::new(r, bytemuck::cast_slice(&data), BufferAccess::Read)
}
fn store(binding: usize, value: Node) -> BufferStore {
    BufferStore {
        binding,
        index: instance_index(),
        value,
    }
}
fn mesh(scene: &mut Scene, g: BufferGeometry, m: ShaderMaterial) -> Object3D {
    let h = scene.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(g),
        Arc::new(Material::Shader(m)),
    )));
    scene.get_mut(h).unwrap().frustum_culled = false;
    h
}
fn grid(scene: &mut Scene, size: f64, div: u32, color: u32, y: f64) -> Result<()> {
    let mut p = Vec::new();
    let half = size / 2.0;
    for i in 0..=div {
        let k = i as f64 * size / div as f64 - half;
        for a in [[-half, y, k], [half, y, k], [k, y, -half], [k, y, half]] {
            p.extend(a.map(|x| x as f32));
        }
    }
    let mut g = BufferGeometry::default();
    g.set_attribute(
        "position",
        Attribute::F32(BufferAttribute::new(p, 3, false)?),
    );
    let mut m = LineBasicMaterial::default();
    m.properties.color = Color::from_hex(color);
    scene.insert(NodeKind::Line(Line {
        geometry: Arc::new(g),
        material: Arc::new(Material::Line(m)),
        segments: true,
    }));
    Ok(())
}
struct Simulation {
    init: BufferCompute,
    update: BufferCompute,
    hit: Option<BufferCompute>,
    initialized: bool,
}
struct PingPong {
    init: TextureKernel,
    updates: [TextureKernel; 2],
    materials: [Arc<Material>; 2],
    mipmaps: Vec<crate::mipmap::MipGenerator>,
    phase: usize,
    last_second: i64,
    seed: u32,
}
pub(super) struct Demo {
    example: u32,
    step_pending: bool,
    time: f64,
    objects: Vec<Object3D>,
    params: [[f32; 4]; 16],
    viewer: OrbitViewer,
    orbit: Vector2,
    pan: Vector3,
    pointer: Vector2,
    dragging: bool,
    simulation: Option<Simulation>,
    ping: Option<PingPong>,
}
impl Demo {
    pub async fn create(
        scene: &mut Scene,
        cam: Object3D,
        example: u32,
        r: &Renderer,
    ) -> Result<Self> {
        let aspect = match scene.camera(cam)?.0 {
            Camera::Perspective(c) => c.aspect,
            _ => 1.0,
        };
        let (fov, near, far, position, target) = match example {
            53 => (
                60.0,
                1.0,
                5000.0,
                Vector3::new(1300.0, 500.0, 0.0),
                Vector3::new(0.0, 500.0, 0.0),
            ),
            54 => (60.0, 0.1, 100.0, Vector3::splat(9.0), Vector3::ZERO),
            56 => (
                50.0,
                0.1,
                1000.0,
                Vector3::new(0.0, 5.0, 20.0),
                Vector3::new(0.0, -8.0, 0.0),
            ),
            _ => (60.0, 0.0, 2.0, Vector3::Z, Vector3::ZERO),
        };
        scene.get_mut(cam)?.kind = NodeKind::Camera(if [55, 57].contains(&example) {
            Camera::Orthographic(OrthographicCamera {
                left: if example == 57 { -aspect } else { -1.0 },
                right: if example == 57 { aspect } else { 1.0 },
                near: 0.0,
                far: if example == 55 { 1.0 } else { 2.0 },
                ..Default::default()
            })
        } else {
            Camera::Perspective(PerspectiveCamera {
                fov,
                aspect,
                near,
                far,
                ..Default::default()
            })
        });
        scene.get_mut(cam)?.position = position;
        scene.look_at(cam, target)?;
        scene.background = if example == 53 {
            Color::from_hex(0x333333)
        } else {
            Color::BLACK
        };
        scene.background_alpha = if [54, 55, 57].contains(&example) {
            0.0
        } else {
            1.0
        };
        let offset = position - target;
        let mut viewer = OrbitViewer::from_camera(target, offset.length());
        viewer.fixture(
            offset.x.atan2(offset.z),
            (offset.y / offset.length()).asin(),
            1.8,
        );
        let mut out = Self {
            example,
            step_pending: true,
            time: 0.0,
            objects: vec![],
            params: [[0.0; 4]; 16],
            viewer,
            orbit: Vector2::ZERO,
            pan: Vector3::ZERO,
            pointer: Vector2::splat(-10.0),
            dragging: false,
            simulation: None,
            ping: None,
        };
        match example {
            53 => out.particles(scene, r).await?,
            54 => out.instances(scene, r).await?,
            55 | 56 => out.simulate(scene, r).await?,
            57 => out.texture(scene, r).await?,
            _ => return Err(Error::Invalid("TSL compute example")),
        }
        Ok(out)
    }
    async fn particles(&mut self, scene: &mut Scene, r: &Renderer) -> Result<()> {
        self.params[1][0] = 0.2;
        let buffers = [
            range(r, 2000, 186, [0.1; 4], [1.0; 4])?,
            range(r, 2000, 187, [-2.0, 3.0, -2.0, 0.0], [2.0, 5.0, 2.0, 0.0])?,
            range(r, 2000, 188, [0.3; 4], [2.0; 4])?,
            range(r, 2000, 189, [0.1; 4], [4.0; 4])?,
            range(r, 1000, 190, [-1.0, 1.0, -1.0, 0.0], [1.0, 2.0, 1.0, 0.0])?,
        ];
        let mut image = decode_image(&fetch("/web/gallery/assets/smoke1.png").await?).await?;
        image.srgb = false;
        image.mipmap_filter = Some(Filter::Linear);
        let tex = r.upload_texture(&Arc::new(image))?;
        let life_range = instanced_attribute(0).x();
        let scaled = (scalar(0) + f(5.0)) * scalar(1);
        let lifetime = (scaled.clone() * life_range.clone()).modulo(f(1.0));
        let life = lifetime.clone() / life_range;
        let angle = scaled * instanced_attribute(3).x();
        let p = uv() - f(0.5);
        let coord = vec2(
            angle.cos() * p.x() - angle.sin() * p.y(),
            angle.sin() * p.x() + angle.cos() * p.y(),
        ) + f(0.5);
        let sample = tsl::Texture::External(0).sample(vec2(coord.x(), f(1.0) - coord.y()));
        let opacity = sample.swizzle("w") * (f(1.0) - life.clone());
        let smoke = mix(
            rgb(0x2c1501),
            rgb(0x222222),
            (position_local().y() * f(3.0)).clamp(f(0.0), f(1.0)),
        );
        let color = mix(rgb(0xf27d0c), smoke, (life * f(2.5)).clamp(f(0.0), f(1.0)))
            * (f(1.0) - position_local().y()).max(f(0.2));
        for fire in [false, true] {
            let mut sprite = SpriteNodeMaterial::new(vec4(
                if fire { rgb(0xb72f17) } else { color.clone() },
                opacity.clone() * f(if fire { 0.5 } else { 1.0 }),
            ));
            sprite.position = instanced_attribute(1).rgb() * lifetime.clone();
            sprite.scale = instanced_attribute(2).x() * lifetime.clone().max(f(0.3));
            let mut m = sprite
                .build(
                    r,
                    &[
                        &buffers[0],
                        &buffers[if fire { 4 } else { 1 }],
                        &buffers[2],
                        &buffers[3],
                    ],
                    &[(&tex.view, &tex.sampler)],
                )
                .await?;
            m.properties.depth_write = false;
            if fire {
                m.properties.blending = Some(wgpu::BlendState {
                    color: wgpu::BlendComponent {
                        src_factor: wgpu::BlendFactor::SrcAlpha,
                        dst_factor: wgpu::BlendFactor::One,
                        operation: wgpu::BlendOperation::Add,
                    },
                    alpha: wgpu::BlendComponent {
                        src_factor: wgpu::BlendFactor::One,
                        dst_factor: wgpu::BlendFactor::One,
                        operation: wgpu::BlendOperation::Add,
                    },
                });
            }
            let mut g = PlaneGeometry::build(1.0, 1.0, 1, 1)?;
            g.instance_count = Some(if fire { 1000 } else { 2000 });
            if fire {
                g.set_indirect(Some(vec![6, 1000, 0, 0, 0]));
            }
            let h = mesh(scene, g, m);
            let n = scene.get_mut(h)?;
            n.scale = Vector3::splat(400.0);
            if fire {
                n.position.y = -100.0;
                n.render_order = 1;
            }
            self.objects.push(h);
        }
        grid(scene, 3000.0, 40, 0x444444, -75.0)
    }
    async fn instances(&mut self, scene: &mut Scene, r: &Renderer) -> Result<()> {
        self.params[1][0] = 1000.0;
        // This asset has one Float32 position attribute and a Uint16 index.
        #[derive(serde::Deserialize)]
        struct Array<T> {
            array: Vec<T>,
        }
        #[derive(serde::Deserialize)]
        struct Attributes {
            position: Array<f32>,
        }
        #[derive(serde::Deserialize)]
        struct Data {
            attributes: Attributes,
            index: Array<u16>,
        }
        #[derive(serde::Deserialize)]
        struct Asset {
            data: Data,
        }
        let asset: Asset = serde_json::from_slice(
            &fetch("/web/gallery/assets/suzanne_buffergeometry.json").await?,
        )
        .map_err(|e| Error::Asset(e.to_string()))?;
        let mut g = BufferGeometry::default();
        g.set_attribute(
            "position",
            Attribute::F32(BufferAttribute::new(
                asset.data.attributes.position.array,
                3,
                false,
            )?),
        );
        g.set_index(Some(
            asset.data.index.array.into_iter().map(u32::from).collect(),
        ));
        g.compute_vertex_normals()?;
        g.scale(Vector3::splat(0.5))?;
        let colors = range(r, 1000, 186, [0.0, 0.0, 0.0, 1.0], [1.0; 4])?;
        let osc =
            ((scalar(0) * f(0.1) + f(0.75)) * f(std::f32::consts::TAU)).sin() * f(0.5) + f(0.5);
        let m =
            NodeMaterial::new(mix(normal_world(), instanced_attribute(0).rgb(), osc).max(f(0.0)))
                .build_with_storage(r, &[(&colors, Type::Vec4)], &[])
                .await?;
        let h = mesh(scene, g, m);
        scene.get_mut(h)?.instances = vec![
            Instance {
                matrix: Matrix4::IDENTITY,
                color: Color::WHITE
            };
            1000
        ];
        self.objects.push(h);
        Ok(())
    }
    async fn simulate(&mut self, scene: &mut Scene, r: &Renderer) -> Result<()> {
        let points = self.example == 55;
        let count = if points { 300000 } else { 200000 };
        let ty = if points { Type::Vec2 } else { Type::Vec3 };
        let stride = if points { 8 } else { 16 };
        let position = GpuBuffer::zeroed(r, count as u64 * stride, BufferAccess::ReadWrite)?;
        let velocity = GpuBuffer::zeroed(r, count as u64 * stride, BufferAccess::ReadWrite)?;
        let colors = if points {
            None
        } else {
            Some(GpuBuffer::zeroed(
                r,
                count as u64 * 16,
                BufferAccess::ReadWrite,
            )?)
        };
        let mut buffers = vec![(&position, ty), (&velocity, ty)];
        if let Some(c) = &colors {
            buffers.push((c, Type::Vec3));
        }
        let index = instance_index();
        let pos = storage_element(0, index.clone());
        let vel = storage_element(1, index.clone());
        let (initial, updates) = if points {
            self.params[1] = [1.0, 1.0, 0.0, 0.0];
            self.params[2] = [-10.0, -10.0, 0.0, 0.0];
            let angle = index.to_float() * f(0.005) * f(std::f32::consts::TAU);
            let speed = index.to_float() * f(0.00000001) + f(0.0000001);
            let next = pos.clone() + vel.clone();
            let limit = uniform(1, Type::Vec2);
            let vx = next
                .x()
                .abs()
                .less_than(limit.x())
                .select(vel.x(), -vel.x());
            let vy = next
                .y()
                .abs()
                .less_than(limit.y())
                .select(vel.y(), -vel.y());
            let next = next.clamp(-limit.clone(), limit);
            let next = (uniform(2, Type::Vec2) - next.clone())
                .length()
                .greater_than(f(0.1))
                .select(next, splat(f(0.0), Type::Vec2));
            (
                vec![
                    store(0, splat(f(0.0), Type::Vec2)),
                    store(1, vec2(angle.sin(), angle.cos()) * speed),
                ],
                vec![store(0, next), store(1, vec2(vx, vy))],
            )
        } else {
            self.params[1] = [-0.00098, 0.8, 0.99, 0.12];
            let amount = (count as f32).sqrt();
            let x = index.clone().modulo(uint(amount as u32)).to_float();
            let z = (index.clone() / uint(amount as u32)).to_float();
            let start = vec3(
                (f(amount / 2.0) - x) * f(0.2),
                f(0.0),
                (f(amount / 2.0) - z) * f(0.2),
            );
            let v = vel + vec3(f(0.0), uniform(1, Type::Vec4).x(), f(0.0));
            let p = pos + v.clone();
            let v = v * uniform(1, Type::Vec4).swizzle("z");
            let ground = p.y().less_than(f(0.0));
            let newp = vec3(p.x(), p.y().max(f(0.0)), p.swizzle("z"));
            let bounced = vec3(
                v.x() * f(0.9),
                -v.y() * uniform(1, Type::Vec4).y(),
                v.swizzle("z") * f(0.9),
            );
            (
                vec![
                    store(0, start),
                    store(1, splat(f(0.0), Type::Vec3)),
                    store(
                        2,
                        vec3(hash(index.clone()), hash(index.clone() + uint(2)), f(0.0)),
                    ),
                ],
                vec![store(0, newp), store(1, ground.select(bounced, v))],
            )
        };
        let init = BufferCompute::new(r, count, &buffers, &initial).await?;
        let update = BufferCompute::new(r, count, &buffers, &updates).await?;
        let hit = if points {
            None
        } else {
            let pos = storage_element(0, index.clone());
            let vel = storage_element(1, index.clone());
            let delta = pos - uniform(2, Type::Vec3);
            let power =
                (f(3.0) - delta.length()).max(f(0.0)) * f(0.01) * (hash(index) * f(1.5) + f(0.5));
            Some(
                BufferCompute::new(
                    r,
                    count,
                    &buffers,
                    &[store(1, vel + delta.normalize() * power)],
                )
                .await?,
            )
        };
        let h = if points {
            let p = storage_element(0, instance_index());
            let mut node = NodeMaterial::new(vec3(p.x() + f(1.0), p.y() + f(1.0), f(1.0)));
            node.position = Some(vec3(p.x(), p.y(), f(0.0)));
            let m = node
                .build_with_storage(r, &[(&position, Type::Vec2)], &[])
                .await?;
            let mut g = BufferGeometry::default();
            g.set_attribute(
                "position",
                Attribute::F32(BufferAttribute::new(vec![0.0f32; 3], 3, false)?),
            );
            g.instance_count = Some(count);
            let h = scene.insert(NodeKind::Points(Points {
                geometry: Arc::new(g),
                material: Arc::new(Material::Shader(m)),
            }));
            scene.get_mut(h)?.frustum_culled = false;
            h
        } else {
            let circle = shape_circle(true);
            let c = storage_element(1, instance_index()) * vec3(uv().x(), uv().y(), f(0.0));
            let mut sprite = SpriteNodeMaterial::new(vec4(c, circle));
            sprite.position = storage_element(0, instance_index());
            sprite.scale = uniform(1, Type::Vec4).swizzle("w");
            let mut m = sprite
                .build_with_storage(
                    r,
                    &[
                        (&position, Type::Vec3),
                        (colors.as_ref().unwrap(), Type::Vec3),
                    ],
                    &[],
                )
                .await?;
            m.properties.alpha_to_coverage = true;
            let mut g = PlaneGeometry::build(1.0, 1.0, 1, 1)?;
            g.instance_count = Some(count);
            grid(scene, 90.0, 45, 0x303030, 0.0)?;
            mesh(scene, g, m)
        };
        self.objects.push(h);
        self.simulation = Some(Simulation {
            init,
            update,
            hit,
            initialized: false,
        });
        Ok(())
    }
    async fn texture(&mut self, scene: &mut Scene, r: &Renderer) -> Result<()> {
        let format = wgpu::TextureFormat::Rgba16Float;
        let mut views = vec![];
        let mut sampled_views = vec![];
        let mut mipmaps = vec![];
        for _ in 0..2 {
            let texture = r.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("TSL ping-pong HDR"),
                size: wgpu::Extent3d {
                    width: 512,
                    height: 512,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 10,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage: wgpu::TextureUsages::STORAGE_BINDING
                    | wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            });
            views.push(texture.create_view(&wgpu::TextureViewDescriptor {
                mip_level_count: Some(1),
                ..Default::default()
            }));
            sampled_views.push(texture.create_view(&Default::default()));
            mipmaps.push(crate::mipmap::MipGenerator::new(&r.device, &texture)?);
        }
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let i = instance_index();
        let x = i.clone().modulo(uint(512));
        let y = i / uint(512);
        let coord = uvec2(x.clone(), y.clone());
        let p = vec2(x.to_float(), y.to_float());
        let coordinate_uv = p.clone() / f(512.0);
        let seed = uniform(0, Type::Vec2);
        let random = |scale: f32| {
            ((coordinate_uv.clone() + seed.clone() * f(scale))
                .dot(vec2(f(12.9898), f(4.1414)))
                .sin()
                * f(43_758.547))
            .fract()
        };
        let color = vec4(
            vec3(
                random(100.0) - random(300.0),
                random(200.0) - random(300.0),
                random(200.0) - random(100.0),
            ),
            f(1.0),
        );
        let init = TextureKernel::new(r, 512 * 512, coord.clone(), color, (&views[0], format), &[])
            .await?;
        let mut blur = texture_load(0, p.clone() + vec2(f(-1.0), f(1.0)));
        for (x, y) in [(-1.0, -1.0), (0.0, 0.0), (1.0, -1.0), (1.0, 1.0)] {
            blur = blur + texture_load(0, p.clone() + vec2(f(x), f(y)));
        }
        let blur = vec4((blur / f(5.0)).rgb() * f(1.05), f(1.0));
        let a = TextureKernel::new(
            r,
            512 * 512,
            coord.clone(),
            blur.clone(),
            (&views[1], format),
            &[(&views[0], format)],
        )
        .await?;
        let b = TextureKernel::new(
            r,
            512 * 512,
            coord,
            blur,
            (&views[0], format),
            &[(&views[1], format)],
        )
        .await?;
        let node = NodeMaterial::new(tsl::Texture::External(0).sample(uv()).max(f(0.0)));
        let m0 = node.build(r, &[(&sampled_views[1], &sampler)]).await?;
        let m1 = node.build(r, &[(&sampled_views[0], &sampler)]).await?;
        let materials = [
            Arc::new(Material::Shader(m0.clone())),
            Arc::new(Material::Shader(m1)),
        ];
        self.objects
            .push(mesh(scene, PlaneGeometry::build(1.0, 1.0, 1, 1)?, m0));
        self.ping = Some(PingPong {
            init,
            updates: [a, b],
            materials,
            mipmaps,
            phase: 0,
            last_second: -1,
            seed: 186,
        });
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
        self.step_pending = true;
    }
    pub fn parameter(&mut self, i: usize, v: f32) -> Result<()> {
        let ranges: &[(f32, f32)] = match self.example {
            53 => &[(0.0, 1.0)],
            54 => &[(1.0, 1000.0)],
            55 => &[(0.0, 1.0), (0.0, 1.0)],
            56 => &[(-0.0098, 0.0), (0.1, 1.0), (0.96, 0.99), (0.12, 0.5)],
            _ => &[],
        };
        if !v.is_finite() || ranges.get(i).is_none_or(|(a, b)| v < *a || v > *b) {
            return Err(Error::Invalid("TSL compute parameter"));
        }
        self.params[1][i] = v;
        Ok(())
    }
    pub fn pointer(&mut self, x: f64, y: f64) {
        self.pointer = Vector2::new(x, y);
    }
    pub fn gpu_pointer(
        &mut self,
        r: &Renderer,
        scene: &mut Scene,
        cam: Object3D,
        x: f64,
        y: f64,
    ) -> Result<()> {
        self.pointer(x, y);
        if self.example != 56 || self.dragging {
            return Ok(());
        }
        let s = self.simulation.as_mut().unwrap();
        if !s.initialized {
            s.init.dispatch(r)?;
            s.initialized = true;
        }
        scene.update_world_matrix(cam, true, false)?;
        let (camera, world) = scene.camera(cam)?;
        let inv = camera.projection_matrix()?.inverse();
        let a = world.transform_point3(inv.project_point3(Vector3::new(
            self.pointer.x,
            self.pointer.y,
            0.0,
        )));
        let b = world.transform_point3(inv.project_point3(Vector3::new(
            self.pointer.x,
            self.pointer.y,
            1.0,
        )));
        let dir = (b - a).normalize();
        let t = -a.y / dir.y;
        if t >= 0.0 {
            let p = a + dir * t;
            if p.x.abs() <= 100.0 && p.z.abs() <= 100.0 {
                self.params[2] = [p.x as f32, -1.0, p.z as f32, 0.0];
                if let Some(hit) = &s.hit {
                    hit.set_uniforms(r, &self.params)?;
                    hit.dispatch(r)?;
                }
            }
        }
        Ok(())
    }
    pub fn dragging(&mut self, value: bool) {
        self.dragging = value;
    }
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        scene: &Scene,
        cam: Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Result<()> {
        if ![53, 56].contains(&self.example) {
            return Ok(());
        }
        if pan {
            self.pan += OrbitViewer::pan_delta(scene, cam, self.viewer.radius(), dx, dy, height)?;
        } else {
            self.orbit += Vector2::new(dx, dy);
            self.viewer.orbit_pixels(
                0.0,
                0.0,
                wheel,
                height,
                if self.example == 56 { 5.0 } else { 0.1 },
                if self.example == 56 { 200.0 } else { 2700.0 },
            );
        }
        Ok(())
    }
    pub fn update(
        &mut self,
        scene: &mut Scene,
        cam: Object3D,
        delta: f64,
        animate: bool,
    ) -> Result<()> {
        if animate {
            self.step_pending = true;
            self.time += delta;
        }
        self.params[0][0] = self.time as f32;
        if self.example == 55 {
            self.params[2][0] = self.pointer.x as f32;
            self.params[2][1] = self.pointer.y as f32;
        }
        if [53, 56].contains(&self.example) {
            let damping = if self.example == 56 { 0.05 } else { 1.0 };
            let height = web_sys::window()
                .unwrap()
                .inner_height()
                .unwrap()
                .as_f64()
                .unwrap_or(512.0);
            self.viewer.orbit_pixels(
                self.orbit.x * damping,
                self.orbit.y * damping,
                0.0,
                height,
                if self.example == 56 { 5.0 } else { 0.1 },
                if self.example == 56 { 200.0 } else { 2700.0 },
            );
            self.orbit *= 1.0 - damping;
            self.viewer.pan_world(self.pan * damping);
            self.pan *= 1.0 - damping;
            self.viewer.update(scene, cam)?;
            if let NodeKind::Camera(Camera::Perspective(c)) = &mut scene.get_mut(cam)?.kind {
                c.near = if self.example == 53 { 1.0 } else { 0.1 };
                c.far = if self.example == 53 { 5000.0 } else { 1000.0 };
            }
        }
        for &h in &self.objects {
            let n = scene.get_mut(h)?;
            if self.example == 54 {
                n.quaternion = Quaternion::from_euler(
                    glam::EulerRot::XYZ,
                    (self.time / 4.0).sin(),
                    (self.time / 2.0).sin(),
                    0.0,
                );
                n.instances.resize(
                    self.params[1][0] as usize,
                    Instance {
                        matrix: Matrix4::IDENTITY,
                        color: Color::WHITE,
                    },
                );
                for (i, instance) in n.instances.iter_mut().enumerate() {
                    let x = (i / 100) as f64;
                    let y = ((i / 10) % 10) as f64;
                    let z = (i % 10) as f64;
                    let angle = (x / 4.0 + self.time).sin()
                        + (y / 4.0 + self.time).sin()
                        + (z / 4.0 + self.time).sin();
                    instance.matrix = Matrix4::from_rotation_translation(
                        Quaternion::from_euler(glam::EulerRot::XYZ, 0.0, angle, angle * 2.0),
                        Vector3::new(4.5 - x, 4.5 - y, 4.5 - z),
                    );
                }
            }
            match &mut n.kind {
                NodeKind::Mesh(m) => {
                    if let Material::Shader(m) = Arc::make_mut(&mut m.materials[0]) {
                        m.uniforms = self.params;
                    }
                }
                NodeKind::Points(p) => {
                    if let Material::Shader(m) = Arc::make_mut(&mut p.material) {
                        m.uniforms = self.params;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
    pub fn prepare(&mut self, scene: &mut Scene, cam: Object3D, aspect: f64) -> Result<()> {
        if self.example == 57
            && let NodeKind::Camera(Camera::Orthographic(c)) = &mut scene.get_mut(cam)?.kind
        {
            c.left = -aspect;
            c.right = aspect;
        }
        Ok(())
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        scene: &mut Scene,
        cam: Object3D,
        target: &RenderTarget,
    ) -> Result<bool> {
        if self.step_pending
            && let Some(s) = &mut self.simulation
        {
            if !s.initialized {
                s.init.dispatch(r)?;
                s.initialized = true;
            }
            s.update.set_uniforms(r, &self.params)?;
            s.update.dispatch(r)?;
        }
        if self.step_pending
            && let Some(p) = &mut self.ping
        {
            let seconds = self.time.floor() as i64;
            if p.phase == 0 && seconds != p.last_second {
                let mut values = [[0.0; 4]; 16];
                values[0][0] = rand(&mut p.seed) as f32;
                values[0][1] = rand(&mut p.seed) as f32;
                p.init.set_uniforms(r, &values)?;
                p.init.dispatch(r)?;
                p.mipmaps[0].update(&r.device, &r.queue);
                p.last_second = seconds;
            }
            p.updates[p.phase].dispatch(r)?;
            p.mipmaps[p.phase ^ 1].update(&r.device, &r.queue);
            if let NodeKind::Mesh(m) = &mut scene.get_mut(self.objects[0])?.kind {
                m.materials[0] = p.materials[p.phase].clone();
            }
            p.phase ^= 1;
        }
        self.step_pending = false;
        r.render(scene, cam, target)?;
        Ok(true)
    }
}
