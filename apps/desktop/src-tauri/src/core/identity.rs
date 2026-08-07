use calemity_identity::LocalIdentity;
use calemity_storage::{get_local_identity, insert_local_identity};
use sqlx::SqlitePool;

pub async fn setup_local_identity(pool: &SqlitePool) -> LocalIdentity {
    if let Some(identity) = get_local_identity(pool)
        .await
        .expect("Could not load local identity")
    {
        return identity;
    }

    let identity = LocalIdentity::new();

    insert_local_identity(pool, &identity)
        .await
        .expect("Could not store local identity");

    identity
}
