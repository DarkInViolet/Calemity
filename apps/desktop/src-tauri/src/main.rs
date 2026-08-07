#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod core;
use commands::{
    conversations::{create_conversation, list_conversations},
    messages::{load_messages, send_message},
};

use core::{database::setup_database, identity::setup_local_identity};

#[tokio::main]
async fn main() {
    let database = setup_database().await;
    let identity = setup_local_identity(&database.pool).await;

    tauri::Builder::default()
        .manage(database)
        .manage(identity)
        .invoke_handler(tauri::generate_handler![
            create_conversation,
            list_conversations,
            send_message,
            load_messages
        ])
        .run(tauri::generate_context!())
        .expect("Tauri error");
}
