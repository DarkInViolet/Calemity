use crate::core::database::Database;
use calemity_api::{
    error::{ApiError, ApiErrorCode},
    messages::{LoadMessagesRequest, MessageView, SendMessageRequest},
};
use calemity_identity::LocalIdentity;
use calemity_protocol::models::message::Message;
use calemity_storage::{get_messages, insert_message};
use tauri::State;
use ulid::Ulid;

fn message_to_view(message: Message, current_user_id: Ulid) -> MessageView {
    MessageView {
        id: message.id,
        content: message.content,
        timestamp: message.timestamp,
        is_own: message.author_id == current_user_id,
    }
}

#[tauri::command]
pub async fn send_message(
    database: State<'_, Database>,
    identity: State<'_, LocalIdentity>,
    request: SendMessageRequest,
) -> Result<MessageView, ApiError> {
    if request.content.trim().is_empty() {
        return Err(ApiError::new(
            ApiErrorCode::MessageContentEmpty,
            "Message content cannot be empty",
        ));
    }

    let message = Message::new(
        identity.user_id(),
        request.conversation_id,
        identity.device_id().to_string(),
        request.content,
    );

    insert_message(&database.pool, &message)
        .await
        .map_err(|error| {
            eprintln!("Failed to store message: {error}");

            ApiError::new(ApiErrorCode::StorageFailure, "Failed to send the message")
        })?;

    Ok(message_to_view(message, identity.user_id()))
}

#[tauri::command]
pub async fn load_messages(
    database: State<'_, Database>,
    identity: State<'_, LocalIdentity>,
    request: LoadMessagesRequest,
) -> Result<Vec<MessageView>, ApiError> {
    let messages = get_messages(&database.pool, &request.conversation_id)
        .await
        .map_err(|error| {
            eprintln!("Failed to load messages: {error}");

            ApiError::new(ApiErrorCode::StorageFailure, "Failed to load messages")
        })?;

    Ok(messages
        .into_iter()
        .map(|message| message_to_view(message, identity.user_id()))
        .collect())
}
