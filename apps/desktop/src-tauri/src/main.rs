#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod core;

use commands::messages::{load_messages, send_message};

use core::database::setup_database;

#[tokio::main]
async fn main() {
    let database = setup_database().await;

    tauri::Builder::default()
        .manage(database)
        .invoke_handler(tauri::generate_handler![send_message, load_messages])
        .run(tauri::generate_context!())
        .expect("Tauri error");
}
