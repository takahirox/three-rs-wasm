//! webgl_loader_ifc: the Revit advanced sample project as web-ifc streams it
//! ( baked by tools/tsl/prepare-ifc-loader.mjs: each distinct geometry's
//! vertex data and index, and every placement's geometry, colour and flat
//! transformation in stream order ), merged at load as the page's
//! loadAllGeometry merges it: the sRGB colour converted once per placement
//! into RGBA vertex colours, applyMatrix4 on positions and normals, and
//! mergeGeometries into one opaque and one transparent double-sided vertex-
//! coloured Phong mesh. Two directional lights and an ambient light; OrbitControls
//! render on demand.
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::fetch;
use crate::attribute::BufferAttribute;
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::f64::consts::PI;
use std::sync::Arc;

const ASSETS: &str = "/web/gallery/assets/ifc-loader";

type Json = serde_json::Value;
fn bytes<'a>(bin: &'a [u8], v: &Json, size: usize) -> Result<&'a [u8]> {
    let offset = v["offset"].as_u64().ok_or(Error::Invalid("ifc offset"))? as usize;
    let length = v["length"].as_u64().ok_or(Error::Invalid("ifc length"))? as usize;
    bin.get(offset..offset + length * size)
        .ok_or(Error::Invalid("ifc range"))
}
/// Color.setRGB( …, SRGBColorSpace ): SRGBToLinear.
fn srgb_to_linear(c: f64) -> f64 {
    if c < 0.04045 {
        c * 0.0773993808
    } else {
        (c * 0.9478672986 + 0.0521327014).powf(2.4)
    }
}

/// One merged mesh's attributes and index, as mergeGeometries concatenates them.
#[derive(Default)]
struct Merged {
    position: Vec<f32>,
    normal: Vec<f32>,
    color: Vec<f32>,
    index: Vec<u32>,
}
impl Merged {
    fn geometry(self) -> Result<BufferGeometry> {
        let mut g = BufferGeometry::default();
        g.set_attribute(
            "position",
            Attribute::F32(BufferAttribute::new(self.position, 3, false)?),
        );
        g.set_attribute(
            "normal",
            Attribute::F32(BufferAttribute::new(self.normal, 3, false)?),
        );
        g.set_attribute(
            "color",
            Attribute::F32(BufferAttribute::new(self.color, 4, false)?),
        );
        g.set_index(Some(self.index));
        Ok(g)
    }
}

pub struct Demo {
    controls: Controls,
}

impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.,
            near: 0.1,
            far: 1000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(82.48, 22.09, -45.24);
        s.background = Color::from_hex(0x8cc7de);
        for (color, position) in [
            (0xffeeff, Vector3::new(1., 1., 1.)),
            (0xffffff, Vector3::new(-1., 0.5, -1.)),
        ] {
            let h = s.insert(NodeKind::Light(Light::Directional {
                color: Color::from_hex(color),
                intensity: 2.5,
                target: Vector3::ZERO,
            }));
            s.get_mut(h)?.position = position;
        }
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0xffffee),
            intensity: 0.75,
        }));
        let json: Json = serde_json::from_slice(
            &fetch(&format!("{ASSETS}/rac_advanced_sample_project.json")).await?,
        )
        .map_err(|e| Error::Asset(e.to_string()))?;
        let bin = fetch(&format!("{ASSETS}/rac_advanced_sample_project.bin")).await?;
        let geometries = json["geometries"]
            .as_array()
            .ok_or(Error::Invalid("ifc geometries"))?
            .iter()
            .map(|g| {
                let vertex: Vec<f32> = bytes(&bin, &g["vertex"], 4)?
                    .as_chunks::<4>()
                    .0
                    .iter()
                    .map(|&c| f32::from_le_bytes(c))
                    .collect();
                let index: Vec<u32> = bytes(&bin, &g["index"], 4)?
                    .as_chunks::<4>()
                    .0
                    .iter()
                    .map(|&c| u32::from_le_bytes(c))
                    .collect();
                Ok((vertex, index))
            })
            .collect::<Result<Vec<_>>>()?;
        let placements: Vec<f64> = bytes(&bin, &json["placements"], 8)?
            .as_chunks::<8>()
            .0
            .iter()
            .map(|&c| f64::from_le_bytes(c))
            .collect();
        let stride = json["placementStride"].as_u64().unwrap_or(21) as usize;
        let mut opaque = Merged::default();
        let mut transparent = Merged::default();
        for p in placements.chunks_exact(stride) {
            let (vertex, index) = geometries
                .get(p[0] as usize)
                .ok_or(Error::Invalid("ifc placement geometry"))?;
            let alpha = p[4];
            let target = if alpha != 1. {
                &mut transparent
            } else {
                &mut opaque
            };
            let color = [
                srgb_to_linear(p[1]) as f32,
                srgb_to_linear(p[2]) as f32,
                srgb_to_linear(p[3]) as f32,
                alpha as f32,
            ];
            let e: [f64; 16] = std::array::from_fn(|i| p[5 + i]);
            // getNormalMatrix: the inverse transpose of the upper 3 × 3.
            let normal_matrix = Matrix3::from_cols(
                Vector3::new(e[0], e[1], e[2]),
                Vector3::new(e[4], e[5], e[6]),
                Vector3::new(e[8], e[9], e[10]),
            )
            .inverse()
            .transpose();
            let base = (target.position.len() / 3) as u32;
            for v in vertex.as_chunks::<6>().0 {
                let (x, y, z) = (f64::from(v[0]), f64::from(v[1]), f64::from(v[2]));
                let w = 1. / (e[3] * x + e[7] * y + e[11] * z + e[15]);
                target.position.extend([
                    ((e[0] * x + e[4] * y + e[8] * z + e[12]) * w) as f32,
                    ((e[1] * x + e[5] * y + e[9] * z + e[13]) * w) as f32,
                    ((e[2] * x + e[6] * y + e[10] * z + e[14]) * w) as f32,
                ]);
                let n =
                    normal_matrix * Vector3::new(f64::from(v[3]), f64::from(v[4]), f64::from(v[5]));
                let length = n.length();
                let n = if length > 0. { n / length } else { n };
                target.normal.extend([n.x as f32, n.y as f32, n.z as f32]);
                target.color.extend(color);
            }
            target.index.extend(index.iter().map(|i| i + base));
        }
        for (merged, is_transparent) in [(opaque, false), (transparent, true)] {
            if merged.index.is_empty() {
                continue;
            }
            let mut material = MeshPhongMaterial::default();
            material.properties.side = Side::Double;
            material.properties.vertex_colors = true;
            material.properties.transparent = is_transparent;
            s.insert(NodeKind::Mesh(Mesh::new(
                Arc::new(merged.geometry()?),
                Arc::new(Material::Phong(material)),
            )));
        }
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.set_target(Vector3::new(30.86, 7.73, 0.15));
        controls.update(s, c)?;
        Ok(Self { controls })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, _animate: bool) -> Result<()> {
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        self.controls.update(s, c)
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
        Ok(())
    }
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {}
}
