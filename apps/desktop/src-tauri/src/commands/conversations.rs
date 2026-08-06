use crate::core::database::Database;
use calemity_api::{
    conversations::CreateConversationRequest,
    error::{ApiError, ApiErrorCode},
};
use calemity_protocol::models::conversation::Conversation;
use calemity_storage::{get_conversations, insert_conversation};
use tauri::State;

#[tauri::command]
pub async fn create_conversation(
    state: State<'_, Database>,
    request: CreateConversationRequest,
) -> Result<Conversation, ApiError> {
    let title = request.title.trim();

    if title.is_empty() {
        return Err(ApiError::new(
            ApiErrorCode::ConversationTitleEmpty,
            "Conversation title cannot be empty",
        ));
    }

    if title.chars().count() > 100 {
        return Err(ApiError::new(
            ApiErrorCode::ConversationTitleTooLong,
            "Conversation title cannot exceed 100 characters",
        ));
    }

    let conversation = Conversation::new(title.to_string());

    insert_conversation(&state.pool, &conversation)
        .await
        .map_err(|error| {
            eprintln!("Failed to store conversation: {error}");

            ApiError::new(
                ApiErrorCode::StorageFailure,
                "Failed to save the conversation",
            )
        })?;

    Ok(conversation)
}

#[tauri::command]
pub async fn list_conversations(state: State<'_, Database>) -> Result<Vec<Conversation>, ApiError> {
    get_conversations(&state.pool).await.map_err(|error| {
        eprintln!("Failed to load conversations: {error}");

        ApiError::new(ApiErrorCode::StorageFailure, "Failed to load conversations")
    })
}
