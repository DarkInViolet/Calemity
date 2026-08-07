use calemity_identity::LocalIdentity;
use calemity_storage::{get_local_identity, insert_local_identity};
use sqlx::SqlitePool;

pub async fn setup_local_identity(pool: &SqlitePool) -> LocalIdentity {
    if let Some(identity) = get_local_identity(pool)
        .await
        .expect("Could not load local identity")
    {
        eprintln!(
            "Loaded local identity: user={}, device={}",
            identity.user_id, identity.device_id
        );

        return identity;
    }

    let identity = LocalIdentity::new();

    insert_local_identity(pool, &identity)
        .await
        .expect("Could not store local identity");

    eprintln!(
        "Created local identity: user={}, device={}",
        identity.user_id, identity.device_id
    );

    identity
}
