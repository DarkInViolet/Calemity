use calemity_api::messages::{LoadMessagesRequest, SendMessageRequest};
use calemity_protocol::models::message::Message;
use calemity_storage::{get_messages, insert_message};
use tauri::State;

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
    request: LoadMessagesRequest,
) -> Result<Vec<Message>, String> {
    get_messages(&state.pool, &request.conversation_id)
        .await
        .map_err(|error| error.to_string())
}
