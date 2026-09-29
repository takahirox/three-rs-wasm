//! webgpu_clipping_stencil: a torus knot clipped by three planes, capped
//! where the stencil bit of each plane marks the exposed interior, over a
//! ShadowNodeMaterial ground receiving the clipped SunLight shadow.
use super::controls_attributes::{CameraState, Controls, camera_state};
use crate::attribute::BufferAttribute;
use crate::shader::ShaderProgram;
use crate::tsl::{NodeMaterial, Type, WgslFn};
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::f64::consts::PI;
use std::sync::Arc;

/// ShadowNodeMaterial's ShadowMaskModel: the product of the lights' shadow
/// factors masks the color (black) as alpha × ( 1 − mask ); u.custom[0].x is
/// the opacity.
const SHADOW_MASK: &str = "fn shadow_mask()->vec4<f32>{var mask=1.0;for(var i=0u;i<u32(u.material.w);i++){mask*=shadow_visibility(i,fragment_surface.position,normalize(fragment_surface.normal));}return vec4(0.0,0.0,0.0,u.custom[0].x*(1.0-mask));}";
fn stencil(
    compare: wgpu::CompareFunction,
    op: wgpu::StencilOperation,
    read: u32,
    write: u32,
) -> wgpu::StencilState {
    let face = wgpu::StencilFaceState {
        compare,
        fail_op: op,
        depth_fail_op: op,
        pass_op: op,
    };
    wgpu::StencilState {
        front: face,
        back: face,
        read_mask: read,
        write_mask: write,
    }
}
pub(super) struct Demo {
    time: f64,
    last: f64,
    object: Object3D,
    rotation: Vector2,
    planes: [Plane; 3],
    /// The stencil group meshes, the caps, the clipped knot and the helpers.
    stencil_meshes: [Object3D; 3],
    caps: [Object3D; 3],
    knot: Object3D,
    helpers: [Object3D; 3],
    /// animate, then displayHelper, constant and negated per plane.
    params: [f64; 10],
    /// The planes changed since they were last applied.
    dirty: bool,
    controls: Controls,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 36.,
            near: 1.,
            far: 100.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(2., 2., 2.);
        s.background = Color::from_hex(0x263238);
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::WHITE,
            intensity: 1.5,
        }));
        let sun = s.insert(NodeKind::Light(Light::Sun {
            color: Color::WHITE,
            intensity: 3.,
        }));
        let n = s.get_mut(sun)?;
        n.position = Vector3::new(5., 10., 7.5);
        n.cast_shadow = true;
        n.shadow = crate::shadow::Shadow {
            map_size: Some(2048),
            far: 15.,
            ..Default::default()
        };
        let planes = [Vector3::NEG_X, Vector3::NEG_Y, Vector3::NEG_Z].map(|normal| Plane {
            normal,
            constant: 0.,
        });
        // PlaneHelper( plane, 2, 0xffffff ): the outline strip and a 0.2-opacity plane.
        let mut helpers = vec![];
        let outline = Arc::new({
            let mut g = BufferGeometry::default();
            g.set_attribute(
                "position",
                Attribute::F32(BufferAttribute::new(
                    vec![
                        1., -1., 0., -1., 1., 0., -1., -1., 0., 1., 1., 0., -1., 1., 0., -1., -1.,
                        0., 1., -1., 0., 1., 1., 0.,
                    ],
                    3,
                    false,
                )?),
            );
            g
        });
        let square = Arc::new({
            let mut g = BufferGeometry::default();
            g.set_attribute(
                "position",
                Attribute::F32(BufferAttribute::new(
                    vec![
                        1., 1., 0., -1., 1., 0., -1., -1., 0., 1., 1., 0., -1., -1., 0., 1., -1.,
                        0.,
                    ],
                    3,
                    false,
                )?),
            );
            g
        });
        for _ in 0..3 {
            let mut line = LineBasicMaterial::default();
            line.properties.tone_mapped = false;
            let h = s.insert(NodeKind::Line(Line {
                geometry: outline.clone(),
                material: Arc::new(Material::Line(line)),
                segments: false,
            }));
            let mut m = MeshBasicMaterial::default();
            m.properties.opacity = 0.2;
            m.properties.transparent = true;
            m.properties.depth_write = false;
            m.properties.tone_mapped = false;
            let plane = s.insert(NodeKind::Mesh(Mesh::new(
                square.clone(),
                Arc::new(Material::Basic(m)),
            )));
            s.add(h, plane)?;
            s.get_mut(h)?.visible = false;
            helpers.push(h);
        }
        let geometry = Arc::new(TorusKnotGeometry::build(0.4, 0.15, 220, 60, 2, 3)?);
        let object = s.insert(NodeKind::Group);
        // renderOrder 1, 2, 3 for the stencil groups, 1.1, 2.1, 3.1 for the
        // caps and 6 for the knot, kept in that order as tenths.
        let mut stencil_meshes = vec![];
        let mut caps = vec![];
        let cap_geometry = Arc::new(PlaneGeometry::build(4., 4., 1, 1)?);
        for (i, plane) in planes.iter().enumerate() {
            let bit = 1u32 << i;
            // The ClippingGroup's MeshBasicNodeMaterial: back and front faces
            // invert the plane's stencil bit, without color or depth.
            let mut m = MeshBasicMaterial::default();
            m.properties.side = Side::Double;
            m.properties.depth_write = false;
            m.properties.depth_test = false;
            m.properties.color_write = false;
            m.properties.stencil = Some(stencil(
                wgpu::CompareFunction::Always,
                wgpu::StencilOperation::Invert,
                0xff,
                bit,
            ));
            m.properties.clipping_planes = vec![*plane];
            let h = s.insert(NodeKind::Mesh(Mesh::new(
                geometry.clone(),
                Arc::new(Material::Basic(m)),
            )));
            s.get_mut(h)?.render_order = 10 * (i as i32 + 1);
            s.add(object, h)?;
            stencil_meshes.push(h);
            // The cap: drawn where the bit is set, clearing it, and clipped by
            // the other two planes.
            let mut m = MeshStandardMaterial {
                energy_conservation: true,
                metalness: 0.1,
                roughness: 0.75,
                ..Default::default()
            };
            m.properties.color = Color::from_hex(0xe91e63);
            m.properties.stencil = Some(stencil(
                wgpu::CompareFunction::NotEqual,
                wgpu::StencilOperation::Replace,
                bit,
                bit,
            ));
            m.properties.stencil_reference = 0;
            m.properties.clipping_planes = planes
                .iter()
                .enumerate()
                .filter(|&(j, _)| j != i)
                .map(|(_, p)| *p)
                .collect();
            let h = s.insert(NodeKind::Mesh(Mesh::new(
                cap_geometry.clone(),
                Arc::new(Material::Standard(m)),
            )));
            s.get_mut(h)?.render_order = 10 * (i as i32 + 1) + 1;
            caps.push(h);
        }
        let mut m = MeshStandardMaterial {
            energy_conservation: true,
            metalness: 0.1,
            roughness: 0.75,
            ..Default::default()
        };
        m.properties.color = Color::from_hex(0xffc107);
        m.properties.shadow_side = Some(Side::Double);
        m.properties.clipping_planes = planes.to_vec();
        m.properties.clip_shadows = true;
        let knot = s.insert(NodeKind::Mesh(Mesh::new(
            geometry.clone(),
            Arc::new(Material::Standard(m)),
        )));
        let n = s.get_mut(knot)?;
        n.cast_shadow = true;
        n.render_order = 60;
        s.add(object, knot)?;
        // The ShadowNodeMaterial ground: black, opacity 0.25, double sided.
        let node = WgslFn::new("shadow_mask", SHADOW_MASK, &[], Type::Vec4)?.call(&[]);
        let mut m = ShaderMaterial::new(Arc::new(
            ShaderProgram::with_projection_and_dimensions(
                r,
                &NodeMaterial::new(node).wgsl_with_texture_types(&[], &[])?,
                &[],
                &[],
                "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}",
            )
            .await?,
        ));
        m.uniforms[0] = [0.25, 0., 0., 0.];
        m.properties.transparent = true;
        m.properties.side = Side::Double;
        // A transparent double-sided material draws back faces, then front faces.
        m.properties.force_single_pass = false;
        let ground = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(9., 9., 1, 1)?),
            Arc::new(Material::Shader(m)),
        )));
        let n = s.get_mut(ground)?;
        n.quaternion = Quaternion::from_rotation_x(-PI / 2.);
        n.position.y = -1.;
        n.receive_shadow = true;
        // OrbitControls: distances 2–20.
        let mut controls = Controls::new(None, (2., 20.), PI, true);
        controls.update(s, c)?;
        Ok(Self {
            time: 0.,
            last: 0.,
            object,
            rotation: Vector2::ZERO,
            planes,
            stencil_meshes: stencil_meshes
                .try_into()
                .map_err(|_| Error::Invalid("stencil meshes"))?,
            caps: caps.try_into().map_err(|_| Error::Invalid("caps"))?,
            knot,
            helpers: helpers.try_into().map_err(|_| Error::Invalid("helpers"))?,
            params: [1., 0., 0., 0., 0., 0., 0., 0., 0., 0.],
            dirty: true,
            controls,
        })
    }
    /// The planes into each material, the caps onto their planes and the
    /// helpers' PlaneHelper.updateMatrixWorld.
    fn apply_planes(&mut self, s: &mut Scene) -> Result<()> {
        let planes = self.planes;
        for (i, plane) in planes.iter().enumerate() {
            if let NodeKind::Mesh(m) = &mut s.get_mut(self.stencil_meshes[i])?.kind {
                Arc::make_mut(&mut m.materials[0])
                    .properties_mut()
                    .clipping_planes = vec![*plane];
            }
            let cap = s.get_mut(self.caps[i])?;
            if let NodeKind::Mesh(m) = &mut cap.kind {
                Arc::make_mut(&mut m.materials[0])
                    .properties_mut()
                    .clipping_planes = planes
                    .iter()
                    .enumerate()
                    .filter(|&(j, _)| j != i)
                    .map(|(_, p)| *p)
                    .collect();
            }
            // plane.coplanarPoint( po.position ); po.lookAt( position − normal ).
            let position = plane.normal * -plane.constant;
            cap.position = position;
            s.look_at(self.caps[i], position - plane.normal)?;
            let helper = s.get_mut(self.helpers[i])?;
            helper.position = Vector3::ZERO;
            helper.scale = Vector3::new(1., 1., 1.);
            s.look_at(self.helpers[i], plane.normal)?;
            let helper = s.get_mut(self.helpers[i])?;
            helper.position = helper.quaternion * Vector3::new(0., 0., -plane.constant);
        }
        if let NodeKind::Mesh(m) = &mut s.get_mut(self.knot)?.kind {
            Arc::make_mut(&mut m.materials[0])
                .properties_mut()
                .clipping_planes = planes.to_vec();
        }
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// animate(): the group turns by the timer's delta (× 0.5, × 0.2) while
    /// animate is on; the caps follow their planes.
    pub fn prepare(&mut self, s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        let delta = self.time - self.last;
        self.last = self.time;
        if self.params[0] > 0.5 {
            self.rotation += Vector2::new(delta * 0.5, delta * 0.2);
        }
        s.get_mut(self.object)?.quaternion =
            Quaternion::from_euler(glam::EulerRot::XYZ, self.rotation.x, self.rotation.y, 0.);
        for (i, &helper) in self.helpers.iter().enumerate() {
            s.get_mut(helper)?.visible = self.params[1 + 3 * i] > 0.5;
        }
        if std::mem::take(&mut self.dirty) {
            self.apply_planes(s)?;
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
        let camera: CameraState = camera_state(s, c)?;
        if wheel != 0. {
            self.controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if pan {
            self.controls.pan(&camera, dx, dy, height);
        } else {
            self.controls.rotate(dx, dy, height);
        }
        self.controls.update(s, c)
    }
    /// The GUI: animate; per plane displayHelper, constant and negated
    /// (Plane.negate on every change).
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        if index >= self.params.len() {
            return Err(Error::Invalid("clipping parameter"));
        }
        self.params[index] = value as f64;
        if index > 0 && !(index - 1).is_multiple_of(3) {
            self.dirty = true;
            let plane = &mut self.planes[(index - 1) / 3];
            match (index - 1) % 3 {
                1 => plane.constant = value as f64,
                2 => {
                    plane.normal = -plane.normal;
                    plane.constant = -plane.constant;
                }
                _ => {}
            }
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
