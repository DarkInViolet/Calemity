use serde::{Deserialize, Serialize};
use ulid::Ulid;

/// Identifies the local Calemity user and the device currently running
/// the application.
///
/// This is deliberately separate from cryptographic identity for now.
/// Account keys and device keys can later be attached to these stable IDs
/// without changing the identity of the user or device.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalIdentity {
    pub user_id: Ulid,
    pub device_id: Ulid,
}

impl LocalIdentity {
    /// Creates a brand-new local user and device identity.
    pub fn new() -> Self {
        Self {
            user_id: Ulid::new(),
            device_id: Ulid::new(),
        }
    }

    /// Reconstructs an existing local identity from persisted IDs.
    pub fn from_ids(user_id: Ulid, device_id: Ulid) -> Self {
        Self { user_id, device_id }
    }
}

impl Default for LocalIdentity {
    fn default() -> Self {
        Self::new()
    }
}
