//! monomorph's `Pool`: instances live in an array, a destroyed one leaves its
//! index on a free list, and `create` reuses the last freed index before it
//! appends ( the version turns odd on destroy, even on reuse ). Iteration
//! visits the live instances in index order. bounce's solver and broadphase
//! orders follow from this, so the port keeps it.

pub struct Pool<T> {
    pub items: Vec<T>,
    versions: Vec<u32>,
    free: Vec<usize>,
    pub length: usize,
}

impl<T> Default for Pool<T> {
    fn default() -> Self {
        Self {
            items: vec![],
            versions: vec![],
            free: vec![],
            length: 0,
        }
    }
}

impl<T> Pool<T> {
    /// `Class.create( data, pool )`: `reset` initialises a reused instance,
    /// `new` builds a fresh one; both return its index.
    pub fn create(&mut self, reset: impl FnOnce(&mut T), new: impl FnOnce() -> T) -> usize {
        if let Some(index) = self.free.pop() {
            self.length += 1;
            self.versions[index] += 1;
            reset(&mut self.items[index]);
            index
        } else {
            let index = self.items.len();
            self.items.push(new());
            self.versions.push(0);
            self.length += 1;
            index
        }
    }
    /// `instance.destroy()`.
    pub fn destroy(&mut self, index: usize) {
        if self.versions[index] & 1 == 1 {
            return;
        }
        self.versions[index] += 1;
        self.length -= 1;
        self.free.push(index);
    }
    pub fn is_alive(&self, index: usize) -> bool {
        self.versions[index] & 1 == 0
    }
    pub fn version(&self, index: usize) -> u32 {
        self.versions[index]
    }
    /// The live indices in iteration order.
    pub fn live(&self) -> Vec<usize> {
        (0..self.items.len())
            .filter(|&i| self.is_alive(i))
            .collect()
    }
    /// `destroyAllInstancesInPool( pool )`: destroys in index order.
    pub fn destroy_all(&mut self) {
        for i in 0..self.items.len() {
            if self.is_alive(i) {
                self.destroy(i);
            }
        }
    }
}

impl<T> std::ops::Index<usize> for Pool<T> {
    type Output = T;
    fn index(&self, i: usize) -> &T {
        &self.items[i]
    }
}

impl<T> std::ops::IndexMut<usize> for Pool<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        &mut self.items[i]
    }
}

/// An insertion-ordered `Set` of indices.
#[derive(Default)]
pub struct OrderedSet {
    items: Vec<usize>,
    present: Vec<bool>,
}

impl OrderedSet {
    pub fn add(&mut self, i: usize) {
        if self.present.len() <= i {
            self.present.resize(i + 1, false);
        }
        if !self.present[i] {
            self.present[i] = true;
            self.items.push(i);
        }
    }
    pub fn items(&self) -> Vec<usize> {
        self.items.clone()
    }
    pub fn clear(&mut self) {
        for &i in &self.items {
            self.present[i] = false;
        }
        self.items.clear();
    }
}
