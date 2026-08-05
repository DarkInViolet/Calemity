use calemity_protocol::models::message::Message;
use calemity_storage::{get_messages, insert_message};
use tauri::State;
use ulid::Ulid;

use crate::core::database::Database;

#[tauri::command]
pub async fn send_message(
    state: State<'_, Database>,

    author_id: String,
    conversation_id: String,
    device_id: String,
    content: String,
) -> Result<(), String> {
    let author = Ulid::from_string(&author_id).map_err(|e| e.to_string())?;

    let conversation = Ulid::from_string(&conversation_id).map_err(|e| e.to_string())?;

    let message = Message::new(author, conversation, device_id, content);

    insert_message(&state.pool, &message)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn load_messages(
    state: State<'_, Database>,

    conversation_id: String,
) -> Result<Vec<Message>, String> {
    let conversation = Ulid::from_string(&conversation_id).map_err(|e| e.to_string())?;

    get_messages(&state.pool, &conversation)
        .await
        .map_err(|e| e.to_string())
}
