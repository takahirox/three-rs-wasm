//! RapierPhysics.js ( `examples/jsm/physics` ) on the engine of rapier.js
//! 0.21.0: `@dimforge/rapier3d-compat` is rapier3d 0.36.0 built to wasm, and
//! this module drives the same crate. Bodies, colliders and steps go through
//! the builders and calls of rapier.js's bindings, with ColliderDesc's and
//! RigidBodyDesc's defaults, so a world given the same inputs takes the same
//! steps.
//!
//! rapier3d 0.36 keys its contact maps with foldhash's fixed seed, so the
//! order of contacts, and the trajectories, follow rapier.js's.
use crate::math::{Quaternion, Vector3};
use crate::scene::{NodeKind, Object3D, Scene};
use crate::{Error, Result};
use rapier3d::prelude::*;

/// RapierPhysics.js's getShape( geometry ): the collider for a geometry's
/// parameters.
#[derive(Clone, Debug)]
pub enum PhysicsShape {
    /// RoundedBoxGeometry( width, height, depth, segments, radius ).
    RoundedBox {
        width: f64,
        height: f64,
        depth: f64,
        radius: f64,
    },
    Box {
        width: f64,
        height: f64,
        depth: f64,
    },
    /// SphereGeometry or IcosahedronGeometry.
    Ball {
        radius: f64,
    },
    /// CylinderGeometry: radiusBottom and height.
    Cylinder {
        radius: f64,
        height: f64,
    },
    Capsule {
        radius: f64,
        height: f64,
    },
    /// A BufferGeometry's positions and indices ( 0, 1, 2, … when it has none ).
    TriMesh {
        vertices: Vec<f32>,
        indices: Vec<u32>,
    },
}
impl PhysicsShape {
    /// ColliderDesc.roundCuboid / cuboid / ball / cylinder / capsule / trimesh.
    pub fn collider(&self) -> Option<SharedShape> {
        Some(match self {
            Self::RoundedBox {
                width,
                height,
                depth,
                radius,
            } => SharedShape::round_cuboid(
                (width / 2. - radius) as f32,
                (height / 2. - radius) as f32,
                (depth / 2. - radius) as f32,
                *radius as f32,
            ),
            Self::Box {
                width,
                height,
                depth,
            } => SharedShape::cuboid(
                (width / 2.) as f32,
                (height / 2.) as f32,
                (depth / 2.) as f32,
            ),
            Self::Ball { radius } => SharedShape::ball(*radius as f32),
            Self::Cylinder { radius, height } => {
                SharedShape::cylinder((height / 2.) as f32, *radius as f32)
            }
            // RawShape.capsule: the segment along y.
            Self::Capsule { radius, height } => {
                let p2 = Vector::Y * (height / 2.) as f32;
                SharedShape::capsule(-p2, p2, *radius as f32)
            }
            Self::TriMesh { vertices, indices } => SharedShape::trimesh_with_flags(
                vertices.chunks(3).map(Vector::from_slice).collect(),
                indices.chunks(3).map(|v| [v[0], v[1], v[2]]).collect(),
                TriMeshFlags::empty(),
            )
            .ok()?,
        })
    }
}

/// A rigid body of the world: one per mesh, or one per instance.
#[derive(Clone, Debug)]
pub enum PhysicsBody {
    Single(RigidBodyHandle),
    Instanced(Vec<RigidBodyHandle>),
}

/// The world, its pipelines and the meshes it moves.
pub struct RapierPhysics {
    pub gravity: Vector,
    pub integration_parameters: IntegrationParameters,
    pub pipeline: PhysicsPipeline,
    pub islands: IslandManager,
    pub broad_phase: DefaultBroadPhase,
    pub narrow_phase: NarrowPhase,
    pub bodies: RigidBodySet,
    pub colliders: ColliderSet,
    /// World.softBodies: rapier.js's World passes its ( here empty ) set to
    /// each step.
    pub soft_bodies: SoftBodySet,
    pub impulse_joints: ImpulseJointSet,
    pub multibody_joints: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    pub debug_render: DebugRenderPipeline,
    /// The meshes with mass, in the order they were added.
    meshes: Vec<(Object3D, PhysicsBody)>,
}
impl Default for RapierPhysics {
    fn default() -> Self {
        Self::new()
    }
}

/// ColliderDesc's defaults: density 1, friction 0.5, the average combine
/// rules, every group ( 0xffffffff unpacked into 16-bit halves ), no events.
fn collider_builder(shape: SharedShape) -> ColliderBuilder {
    let groups = InteractionGroups::new(
        Group::from_bits_retain(0xffff),
        Group::from_bits_retain(0xffff),
        InteractionTestMode::And,
    );
    ColliderBuilder::new(shape)
        .enabled(true)
        .position(Pose::IDENTITY)
        .friction(0.5)
        .restitution(0.)
        .collision_groups(groups)
        .solver_groups(groups)
        .active_hooks(ActiveHooks::empty())
        .active_events(ActiveEvents::empty())
        .active_collision_types(ActiveCollisionTypes::default())
        .sensor(false)
        .friction_combine_rule(CoefficientCombineRule::Average)
        .restitution_combine_rule(CoefficientCombineRule::Average)
        .contact_force_event_threshold(0.)
        .contact_skin(0.)
}

/// RigidBodyDesc's defaults, as RawRigidBodySet.createRigidBody builds them.
pub fn rigid_body_builder(
    body_type: RigidBodyType,
    position: Vector3,
    rotation: Quaternion,
) -> RigidBodyBuilder {
    // RawRotation.new: Rotation::from_xyzw, without normalizing.
    let rotation = Rotation::from_xyzw(
        rotation.x as f32,
        rotation.y as f32,
        rotation.z as f32,
        rotation.w as f32,
    );
    RigidBodyBuilder::new(body_type)
        .enabled(true)
        .pose(Pose::from_parts(vector(position), rotation))
        .gravity_scale(1.)
        .enabled_translations(true, true, true)
        .enabled_rotations(true, true, true)
        .linvel(Vector::ZERO)
        .angvel(Vector::ZERO)
        .linear_damping(0.)
        .angular_damping(0.)
        .can_sleep(true)
        .sleeping(false)
        .ccd_enabled(false)
        .dominance_group(0)
        .additional_solver_iterations(0)
        .additional_pgs_iterations(0)
        .soft_ccd_prediction(0.)
        .additional_mass_properties(MassProperties::with_principal_inertia_frame(
            Vector::ZERO,
            0.,
            Vector::ZERO,
            Rotation::IDENTITY,
        ))
}

/// A body's translation and rotation in the scene's math types.
pub fn body_pose(body: &RigidBody) -> (Vector3, Quaternion) {
    let t = body.translation();
    let r = body.rotation();
    (
        Vector3::new(f64::from(t.x), f64::from(t.y), f64::from(t.z)),
        Quaternion::from_xyzw(
            f64::from(r.x),
            f64::from(r.y),
            f64::from(r.z),
            f64::from(r.w),
        ),
    )
}

fn vector(v: Vector3) -> Vector {
    Vector::new(v.x as f32, v.y as f32, v.z as f32)
}

impl RapierPhysics {
    /// `new RAPIER.World( { x: 0, y: -9.81, z: 0 } )`.
    pub fn new() -> Self {
        Self {
            gravity: Vector::new(0., -9.81, 0.),
            integration_parameters: IntegrationParameters::default(),
            pipeline: PhysicsPipeline::new(),
            islands: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            soft_bodies: SoftBodySet::new(),
            impulse_joints: ImpulseJointSet::new(),
            multibody_joints: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            debug_render: DebugRenderPipeline::default(),
            meshes: vec![],
        }
    }
    /// `world.createRigidBody( desc )` then `world.createCollider( shape, body )`.
    pub fn create_body(
        &mut self,
        builder: RigidBodyBuilder,
        collider: ColliderBuilder,
    ) -> (RigidBodyHandle, ColliderHandle) {
        let body = self.bodies.insert(builder.build());
        let collider = self
            .colliders
            .insert_with_parent(collider.build(), body, &mut self.bodies);
        (body, collider)
    }
    /// `world.createCollider( desc )` without a parent body.
    pub fn insert_collider(&mut self, collider: ColliderBuilder) -> ColliderHandle {
        self.colliders.insert(collider.build())
    }
    /// ColliderDesc for a shape with setMass( mass ) and setRestitution().
    pub fn collider_desc(shape: SharedShape, mass: f64, restitution: f64) -> ColliderBuilder {
        collider_builder(shape)
            .restitution(restitution as f32)
            .mass(mass as f32)
    }
    /// The bare ColliderDesc of a shape ( density 1 ).
    pub fn default_collider(shape: SharedShape) -> ColliderBuilder {
        collider_builder(shape).density(1.)
    }
    /// addMesh( mesh, mass, restitution ): a fixed body without mass, else
    /// dynamic; one body per instance of an instanced mesh, at its
    /// translation. Meshes with mass follow their bodies on each step.
    pub fn add_mesh(
        &mut self,
        s: &Scene,
        mesh: Object3D,
        shape: &PhysicsShape,
        mass: f64,
        restitution: f64,
    ) -> Result<Option<PhysicsBody>> {
        let Some(shape) = shape.collider() else {
            return Ok(None);
        };
        let body_type = if mass > 0. {
            RigidBodyType::Dynamic
        } else {
            RigidBodyType::Fixed
        };
        let node = s.get(mesh)?;
        let body = if matches!(node.kind, NodeKind::Mesh(_)) && !node.instances.is_empty() {
            let count = node
                .instance_count
                .map_or(node.instances.len(), |n| n as usize);
            let translations: Vec<Vector3> = node.instances[..count]
                .iter()
                .map(|i| i.matrix.w_axis.truncate())
                .collect();
            PhysicsBody::Instanced(
                translations
                    .into_iter()
                    .map(|p| {
                        self.create_body(
                            rigid_body_builder(body_type, p, Quaternion::IDENTITY),
                            Self::collider_desc(shape.clone(), mass, restitution),
                        )
                        .0
                    })
                    .collect(),
            )
        } else {
            PhysicsBody::Single(
                self.create_body(
                    rigid_body_builder(body_type, node.position, node.quaternion),
                    Self::collider_desc(shape, mass, restitution),
                )
                .0,
            )
        };
        if mass > 0. {
            self.meshes.push((mesh, body.clone()));
        }
        Ok(Some(body))
    }
    /// removeMesh( mesh ): its bodies and their colliders.
    pub fn remove_mesh(&mut self, mesh: Object3D) {
        let Some(i) = self.meshes.iter().position(|(m, _)| *m == mesh) else {
            return;
        };
        let (_, body) = self.meshes.remove(i);
        let handles = match body {
            PhysicsBody::Single(h) => vec![h],
            PhysicsBody::Instanced(h) => h,
        };
        for h in handles {
            self.bodies.remove(
                h,
                &mut self.islands,
                &mut self.colliders,
                &mut self.impulse_joints,
                &mut self.multibody_joints,
                &mut self.soft_bodies,
                true,
            );
        }
    }
    /// The body of a mesh with mass ( an instance's for instanced meshes ).
    pub fn body(&self, mesh: Object3D, index: usize) -> Option<RigidBodyHandle> {
        self.meshes
            .iter()
            .find(|(m, _)| *m == mesh)
            .and_then(|(_, b)| match b {
                PhysicsBody::Single(h) => Some(*h),
                PhysicsBody::Instanced(h) => h.get(index).copied(),
            })
    }
    /// setMeshPosition( mesh, position, index ): zero velocities, then the
    /// translation, without waking the body.
    pub fn set_mesh_position(&mut self, mesh: Object3D, position: Vector3, index: usize) {
        if let Some(b) = self.body(mesh, index).and_then(|h| self.bodies.get_mut(h)) {
            b.set_angvel(Vector::ZERO, false);
            b.set_linvel(Vector::ZERO, false);
            b.set_translation(vector(position), false);
        }
    }
    /// setMeshVelocity( mesh, velocity, index ).
    pub fn set_mesh_velocity(&mut self, mesh: Object3D, velocity: Vector3, index: usize) {
        if let Some(b) = self.body(mesh, index).and_then(|h| self.bodies.get_mut(h)) {
            b.set_linvel(vector(velocity), false);
        }
    }
    /// applyImpulse( mesh, impulse, index ): waking the body.
    pub fn apply_impulse(&mut self, mesh: Object3D, impulse: Vector3, index: usize) {
        if let Some(b) = self.body(mesh, index).and_then(|h| self.bodies.get_mut(h)) {
            b.apply_impulse(vector(impulse), true);
        }
    }
    /// addHeightfield( mesh, width, depth, heights, scale ): a fixed body at
    /// the mesh's pose with ColliderDesc.heightfield( width, depth, … ).
    pub fn add_heightfield(
        &mut self,
        position: Vector3,
        rotation: Quaternion,
        width: usize,
        depth: usize,
        heights: Vec<f32>,
        scale: Vector3,
    ) -> Result<RigidBodyHandle> {
        if heights.len() != (width + 1) * (depth + 1) {
            return Err(Error::Invalid("heightfield size"));
        }
        // RawShape.heightfield( nrows, ncols, … ): column-major.
        let heights = rapier3d::parry::utils::Array2::new(width + 1, depth + 1, heights);
        let shape =
            SharedShape::heightfield_with_flags(heights, vector(scale), HeightFieldFlags::empty());
        Ok(self
            .create_body(
                rigid_body_builder(RigidBodyType::Fixed, position, rotation),
                Self::default_collider(shape),
            )
            .0)
    }
    /// `world.timestep = delta; world.step()`. Scene queries read the broad
    /// phase's tree, which the step keeps, as rapier.js's queries do.
    pub fn world_step(&mut self, delta: f64) {
        self.integration_parameters.dt = delta as f32;
        self.pipeline.step(
            self.gravity,
            &self.integration_parameters,
            &mut self.islands,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.impulse_joints,
            &mut self.multibody_joints,
            &mut self.soft_bodies,
            &mut self.ccd_solver,
            &(),
            &(),
        );
    }
    /// The scene queries of the controllers' bindings:
    /// `broadPhase.as_query_pipeline_mut( narrowPhase.query_dispatcher(), … )`.
    pub fn query_pipeline_mut<'a>(&'a mut self, filter: QueryFilter<'a>) -> QueryPipelineMut<'a> {
        self.broad_phase.as_query_pipeline_mut(
            self.narrow_phase.query_dispatcher(),
            &mut self.bodies,
            &mut self.colliders,
            filter,
        )
    }
    /// step(): `world.timestep = Math.min( timer.getDelta(), 1 / 30 )`, the
    /// world step, then each mesh with mass takes its body's pose ( an
    /// instanced mesh, its instances' matrices ).
    pub fn step(&mut self, s: &mut Scene, delta: f64) -> Result<()> {
        self.world_step(delta.min(1. / 30.));
        for (mesh, body) in &self.meshes {
            match body {
                PhysicsBody::Single(h) => {
                    let (p, q) = body_pose(&self.bodies[*h]);
                    let n = s.get_mut(*mesh)?;
                    n.position = p;
                    n.quaternion = q;
                }
                PhysicsBody::Instanced(handles) => {
                    let poses: Vec<_> = handles
                        .iter()
                        .map(|h| body_pose(&self.bodies[*h]))
                        .collect();
                    let n = s.get_mut(*mesh)?;
                    for (instance, (p, q)) in n.instances.iter_mut().zip(poses) {
                        instance.matrix = crate::math::Matrix4::from_rotation_translation(q, p);
                    }
                }
            }
        }
        Ok(())
    }
    /// `world.debugRender()`: the line segments' positions and RGBA colors,
    /// converted from the pipeline's HSLA as rapier.js converts them.
    pub fn debug_lines(&mut self, vertices: &mut Vec<f32>, colors: &mut Vec<f32>) {
        vertices.clear();
        colors.clear();
        let mut backend = Lines { vertices, colors };
        self.debug_render.render(
            &mut backend,
            &self.bodies,
            &self.colliders,
            &self.impulse_joints,
            &self.multibody_joints,
            &self.narrow_phase,
            &self.soft_bodies,
        );
    }
}

struct Lines<'a> {
    vertices: &'a mut Vec<f32>,
    colors: &'a mut Vec<f32>,
}
impl DebugRenderBackend for Lines<'_> {
    fn draw_line(&mut self, _object: DebugRenderObject, a: Vector, b: Vector, color: [f32; 4]) {
        self.vertices.extend_from_slice(&a.to_array());
        self.vertices.extend_from_slice(&b.to_array());
        let [r, g, b] = hsl_to_rgb(color[0], color[1], color[2]);
        self.colors
            .extend_from_slice(&[r, g, b, color[3], r, g, b, color[3]]);
    }
}

/// palette 0.7's unclamped Hsl → Rgb ( `FromColorUnclamped<Hsl>` ).
fn hsl_to_rgb(hue: f32, saturation: f32, lightness: f32) -> [f32; 3] {
    let c = (1. - (lightness * 2. - 1.).abs()) * saturation;
    let h = (hue - (hue / 360.).floor() * 360.) / 60.;
    let h_mod_two = h - (h * 0.5).floor() * 2.;
    let x = c * (1. - (h_mod_two - 1.).abs());
    let m = lightness - c * 0.5;
    let zone = |lo: f32| h >= lo && h < lo + 1.;
    let red = if zone(1.) || zone(4.) {
        x
    } else if zone(2.) || zone(3.) {
        0.
    } else {
        c
    };
    let green = if zone(0.) || zone(3.) {
        x
    } else if zone(1.) || zone(2.) {
        c
    } else {
        0.
    };
    let blue = if zone(0.) || zone(1.) {
        0.
    } else if zone(3.) || zone(4.) {
        c
    } else {
        x
    };
    [red + m, green + m, blue + m]
}
