use std::env;

use diesel::SqliteConnection;
use diesel_async::{
    SimpleAsyncConnection,
    pooled_connection::{
        AsyncDieselConnectionManager,
        deadpool::{Hook, HookError, Object, Pool, PoolError},
    },
    sync_connection_wrapper::SyncConnectionWrapper,
};
use dotenvy::dotenv;
use once_cell::sync::Lazy;

/// Type alias for the deadpool-backed async SQLite connection pool.
pub type DbPool = Pool<SyncConnectionWrapper<SqliteConnection>>;

/// Acquires a connection from the global pool.
///
/// # Returns
///
/// A pooled connection object ready for Diesel queries.
///
/// # Errors
///
/// Returns [`PoolError`] when the pool has been shut down or all connections
/// are exhausted.
pub async fn connect_from_pool()
-> Result<Object<SyncConnectionWrapper<SqliteConnection>>, PoolError> {
    DB_POOL.get().await
}

/// Creates a new deadpool connection pool for the given SQLite database URL.
///
/// A `post_create` hook sets SQLite pragmas on every new connection:
/// `busy_timeout` first (so later pragmas can wait), then WAL mode,
/// `synchronous = NORMAL`, and foreign-key enforcement.
///
/// # Arguments
///
/// * `database_url` - Path or `sqlite://` URI to the SQLite database file.
///
/// # Panics
///
/// Panics if the pool cannot be built (e.g., invalid configuration).
pub fn create_pool(database_url: &str) -> DbPool {
    let config =
        AsyncDieselConnectionManager::<SyncConnectionWrapper<SqliteConnection>>::new(database_url);

    Pool::builder(config)
        .post_create(Hook::async_fn(
            |conn: &mut SyncConnectionWrapper<SqliteConnection>, _meta| {
                Box::pin(async move {
                    conn.batch_execute("PRAGMA busy_timeout = 2000;")
                        .await
                        .map_err(|e| {
                            HookError::Message(format!("Failed to set busy_timeout: {e}").into())
                        })?;
                    conn.batch_execute("PRAGMA journal_mode = WAL;")
                        .await
                        .map_err(|e| {
                            HookError::Message(format!("Failed to set journal_mode: {e}").into())
                        })?;
                    conn.batch_execute("PRAGMA synchronous = NORMAL;")
                        .await
                        .map_err(|e| {
                            HookError::Message(format!("Failed to set synchronous: {e}").into())
                        })?;
                    conn.batch_execute("PRAGMA foreign_keys = ON;")
                        .await
                        .map_err(|e| {
                            HookError::Message(format!("Failed to set foreign_keys: {e}").into())
                        })?;
                    Ok(())
                })
            },
        ))
        .build()
        .expect("Failed to create SQLite connection pool")
}

/// Global connection pool, initialized lazily from the `DATABASE_URL` env var.
static DB_POOL: Lazy<DbPool> = Lazy::new(|| {
    dotenv().ok();
    let database_path = env::var("DATABASE_URL").unwrap_or_else(|_| "./database.db".to_string());
    create_pool(&database_path)
});
