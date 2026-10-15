//! Per-object motion state shared by temporal gallery examples.
use super::*;
use crate::tsl::motion::{MotionVectors, PreviousPose, clip_position, previous_position};
struct Item {
    object: Object3D,
    previous: Option<Matrix4>,
    pose: Option<PreviousPose>,
}
pub(super) struct MotionScene(Vec<Item>);
impl MotionScene {
    pub async fn new(r: &Renderer, s: &mut Scene, objects: &[Object3D]) -> Result<Self> {
        s.update()?;
        let graph = |previous| MotionVectors {
            current_clip: clip_position(
                position_geometry(),
                std::array::from_fn(|i| uniform(i, Type::Vec4)),
            ),
            previous_clip: clip_position(
                previous,
                std::array::from_fn(|i| uniform(i + 4, Type::Vec4)),
            ),
        };
        let rigid = Arc::new(
            graph(position_geometry())
                .build(r, &SurfaceNodes::default(), &[], &[])
                .await?,
        );
        let shadow = Arc::new(shadow_program(r, None, None).await?);
        let mut items = Vec::new();
        for &object in objects {
            let n = s.get(object)?;
            let deformed =
                n.skin.is_some() || n.geometry().is_some_and(|g| !g.morph_attributes.is_empty());
            let pose = if deformed {
                Some(PreviousPose::new(r, s, object)?)
            } else {
                None
            };
            let program = if let Some(p) = &pose {
                Arc::new(
                    graph(previous_position(0))
                        .build(
                            r,
                            &SurfaceNodes::default(),
                            &[(&p.buffer, Type::Float)],
                            &[],
                        )
                        .await?,
                )
            } else {
                rigid.clone()
            };
            if let NodeKind::Mesh(m) = &mut s.get_mut(object)?.kind {
                for material in &mut m.materials {
                    let p = Arc::make_mut(material).properties_mut();
                    if p.vertex_program.is_some() {
                        return Err(Error::Invalid(
                            "motion scene cannot replace an existing vertex graph",
                        ));
                    }
                    p.vertex_program = Some(program.clone());
                    p.shadow_program = Some(shadow.clone());
                }
            }
            items.push(Item {
                object,
                previous: None,
                pose,
            });
        }
        Ok(Self(items))
    }
    pub fn update(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        current_projection: Option<Matrix4>,
    ) -> Result<()> {
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let view_projection = camera.projection_matrix()? * world.inverse();
        let current_view_projection =
            current_projection.map_or(view_projection, |p| p * world.inverse());
        for item in &mut self.0 {
            if let Some(pose) = &mut item.pose {
                pose.update(r, s)?;
            }
            let model = s.get(item.object)?.matrix_world;
            let current = current_view_projection * model;
            let previous = item.previous.unwrap_or(current);
            if let NodeKind::Mesh(m) = &mut s.get_mut(item.object)?.kind {
                for material in &mut m.materials {
                    let p = Arc::make_mut(material).properties_mut();
                    for (slot, value) in current
                        .to_cols_array_2d()
                        .into_iter()
                        .chain(previous.to_cols_array_2d())
                        .enumerate()
                    {
                        p.vertex_uniforms[slot] = value.map(|v| v as f32);
                    }
                }
            }
            item.previous = Some(view_projection * model);
        }
        Ok(())
    }
}
