use diesel::{PgConnection, r2d2::{self, ConnectionManager}};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::info;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub type Connection = PgConnection;
pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;

/// Creates a new connection pool to the database using the given connection url. 
pub fn init_db_pool(url: &str) -> Pool {
    info!("Configuring database pool");

    let manager = ConnectionManager::<Connection>::new(url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    pool
}

/// Runs the migration scripts to create, update or delete database related content.
pub fn  run_migration(conn: &mut Connection) {
    info!("Running migrations");

    conn.run_pending_migrations(MIGRATIONS).unwrap();
}