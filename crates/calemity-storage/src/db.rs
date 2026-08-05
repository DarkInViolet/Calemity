use calemity_protocol::models::message::Message;
use sqlx::{sqlite::SqlitePoolOptions, Row, SqlitePool};
use std::path::Path;
use ulid::Ulid;

pub async fn init_db(db_path: &Path) -> Result<SqlitePool, sqlx::Error> {
    let db_url = format!("sqlite://{}?mode=rwc", db_path.display());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT NOT NULL,
            identity_pubkey TEXT NOT NULL,
            created_at INTEGER NOT NULL
    );
    ",
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS messages (
            id TEXT PRIMARY KEY,
            author_id TEXT NOT NULL,
            conversation_id TEXT NOT NULL,
            device_id TEXT NOT NULL,
            content TEXT NOT NULL,
            timestamp INTEGER NOT NULL
    );
    ",
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}

pub async fn insert_message(pool: &SqlitePool, msg: &Message) -> Result<(), sqlx::Error> {
    sqlx::query(
        "
        INSERT INTO messages
        (
            id,
            author_id,
            conversation_id,
            device_id,
            content,
            timestamp
    )

    VALUES (?, ?, ?, ?, ?, ?)
    ",
    )
    .bind(msg.id.to_string())
    .bind(msg.author_id.to_string())
    .bind(msg.conversation_id.to_string())
    .bind(&msg.device_id)
    .bind(&msg.content)
    .bind(msg.timestamp as i64)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_messages(
    pool: &SqlitePool,
    conversation_id: &Ulid,
) -> Result<Vec<Message>, sqlx::Error> {
    let rows = sqlx::query(
        "
        SELECT *
        FROM messages
        WHERE conversation_id = ?
        ORDER BY timestamp ASC
        ",
    )
    .bind(conversation_id.to_string())
    .fetch_all(pool)
    .await?;

    let mut messages = Vec::new();

    for row in rows {
        let msg = Message {
            id: Ulid::from_string(row.try_get("id")?)
                .map_err(|_| sqlx::Error::Protocol("Invalid message ID".into()))?,

            author_id: Ulid::from_string(row.try_get("author_id")?)
                .map_err(|_| sqlx::Error::Protocol("Invalid author ID".into()))?,

            conversation_id: Ulid::from_string(row.try_get("conversation_id")?)
                .map_err(|_| sqlx::Error::Protocol("Invalid conversation ID".into()))?,

            device_id: row.try_get("device_id")?,

            content: row.try_get("content")?,

            timestamp: row.try_get::<i64, _>("timestamp")? as u64,
        };
        messages.push(msg);
    }
    Ok(messages)
}
