//! webgl_geometry_csg: every frame subtracts ( or intersects, or adds ) a
//! turning, breathing cylinder brush from a turning icosahedron brush with the
//! port of three-bvh-csg ( csg_eval.rs ) on the CPU, as the page does, and
//! draws the result ( flat white where it comes from the icosahedron, teal
//! from the cylinder ) with an orange core inside, under a hemisphere light
//! and a PCF-shadowed directional light over a shadow-only plane. The result's
//! geometry is rewritten into the same resident buffers each frame; the
//! wireframe mesh shares it ( without the result's transform, as on the page ).
use super::controls_attributes::{Controls, camera_state};
use super::csg_eval::{self, Brush, Builder, M4, Source};
use crate::{
    Error, Result, attribute::*, camera::*, geometry::*, material::*, math::*, renderer::*,
    scene::*,
};
use std::{f64::consts::PI, sync::Arc};

/// The GUI's operation options in order: SUBTRACTION, INTERSECTION, ADDITION.
const OPERATIONS: [u32; 3] = [
    csg_eval::SUBTRACTION,
    csg_eval::INTERSECTION,
    csg_eval::ADDITION,
];
pub(super) struct Demo {
    controls: Controls,
    time: f64,
    base: Brush,
    brush: Brush,
    builder: Builder,
    operation: u32,
    use_groups: bool,
    wireframe: bool,
    result: Object3D,
    lines: Object3D,
    /// The result's materials: the icosahedron's and the cylinder's.
    materials: [Arc<Material>; 2],
    empty: Arc<BufferGeometry>,
}
fn source(g: &BufferGeometry) -> Result<Source> {
    let get = |name: &str, size: usize| -> Result<Vec<f32>> {
        let a = g
            .attributes
            .get(name)
            .ok_or(Error::Invalid("csg attribute"))?;
        (0..a.count())
            .flat_map(|i| (0..size).map(move |k| (i, k)))
            .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
            .collect()
    };
    Ok(Source {
        positions: get("position", 3)?,
        normals: get("normal", 3)?,
        uvs: get("uv", 2)?,
        index: g.index.clone(),
        groups: g
            .groups
            .iter()
            .map(|g| (g.start, g.count, g.material_index))
            .collect(),
    })
}
/// Object3D.updateMatrix(): Euler XYZ rotation and scale composed as three.js does.
fn compose(rotation: [f64; 3], scale: [f64; 3]) -> M4 {
    let [x, y, z] = rotation;
    let (c1, c2, c3) = ((x / 2.).cos(), (y / 2.).cos(), (z / 2.).cos());
    let (s1, s2, s3) = ((x / 2.).sin(), (y / 2.).sin(), (z / 2.).sin());
    let qx = s1 * c2 * c3 + c1 * s2 * s3;
    let qy = c1 * s2 * c3 - s1 * c2 * s3;
    let qz = c1 * c2 * s3 + s1 * s2 * c3;
    let qw = c1 * c2 * c3 - s1 * s2 * s3;
    let (x2, y2, z2) = (qx + qx, qy + qy, qz + qz);
    let (xx, xy, xz) = (qx * x2, qx * y2, qx * z2);
    let (yy, yz, zz) = (qy * y2, qy * z2, qz * z2);
    let (wx, wy, wz) = (qw * x2, qw * y2, qw * z2);
    let [sx, sy, sz] = scale;
    [
        (1. - (yy + zz)) * sx,
        (xy + wz) * sx,
        (xz - wy) * sx,
        0.,
        (xy - wz) * sy,
        (1. - (xx + zz)) * sy,
        (yz + wx) * sy,
        0.,
        (xz + wy) * sz,
        (yz - wx) * sz,
        (1. - (xx + yy)) * sz,
        0.,
        0.,
        0.,
        0.,
        1.,
    ]
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 50.,
            near: 1.,
            far: 100.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(-1., 1., 1.).normalize() * 10.;
        s.background = Color::from_hex(0xfce4ec);
        // HemisphereLight at its default position ( 0, 1, 0 ): the sky from +Y.
        let hemisphere = s.insert(NodeKind::Light(Light::Hemisphere {
            sky: Color::WHITE,
            ground: Color::from_hex(0xbfd4d2),
            intensity: 3.,
        }));
        s.get_mut(hemisphere)?.position = Vector3::Y;
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 0.3,
            target: Vector3::ZERO,
        }));
        {
            let n = s.get_mut(light)?;
            n.position = Vector3::new(1., 4., 3.) * 3.;
            n.cast_shadow = true;
            n.shadow.map_size = Some(2048);
            n.shadow.bias = -1e-4;
            n.shadow.normal_bias = 1e-4;
        }
        // The shadow plane.
        let plane = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(1., 1., 1, 1)?),
            Arc::new(Material::Shadow(ShadowMaterial {
                properties: MaterialProperties {
                    color: Color::from_hex(0xd81b60),
                    transparent: true,
                    opacity: 0.075,
                    side: Side::Double,
                    ..ShadowMaterial::default().properties
                },
            })),
        )));
        {
            let n = s.get_mut(plane)?;
            n.position.y = -3.;
            n.quaternion = Quaternion::from_rotation_x(-PI / 2.);
            n.scale = Vector3::splat(10.);
            n.receive_shadow = true;
        }
        let offset = Some((1., 1));
        let standard = |color: u32, flat: bool, emissive: Option<(u32, f64)>| {
            Arc::new(Material::Standard(MeshStandardMaterial {
                properties: MaterialProperties {
                    color: Color::from_hex(color),
                    flat_shading: flat,
                    polygon_offset: offset,
                    ..Default::default()
                },
                emissive: emissive.map_or(Color::BLACK, |(e, i)| Color(Color::from_hex(e).0 * i)),
                ..Default::default()
            }))
        };
        let materials = [
            standard(0xffffff, true, None),
            standard(0x80cbc4, false, None),
        ];
        let base = Brush::new(source(&IcosahedronGeometry::build(2., 3)?)?);
        let brush = Brush::new(source(&CylinderGeometry::build(
            1.,
            1.,
            5.,
            45,
            1,
            false,
            0.,
            2. * PI,
        )?)?);
        let core = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(IcosahedronGeometry::build(0.15, 1)?),
            standard(0xff9800, true, Some((0xff9800, 0.35))),
        )));
        s.get_mut(core)?.cast_shadow = true;
        let empty = Arc::new(BufferGeometry::default());
        let result = s.insert(NodeKind::Mesh(Mesh {
            geometry: empty.clone(),
            materials: materials.to_vec(),
        }));
        {
            let n = s.get_mut(result)?;
            n.cast_shadow = true;
            n.receive_shadow = true;
            n.visible = false;
        }
        let lines = s.insert(NodeKind::Mesh(Mesh::new(
            empty.clone(),
            Arc::new(Material::Basic(MeshBasicMaterial {
                properties: MaterialProperties {
                    color: Color::from_hex(0x009688),
                    wireframe: true,
                    ..Default::default()
                },
            })),
        )));
        s.get_mut(lines)?.visible = false;
        let mut controls = Controls::new(None, (5., 75.), PI, true);
        controls.update(s, c)?;
        Ok(Self {
            controls,
            time: 0.,
            base,
            brush,
            builder: Builder::default(),
            operation: csg_eval::SUBTRACTION,
            use_groups: true,
            wireframe: false,
            result,
            lines,
            materials,
            empty,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// animate(): the brushes' transforms from the clock, then updateCSG().
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        self.controls.update(s, c)?;
        let t = self.time * 1000. + 9000.;
        self.base.matrix = compose([t * 0.0001, t * 0.00025, t * 0.0005], [1., 1., 1.]);
        let scale = 0.5 + 0.5 * (1. + (t * 0.001).sin());
        self.brush.matrix = compose([t * -0.0002, t * -0.0005, t * -0.001], [scale, 1., scale]);
        let out = csg_eval::evaluate(
            &self.base,
            &self.brush,
            self.operation,
            self.use_groups,
            &mut self.builder,
        );
        // removeUnusedMaterials: the used materials in group order.
        let mut used: Vec<usize> = vec![];
        let mut geometry_groups = vec![];
        for &(start, count, material) in &out.groups {
            let index = used.iter().position(|&m| m == material).unwrap_or_else(|| {
                used.push(material);
                used.len() - 1
            });
            geometry_groups.push((start, count, index));
        }
        if !self.use_groups {
            used = vec![0];
        }
        // A new geometry with the previous identity: the renderer writes it
        // into the same resident buffers, as WebGL's bufferSubData does.
        let n = s.get_mut(self.result)?;
        let (position, quaternion, scale) = super::ldraw_loader::decompose(&self.base.matrix);
        (n.position, n.quaternion, n.scale) = (position, quaternion, scale);
        n.visible = true;
        let NodeKind::Mesh(m) = &mut n.kind else {
            return Err(Error::Invalid("csg result"));
        };
        m.materials = used.iter().map(|&i| self.materials[i].clone()).collect();
        let identity = if Arc::ptr_eq(&m.geometry, &self.empty) {
            crate::identity::Identity::default()
        } else {
            crate::identity::Identity {
                id: m.geometry.identity.id,
                uuid: m.geometry.identity.uuid,
            }
        };
        let mut g = BufferGeometry {
            identity,
            ..Default::default()
        };
        g.set_attribute(
            "position",
            Attribute::F32(BufferAttribute::new(out.positions, 3, false)?),
        );
        g.set_attribute(
            "normal",
            Attribute::F32(BufferAttribute::new(out.normals, 3, false)?),
        );
        g.set_attribute(
            "uv",
            Attribute::F32(BufferAttribute::new(out.uvs, 2, false)?),
        );
        let count = out.index.len();
        g.index = Some(out.index);
        for (start, count, material) in geometry_groups {
            g.add_group(start, count, material);
        }
        g.set_draw_range(0, Some(count));
        let geometry = Arc::new(g);
        m.geometry = geometry.clone();
        let n = s.get_mut(self.lines)?;
        n.visible = self.wireframe;
        if let NodeKind::Mesh(m) = &mut n.kind {
            m.geometry = geometry;
        }
        Ok(())
    }
    pub fn draw(&mut self, _kind: u32, _x: f64, _y: f64) {}
    pub fn key(&mut self, _code: u32, _down: bool) {}
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
        let camera = camera_state(s, c)?;
        if wheel != 0. {
            self.controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if pan {
            self.controls.pan(&camera, dx, dy, height);
        } else {
            self.controls.rotate(dx, dy, height);
        }
        self.controls.update(s, c)
    }
    /// operation ( SUBTRACTION, INTERSECTION, ADDITION ), wireframe, useGroups.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        match index {
            0 => {
                self.operation = *OPERATIONS
                    .get(value as usize)
                    .ok_or(Error::Invalid("csg operation"))?
            }
            1 => self.wireframe = value != 0.,
            2 => self.use_groups = value != 0.,
            _ => return Err(Error::Invalid("csg parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
