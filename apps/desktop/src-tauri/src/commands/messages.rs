use calemity_api::messages::SendMessageRequest;
use calemity_protocol::models::message::Message;
use calemity_storage::{get_messages, insert_message};
use tauri::State;
use ulid::Ulid;

use crate::core::database::Database;

#[tauri::command]
pub async fn send_message(
    state: State<'_, Database>,
    request: SendMessageRequest,
) -> Result<(), String> {
    let message = Message::new(
        request.author_id,
        request.conversation_id,
        request.device_id,
        request.content,
    );

    insert_message(&state.pool, &message)
        .await
        .map_err(|error| error.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn load_messages(
    state: State<'_, Database>,
    conversation_id: String,
) -> Result<Vec<Message>, String> {
    let conversation = Ulid::from_string(&conversation_id).map_err(|error| error.to_string())?;

    get_messages(&state.pool, &conversation)
        .await
        .map_err(|error| error.to_string())
}
