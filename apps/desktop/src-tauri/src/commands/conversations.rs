use calemity_api::conversations::CreateConversationRequest;
use calemity_protocol::models::conversation::Conversation;
use calemity_storage::{get_conversations, insert_conversation};
use tauri::State;

use crate::core::database::Database;

#[tauri::command]
pub async fn create_conversation(
    state: State<'_, Database>,
    request: CreateConversationRequest,
) -> Result<Conversation, String> {
    let title = request.title.trim();

    if title.is_empty() {
        return Err("Conversation title cannot be empty".to_string());
    }

    if title.chars().count() > 100 {
        return Err("Conversation title cannot exceed 100 characters".to_string());
    }

    let conversation = Conversation::new(title.to_string());

    insert_conversation(&state.pool, &conversation)
        .await
        .map_err(|error| error.to_string())?;

    Ok(conversation)
}

#[tauri::command]
pub async fn list_conversations(state: State<'_, Database>) -> Result<Vec<Conversation>, String> {
    get_conversations(&state.pool)
        .await
        .map_err(|error| error.to_string())
}
