//! misc_raycaster_helper: three capsules crossing a fixed ray, and
//! @gsimone/three-raycaster-helper 0.1.0 drawing the ray, its near and far
//! planes and up to 20 hit points.
use crate::raycast::Raycaster;
use crate::{
    Error, Result, attribute::BufferAttribute, camera::*, geometry::*, material::*, math::*,
    renderer::*, scene::*,
};
use std::f64::consts::PI;
use std::sync::Arc;

const HITS: usize = 20;
fn basic(hex: u32) -> Arc<Material> {
    let mut m = MeshBasicMaterial::default();
    m.properties.color = Color::from_hex(hex);
    Arc::new(Material::Basic(m))
}
fn line_material(hex: u32) -> Arc<Material> {
    let mut m = LineBasicMaterial::default();
    m.properties.color = Color::from_hex(hex);
    Arc::new(Material::Line(m))
}
fn line(points: &[f64]) -> Result<Arc<BufferGeometry>> {
    let mut g = BufferGeometry::default();
    g.set_attribute(
        "position",
        Attribute::F32(BufferAttribute::new(
            points.iter().map(|&v| v as f32).collect(),
            3,
            false,
        )?),
    );
    Ok(Arc::new(g))
}
pub(super) struct Demo {
    time: f64,
    capsules: [Object3D; 3],
    raycaster: Raycaster,
    origin: Object3D,
    /// The origin sphere's colors with and without hits, kept resident.
    origin_materials: [Arc<Material>; 2],
    hit_points: Object3D,
    /// The helper's shared `_o` position: a missing hit keeps the last point
    /// under a zero scale.
    last_point: Vector3,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 70.,
            near: 1.,
            far: 1000.,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(0., 0., 10.);
        n.quaternion = Quaternion::IDENTITY;
        s.background = Color::BLACK;
        let geometry = Arc::new(CapsuleGeometry::build(0.5, 0.5, 4, 32, 1)?);
        let mut normal = MeshNormalMaterial::default();
        normal.properties.side = Side::Double;
        let normal = Arc::new(Material::Normal(normal));
        let mut capsules = vec![];
        for x in [-2., 0., 2.] {
            let h = s.insert(NodeKind::Mesh(Mesh::new(geometry.clone(), normal.clone())));
            s.get_mut(h)?.position.x = x;
            capsules.push(h);
        }
        let mut raycaster = Raycaster::default();
        raycaster.set(Vector3::new(-4., 0., 0.), Vector3::X);
        raycaster.near = 1.;
        raycaster.far = 8.;
        // RaycasterHelper( raycaster ): children in its add() order.
        let helper = s.insert(NodeKind::Group);
        let origin_point = raycaster.ray.origin;
        let direction = raycaster.ray.direction;
        let near_point = origin_point + direction * raycaster.near;
        let far_point = origin_point + direction * raycaster.far;
        // update() rewrites both lines with the same points every frame; the
        // ray never moves, so they stay resident here.
        let near_to_far = line(&[
            near_point.x,
            near_point.y,
            near_point.z,
            far_point.x,
            far_point.y,
            far_point.z,
        ])?;
        let origin_to_near = line(&[
            origin_point.x,
            origin_point.y,
            origin_point.z,
            near_point.x,
            near_point.y,
            near_point.z,
        ])?;
        let size = 0.1;
        let square = line(&[
            -size, size, 0., size, size, 0., size, -size, 0., -size, -size, 0., -size, size, 0.,
        ])?;
        let mut parts = vec![];
        for (geometry, hex) in [
            (near_to_far, 0xffffff),
            (origin_to_near, 0x333333),
            (square.clone(), 0xffffff),
            (square, 0xffffff),
        ] {
            let h = s.insert(NodeKind::Line(Line {
                geometry,
                material: line_material(hex),
                segments: false,
            }));
            s.add(helper, h)?;
            parts.push(h);
        }
        let origin_materials = [basic(0x0eec82), basic(0xff005b)];
        let origin = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(SphereGeometry::build(0.04, 32, 16)?),
            origin_materials[1].clone(),
        )));
        s.add(helper, origin)?;
        let hit_points = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(SphereGeometry::build(0.04, 32, 16)?),
            basic(0xffffff),
        )));
        s.get_mut(hit_points)?.instances = vec![Instance::default(); HITS];
        s.add(helper, hit_points)?;
        s.get_mut(origin)?.position = origin_point;
        s.get_mut(parts[2])?.position = near_point;
        s.get_mut(parts[3])?.position = far_point;
        s.look_at(parts[3], origin_point)?;
        s.look_at(parts[2], origin_point)?;
        Ok(Self {
            time: 0.,
            capsules: [capsules[0], capsules[1], capsules[2]],
            raycaster,
            origin,
            origin_materials,
            hit_points,
            last_point: Vector3::ZERO,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// animate( time ): move the capsules, then intersect. As in the original,
    /// the raycast reads the world matrices of the previous render.
    pub fn prepare(&mut self, s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        let t = self.time;
        for &h in &self.capsules {
            let n = s.get_mut(h)?;
            n.position.y = (t * 0.5 + n.position.x).sin();
            n.quaternion = Quaternion::from_rotation_z((t * 0.5).sin() * PI);
        }
        let hits = self.raycaster.intersect_objects(s, &self.capsules, true)?;
        let n = s.get_mut(self.hit_points)?;
        for i in 0..HITS {
            let scale = if let Some(hit) = hits.get(i) {
                self.last_point = hit.point;
                1.
            } else {
                0.
            };
            n.instances[i].matrix = Matrix4::from_scale_rotation_translation(
                Vector3::splat(scale),
                Quaternion::IDENTITY,
                self.last_point,
            );
        }
        let material = self.origin_materials[usize::from(hits.is_empty())].clone();
        if let NodeKind::Mesh(m) = &mut s.get_mut(self.origin)?.kind
            && !Arc::ptr_eq(&m.materials[0], &material)
        {
            m.materials[0] = material;
        }
        Ok(())
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
        Err(Error::Invalid("raycaster helper parameter"))
    }
    pub fn take_export(&mut self) -> Option<(String, Vec<u8>)> {
        None
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
