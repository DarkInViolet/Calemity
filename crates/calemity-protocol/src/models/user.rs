use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use ulid::Ulid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Ulid,
    pub username: String,
    pub identity_pubkey: String,
    pub created_at: u64,
}

impl User {
    pub fn new(username: String, identity_pubkey: String) -> Self {
        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            id: Ulid::new(),
            username,
            identity_pubkey,
            created_at,
        }
    }
}
