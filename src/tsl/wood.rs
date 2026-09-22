//! Procedural wood color graph from r186 WoodNodeMaterial (Three.js MIT).
use super::materialx::{self, Dimension};
use super::*;
pub struct WoodNodes {
    pub center_size: Node,
    pub large_warp_scale: Node,
    pub large_grain_stretch: Node,
    pub small_warp_strength: Node,
    pub small_warp_scale: Node,
    pub fine_warp_strength: Node,
    pub fine_warp_scale: Node,
    pub ring_thickness: Node,
    pub ring_bias: Node,
    pub ring_size_variance: Node,
    pub ring_variance_scale: Node,
    pub bark_thickness: Node,
    pub splotch_scale: Node,
    pub splotch_intensity: Node,
    pub cell_scale: Node,
    pub cell_size: Node,
    pub dark: Node,
    pub light: Node,
}
fn saturate(n: Node) -> Node {
    n.clamp(float(0.), float(1.))
}
fn normalized(p: Node) -> Node {
    materialx::perlin(p, Dimension::D3) * float(0.5) + float(0.5)
}
fn warp(p: Node, strength: Node, xy: Node, z: Node) -> Node {
    let noise = materialx::perlin_vec3(
        p.clone() * vec3(xy.clone(), xy, z) * float(2.4),
        Dimension::D3,
    ) * float(0.5)
        * strength;
    let xy = p * vec3(float(1.), float(1.), float(0.));
    noise * xy.clone().normalize() + xy
}
fn soft(t: Node, a: Node, b: Node) -> Node {
    let screen = float(1.) - (float(1.) - b.clone()) * (float(1.) - a.clone());
    (float(1.) - t.clone()) * a.clone() + t * ((float(1.) - a.clone()) * b * a.clone() + a * screen)
}
impl WoodNodes {
    pub fn color(&self, p: Node, view_distance: Node) -> Node {
        let center = saturate(p.clone().swizzle("xy").length()) * self.center_size.clone();
        let main = warp(
            warp(
                p.clone(),
                center,
                self.large_warp_scale.clone(),
                self.large_grain_stretch.clone(),
            ),
            self.small_warp_strength.clone(),
            self.small_warp_scale.clone(),
            float(0.17),
        );
        let detail = warp(
            main.clone(),
            self.fine_warp_strength.clone(),
            self.fine_warp_scale.clone(),
            float(0.17),
        );
        let length = detail.clone().length();
        let coord = length.clone() * self.ring_variance_scale.clone();
        let noise =
            materialx::perlin(vec2(coord.clone(), coord), Dimension::D2) * float(0.5) + float(0.5);
        let rings = ((noise * self.ring_size_variance.clone() + length.clone())
            / self.ring_thickness.clone())
        .fract()
            * self.bark_thickness.clone();
        let a = saturate(rings.clone() / self.ring_bias.clone());
        // Pinned r186 mapRange clamps max(min(value,toMax),toMin).
        // Its descending [1,0] branch therefore returns 1, not a descending ramp.
        let sharp = a;
        let blur = (view_distance.clone() / float(10.)).max(float(1.));
        let rings = (sharp - float(0.5)).smoothstep(-blur.clone(), blur) * float(0.5) + float(0.5);
        let angle = WgslFn::new(
            "wood_angle",
            "fn wood_angle(y:f32,x:f32)->f32{return atan2(y,x);}",
            &[Type::Float, Type::Float],
            Type::Float,
        )
        .unwrap()
        .call(&[detail.clone().y(), detail.x()]);
        let radial = saturate(angle / float(std::f32::consts::TAU) + float(0.5))
            * float(std::f32::consts::TAU * 3.);
        let noise = normalized(
            vec3(radial.clone().sin(), length, radial.cos() * p.swizzle("z"))
                * vec3(float(0.1), float(1.19), float(0.05))
                * self.splotch_scale.clone(),
        );
        let cell = warp(
            main * (self.cell_scale.clone() / float(50.)),
            self.cell_scale.clone() / float(1000.),
            float(0.1),
            float(1.77),
        );
        let cells = WgslFn::new(
            "wood_voronoi",
            include_str!("wood.wgsl"),
            &[Type::Vec3],
            Type::Float,
        )
        .unwrap()
        .call(&[vec3(
            cell.clone().x() * float(75.),
            cell.y() * float(75.),
            float(0.),
        )]);
        let size = self.cell_size.clone() / (view_distance * float(10.)).max(float(1.));
        let cells = saturate((cells - size) / float(0.21));
        soft(
            self.splotch_intensity.clone(),
            soft(
                float(0.407),
                mix(self.dark.clone(), self.light.clone(), rings),
                splat(cells, Type::Vec3),
            ),
            splat(noise, Type::Vec3),
        )
    }
}
