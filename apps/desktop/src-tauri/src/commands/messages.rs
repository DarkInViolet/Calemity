use crate::core::database::Database;
use calemity_api::{
    error::{ApiError, ApiErrorCode},
    messages::{LoadMessagesRequest, SendMessageRequest},
};
use calemity_protocol::models::message::Message;
use calemity_storage::{get_messages, insert_message};
use tauri::State;

#[tauri::command]
pub async fn send_message(
    state: State<'_, Database>,
    request: SendMessageRequest,
) -> Result<(), ApiError> {
    if request.content.trim().is_empty() {
        return Err(ApiError::new(
            ApiErrorCode::MessageContentEmpty,
            "Message content cannot be empty",
        ));
    }

    let message = Message::new(
        request.author_id,
        request.conversation_id,
        request.device_id,
        request.content,
    );

    insert_message(&state.pool, &message)
        .await
        .map_err(|error| {
            eprintln!("Failed to store message: {error}");

            ApiError::new(ApiErrorCode::StorageFailure, "Failed to send the message")
        })?;

    Ok(())
}

#[tauri::command]
pub async fn load_messages(
    state: State<'_, Database>,
    request: LoadMessagesRequest,
) -> Result<Vec<Message>, ApiError> {
    get_messages(&state.pool, &request.conversation_id)
        .await
        .map_err(|error| {
            eprintln!("Failed to load messages: {error}");

            ApiError::new(ApiErrorCode::StorageFailure, "Failed to load messages")
        })
}
