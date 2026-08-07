use ulid::Ulid;
/// Stable identifiers for the local Calemity user and the device
/// currently running the application.
///
/// This type contains no cryptographic key material and is intentionally
/// not serializable. Secret key material should live in dedicated
/// secret-bearing types and storage.
#[derive(Clone, PartialEq, Eq)]
pub struct LocalIdentity {
    user_id: Ulid,
    device_id: Ulid,
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

    pub fn user_id(&self) -> Ulid {
        self.user_id
    }

    pub fn device_id(&self) -> Ulid {
        self.device_id
    }
}

impl Default for LocalIdentity {
    fn default() -> Self {
        Self::new()
    }
}
