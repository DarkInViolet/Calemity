use calemity_storage::init_db;
use sqlx::SqlitePool;
use std::path::PathBuf;

pub struct Database {
    pub pool: SqlitePool,
}

pub async fn setup_database() -> Database {
    let db_dir = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("calemity");

    std::fs::create_dir_all(&db_dir).expect("Could not create database directory");

    let db_path = db_dir.join("calemity.sqlite");

    let pool = init_db(&db_path)
        .await
        .expect("Could not initialize database");

    Database { pool }
}
