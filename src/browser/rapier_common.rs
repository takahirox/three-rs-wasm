//! What the physics_rapier pages share: RapierPhysics.js's setInterval step
//! with its Timer delta, the pages' own intervals, and RapierHelper.
use crate::attribute::BufferAttribute;
use crate::physics::RapierPhysics;
use crate::{Result, geometry::*, material::*, scene::*};
use std::sync::Arc;

/// The fixture's Math.random: a 32-bit LCG seeded with 186.
pub(super) struct Random(pub u32);
impl Random {
    pub fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
        f64::from(self.0) / 4294967296.
    }
}

/// setInterval( fn, ms ) on the example clock. The delay is an integer
/// ( WebIDL long ): RapierPhysics's 1000 / 60 runs every 16 ms. Timers due
/// at the same time run in the order they were last scheduled.
pub(super) struct Interval {
    pub period: f64,
    pub at: f64,
    /// When it was last scheduled: earlier ones run first on a tie.
    scheduled: f64,
    order: u32,
}
impl Interval {
    pub fn new(now: f64, period: f64, order: u32) -> Self {
        let period = period.trunc().max(0.);
        Self {
            period,
            at: now + period,
            scheduled: now,
            order,
        }
    }
    fn key(&self) -> (f64, f64, u32) {
        (self.at, self.scheduled, self.order)
    }
    fn fire(&mut self) -> f64 {
        let t = self.at;
        self.scheduled = t;
        self.at = t + self.period;
        t
    }
}
/// The next due interval up to `now`: its index and time, after which it is
/// scheduled again.
pub(super) fn next_due(intervals: &mut [Interval], now: f64) -> Option<(usize, f64)> {
    let i = (0..intervals.len())
        .filter(|&i| intervals[i].at <= now)
        .min_by(|&a, &b| {
            intervals[a]
                .key()
                .partial_cmp(&intervals[b].key())
                .unwrap_or(std::cmp::Ordering::Equal)
        })?;
    Some((i, intervals[i].fire()))
}

/// RapierPhysics.js's Timer: the step's delta, from the interval's time.
#[derive(Default)]
pub(super) struct StepTimer {
    current: f64,
}
impl StepTimer {
    /// timer.update( now ); timer.getDelta(), in seconds.
    pub fn delta(&mut self, now: f64) -> f64 {
        let previous = self.current;
        self.current = now;
        (self.current - previous) / 1000.
    }
}

/// RapierHelper: LineSegments with vertex colors, given the world's debug
/// render on each update(). As on the page, each update replaces the
/// position and color attributes with the frame's lines; the engine writes
/// them into the geometry's resident vertex buffer, which grows only when
/// they do not fit.
pub(super) struct Helper {
    pub node: Object3D,
    vertices: Vec<f32>,
    colors: Vec<f32>,
}
impl Helper {
    pub fn new(s: &mut Scene) -> Result<Self> {
        let material = LineBasicMaterial {
            properties: MaterialProperties {
                vertex_colors: true,
                ..Default::default()
            },
            ..Default::default()
        };
        let node = s.insert(NodeKind::Line(Line {
            geometry: Arc::new(BufferGeometry::default()),
            material: Arc::new(Material::Line(material)),
            segments: true,
        }));
        s.get_mut(node)?.frustum_culled = false;
        Ok(Self {
            node,
            vertices: vec![],
            colors: vec![],
        })
    }
    /// update(): the world's debugRender() as the new attributes.
    pub fn update(&mut self, s: &mut Scene, physics: &mut RapierPhysics) -> Result<()> {
        physics.debug_lines(&mut self.vertices, &mut self.colors);
        let NodeKind::Line(l) = &mut s.get_mut(self.node)?.kind else {
            return Ok(());
        };
        let g = Arc::make_mut(&mut l.geometry);
        g.set_attribute(
            "position",
            Attribute::F32(BufferAttribute::new(self.vertices.clone(), 3, false)?),
        );
        g.set_attribute(
            "color",
            Attribute::F32(BufferAttribute::new(self.colors.clone(), 4, false)?),
        );
        Ok(())
    }
}
