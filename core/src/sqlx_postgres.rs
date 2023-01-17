use serde::Serialize;
use sqlx::types::Json;
use sqlx::{
    migrate::Migrator,
    postgres::{PgConnection, PgPool, PgPoolOptions, PgQueryResult, PgRow, Postgres},
    Error as SqlxError, Executor, Row,
};
use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
    str::FromStr,
};

// re-export
pub use sqlx;

pub static EMBEDDED_MIGRATE: Migrator = sqlx::migrate!();

pub async fn connect_and_migrate(database_url: &str, max_connections: u32) -> sqlx::Result<PgPool> {
    create_database(database_url).await?;
    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(database_url)
        .await?;

    EMBEDDED_MIGRATE.run(&pool).await?;
    Ok(pool)
}

pub async fn create_database(uri: &str) -> sqlx::Result<()> {
    use sqlx::any::Any;
    use sqlx::migrate::MigrateDatabase;

    if !Any::database_exists(uri).await? {
        Any::create_database(uri).await
    } else {
        Ok(())
    }
}
