//! TilesFadePlugin and its FadeManager ( 250 ms fades ): which tiles fade in
//! and out, the setTileVisible answer that keeps a fading-out tile drawn,
//! and the fade values its materials' dithered discard reads.
use super::tiles::TileId;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct FadeState {
    pub fade_in_target: f64,
    pub fade_out_target: f64,
    pub fade_in: f64,
    pub fade_out: f64,
}

#[derive(Default)]
pub(crate) struct Fade {
    pub duration: f64,
    pub states: Vec<(TileId, FadeState)>,
    last_tick: Option<f64>,
    pub now: f64,
    pub quick: HashSet<TileId>,
    popped_in: HashSet<TileId>,
    fading_out_count: i64,
    pub maximum_fade_out_tiles: i64,
    display_active_tiles: bool,
    fading_before: usize,
    /// Tiles whose fade ended hidden: the other plugins hide them.
    pub completed_hidden: Vec<TileId>,
    /// The camera moved fast: every fade completes ( set by the demo ).
    pub moving_fast: bool,
}

impl Fade {
    pub fn new() -> Self {
        Fade {
            duration: 250.,
            maximum_fade_out_tiles: 50,
            ..Default::default()
        }
    }
    fn index(&self, t: TileId) -> Option<usize> {
        self.states.iter().position(|(o, _)| *o == t)
    }
    pub fn is_fading(&self, t: TileId) -> bool {
        self.index(t).is_some()
    }
    pub fn is_fading_out(&self, t: TileId) -> bool {
        self.index(t)
            .is_some_and(|i| self.states[i].1.fade_out_target == 1.)
    }
    /// guaranteeState: true when the state was created.
    fn guarantee(&mut self, t: TileId) -> bool {
        if self.is_fading(t) {
            return false;
        }
        self.states.push((t, FadeState::default()));
        true
    }
    fn fade_in(&mut self, t: TileId) {
        self.guarantee(t);
        let i = self.index(t).expect("state");
        let s = &mut self.states[i].1;
        s.fade_in_target = 1.;
        s.fade_out_target = 0.;
        s.fade_out = 0.;
    }
    fn fade_out(&mut self, t: TileId) {
        let created = self.guarantee(t);
        let i = self.index(t).expect("state");
        let s = &mut self.states[i].1;
        s.fade_out_target = 1.;
        if created {
            s.fade_in_target = 1.;
            s.fade_in = 1.;
        }
    }
    /// completeFade: the state goes, a hidden tile is hidden by the others.
    pub fn complete(&mut self, t: TileId) {
        let Some(i) = self.index(t) else { return };
        let visible = self.states[i].1.fade_out_target == 0.;
        self.states.remove(i);
        if !visible {
            self.completed_hidden.push(t);
            self.fading_out_count -= 1;
        }
    }
    /// deleteObject.
    pub fn delete(&mut self, t: TileId) {
        self.complete(t);
    }

    /// TilesFadePlugin.setTileVisible: true stops the other plugins.
    pub fn set_tile_visible(
        &mut self,
        t: TileId,
        visible: bool,
        depth_from_rendered_parent: i64,
    ) -> bool {
        let was_fading = self.is_fading(t);
        if self.is_fading_out(t) {
            self.fading_out_count -= 1;
        }
        if !visible {
            self.fading_out_count += 1;
            self.fade_out(t);
        } else if depth_from_rendered_parent == 1 {
            // A root renderable tile pops in the first time ( fadeRootTiles false ).
            if self.popped_in.contains(&t) {
                self.fade_in(t);
            }
            self.popped_in.insert(t);
        } else {
            self.fade_in(t);
        }
        if self.quick.remove(&t) {
            self.complete(t);
        }
        if was_fading {
            return true;
        }
        !visible && self.is_fading(t)
    }

    /// update-before: active tiles display while traversing.
    pub fn update_before(&mut self, display_active_tiles: &mut bool) {
        self.fading_before = self.states.len();
        self.display_active_tiles = *display_active_tiles;
        *display_active_tiles = true;
    }

    /// update-after: FadeManager.update and the fast-camera completion.
    /// Returns the tiles whose fade-out finished.
    pub fn update_after(
        &mut self,
        display_active_tiles: &mut bool,
        _visible: &HashSet<TileId>,
    ) -> Vec<TileId> {
        *display_active_tiles = self.display_active_tiles;
        let time = self.now;
        let last = *self.last_tick.get_or_insert(time);
        let delta = ((time - last) / self.duration).clamp(0., 1.);
        self.last_tick = Some(time);
        let mut done = vec![];
        for (t, s) in &mut self.states {
            let in_sign = (s.fade_in_target - s.fade_in).signum_js();
            s.fade_in = (s.fade_in + in_sign * delta).clamp(0., 1.);
            let out_sign = (s.fade_out_target - s.fade_out).signum_js();
            s.fade_out = (s.fade_out + out_sign * delta).clamp(0., 1.);
            let out_complete = s.fade_out == 1. || s.fade_out == 0.;
            let in_complete = s.fade_in == 1. || s.fade_in == 0.;
            if (out_complete && in_complete) || s.fade_out >= s.fade_in {
                done.push(*t);
            }
        }
        for t in done {
            self.complete(t);
        }
        if self.maximum_fade_out_tiles < self.fading_out_count
            && std::mem::take(&mut self.moving_fast)
        {
            let all: Vec<TileId> = self.states.iter().map(|(t, _)| *t).collect();
            for t in all {
                self.complete(t);
            }
        }
        std::mem::take(&mut self.completed_hidden)
    }

    pub fn fading_tiles(&self) -> Vec<TileId> {
        self.states.iter().map(|(t, _)| *t).collect()
    }
    /// The fade values a tile's materials read ( 0, 0 when not fading ).
    pub fn values(&self) -> HashMap<TileId, (f64, f64)> {
        self.states
            .iter()
            .map(|(t, s)| (*t, (s.fade_in, s.fade_out)))
            .collect()
    }
}

/// Math.sign: 0 stays 0.
trait SignumJs {
    fn signum_js(self) -> f64;
}
impl SignumJs for f64 {
    fn signum_js(self) -> f64 {
        if self > 0. {
            1.
        } else if self < 0. {
            -1.
        } else {
            self
        }
    }
}
