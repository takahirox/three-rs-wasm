use serde::Serialize;
use std::sync::atomic::{AtomicU32, Ordering};
static NEXT_ID: AtomicU32 = AtomicU32::new(0);

/// Copying an object creates a new identity; copying fields into an existing
/// object can preserve its identity explicitly with `copy_from`.
#[derive(Debug, Serialize)]
pub struct Identity {
    pub id: u32,
    pub uuid: uuid::Uuid,
}
impl Default for Identity {
    fn default() -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            uuid: uuid::Uuid::new_v4(),
        }
    }
}
impl Clone for Identity {
    fn clone(&self) -> Self {
        Self::default()
    }
}
