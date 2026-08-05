use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use ulid::Ulid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Ulid,
    pub author_id: Ulid,
    pub conversation_id: Ulid,
    pub device_id: String,
    pub content: String,
    pub timestamp: u64,
}

impl Message {
    pub fn new(author_id: Ulid, conversation_id: Ulid, device_id: String, content: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            id: Ulid::new(),
            author_id,
            conversation_id,
            device_id,
            content,
            timestamp,
        }
    }
}
