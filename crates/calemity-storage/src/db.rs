use calemity_identity::LocalIdentity;
use calemity_protocol::models::{conversation::Conversation, message::Message};
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
        CREATE TABLE IF NOT EXISTS local_identity (
            slot INTEGER PRIMARY KEY CHECK (slot = 1),
                user_id TEXT NOT NULL,
                device_id TEXT NOT NULL
    );
    ",
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS conversations (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
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

    sqlx::query(
        "
        CREATE INDEX IF NOT EXISTS idx_messages_conversation_timestamp
        ON messages(conversation_id, timestamp);
    ",
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}

pub async fn insert_local_identity(
    pool: &SqlitePool,
    identity: &LocalIdentity,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "
        INSERT INTO local_identity (
            slot,
            user_id,
            device_id
    )
    VALUES (1, ?, ?)
    ",
    )
    .bind(identity.user_id().to_string())
    .bind(identity.device_id().to_string())
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_local_identity(pool: &SqlitePool) -> Result<Option<LocalIdentity>, sqlx::Error> {
    let row = sqlx::query(
        "
        SELECT user_id, device_id
        FROM local_identity
        WHERE slot = 1
        ",
    )
    .fetch_optional(pool)
    .await?;

    let Some(row) = row else {
        return Ok(None);
    };

    let user_id = Ulid::from_string(row.try_get("user_id")?)
        .map_err(|_| sqlx::Error::Protocol("Invalid local user ID".into()))?;

    let device_id = Ulid::from_string(row.try_get("device_id")?)
        .map_err(|_| sqlx::Error::Protocol("Invalid local device ID".into()))?;

    Ok(Some(LocalIdentity::from_ids(user_id, device_id)))
}

pub async fn insert_conversation(
    pool: &SqlitePool,
    conversation: &Conversation,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "
        INSERT INTO conversations (
            id,
            title,
            created_at
    )
    VALUES (?, ?, ?)
    ",
    )
    .bind(conversation.id.to_string())
    .bind(&conversation.title)
    .bind(conversation.created_at as i64)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_conversations(pool: &SqlitePool) -> Result<Vec<Conversation>, sqlx::Error> {
    let rows = sqlx::query(
        "
        SELECT id, title, created_at
        FROM conversations
        ORDER BY created_at ASC, id ASC
        ",
    )
    .fetch_all(pool)
    .await?;

    let mut conversations = Vec::new();

    for row in rows {
        let conversation = Conversation {
            id: Ulid::from_string(row.try_get("id")?)
                .map_err(|_| sqlx::Error::Protocol("Invalid conversation ID".into()))?,
            title: row.try_get("title")?,
            created_at: row.try_get::<i64, _>("created_at")? as u64,
        };

        conversations.push(conversation);
    }

    Ok(conversations)
}

pub async fn insert_message(pool: &SqlitePool, message: &Message) -> Result<(), sqlx::Error> {
    sqlx::query(
        "
        INSERT INTO messages (
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
    .bind(message.id.to_string())
    .bind(message.author_id.to_string())
    .bind(message.conversation_id.to_string())
    .bind(&message.device_id)
    .bind(&message.content)
    .bind(message.timestamp as i64)
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
        SELECT
        id,
        author_id,
        conversation_id,
        device_id,
        content,
        timestamp
        FROM messages
        WHERE conversation_id = ?
        ORDER BY timestamp ASC, id ASC
        ",
    )
    .bind(conversation_id.to_string())
    .fetch_all(pool)
    .await?;

    let mut messages = Vec::new();

    for row in rows {
        let message = Message {
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

        messages.push(message);
    }

    Ok(messages)
}
