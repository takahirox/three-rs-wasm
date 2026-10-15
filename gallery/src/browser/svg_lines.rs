//! svg_lines: a 50-segment circle drawn by three LineBasicMaterial lines
//! ( width 10, random colors, scale 1/3, 2/3, 1 ) and a dashed blue one
//! ( scale 2 ), turned with the scene by the clock and written as SVG paths
//! by SVGRenderer ( Rust, src/svg.rs ) without color management.
use super::css3d_common::{Random, svg_overlay, window_size};
use crate::svg::SVGRenderer;
use crate::{
    Error, Result, attribute::BufferAttribute, camera::*, material::*, math::*, renderer::*,
    scene::*,
};
use std::f64::consts::PI;
use std::sync::Arc;

pub(super) struct Demo {
    svg: SVGRenderer,
    objects: Scene,
    /// The page's Scene and its lines, in traverse() order.
    nodes: Vec<Object3D>,
    time: f64,
    size: (f64, f64),
}
/// `color.setHex( hex )` without color management: the bytes over 255.
pub(super) fn raw_hex(hex: f64) -> Color {
    let hex = hex.floor() as u32;
    Color::linear(
        f64::from((hex >> 16) & 255) / 255.,
        f64::from((hex >> 8) & 255) / 255.,
        f64::from(hex & 255) / 255.,
    )
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 33.,
            near: 0.1,
            far: 100.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 0., 10.);
        let mut objects = Scene::new();
        objects.background = Color::linear(0., 0., 0.);
        let scene = objects.insert(NodeKind::Group);
        let svg = svg_overlay("svg-renderer", false)?;
        let mut vertices = vec![];
        let divisions = 50;
        for i in 0..=divisions {
            let v = (f64::from(i) / f64::from(divisions)) * (PI * 2.);
            vertices.extend([v.sin() as f32, 0., v.cos() as f32]);
        }
        let mut geometry = crate::geometry::BufferGeometry::default();
        geometry.set_attribute(
            "position",
            crate::geometry::Attribute::F32(BufferAttribute::new(vertices, 3, false)?),
        );
        let geometry = Arc::new(geometry);
        let mut random = Random(186);
        let mut nodes = vec![scene];
        let mut add =
            |objects: &mut Scene, material: LineBasicMaterial, scale: f64| -> Result<()> {
                let line = objects.insert(NodeKind::Line(Line {
                    geometry: geometry.clone(),
                    material: Arc::new(Material::Line(material)),
                    segments: false,
                }));
                objects.get_mut(line)?.scale = Vector3::splat(scale);
                objects.add(scene, line)?;
                nodes.push(line);
                Ok(())
            };
        for i in 1..=3 {
            let mut material = LineBasicMaterial {
                linewidth: 10.,
                ..Default::default()
            };
            material.properties.color = raw_hex(random.next() * f64::from(0xffffff));
            add(&mut objects, material, f64::from(i) / 3.)?;
        }
        let mut material = LineBasicMaterial {
            linewidth: 1.,
            dash: Some(LineDash {
                size: 10.,
                gap: 10.,
                ..Default::default()
            }),
            ..Default::default()
        };
        material.properties.color = raw_hex(f64::from(0x0000ff));
        add(&mut objects, material, 2.)?;
        Ok(Self {
            svg,
            objects,
            nodes,
            time: 0.,
            size: window_size(),
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// animate(): every object of scene.traverse() turned by its count and
    /// the clock, then render().
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        let size = window_size();
        if size != self.size {
            self.size = size;
            self.svg.set_size(size.0, size.1);
        }
        let time = self.time;
        for (count, &h) in self.nodes.iter().enumerate() {
            let count = count as f64;
            self.objects.get_mut(h)?.quaternion = Euler {
                angles: Vector3::new(count + time / 3., 0., count + time / 4.),
                order: EulerOrder::XYZ,
            }
            .quaternion();
        }
        s.update()?;
        self.svg.render_from(&mut self.objects, s, c)
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
        Err(Error::Invalid("svg_lines parameter"))
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
