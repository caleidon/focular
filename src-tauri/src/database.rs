use crate::{
    directory::{get_dir_path, Dir},
    foc_error::Result,
};
use diesel::{r2d2::ConnectionManager, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use once_cell::sync::OnceCell;
use r2d2::PooledConnection;

pub static DATABASE_INSTANCE: OnceCell<r2d2::Pool<ConnectionManager<SqliteConnection>>> =
    OnceCell::new();

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations/");

pub async fn init_database() -> Result<()> {
    let database_dir = get_dir_path(Dir::Data)?.join("focular.db");
    let database_path = database_dir.to_str().unwrap();

    let manager: ConnectionManager<SqliteConnection> =
        diesel::r2d2::ConnectionManager::new(database_path);
    let connection_pool = r2d2::Pool::builder().max_size(32).build(manager).unwrap();
    run_migrations(connection_pool.get()?).unwrap();

    let _ = DATABASE_INSTANCE.set(connection_pool);
    Ok(())
}

fn run_migrations(
    mut connection: PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}
