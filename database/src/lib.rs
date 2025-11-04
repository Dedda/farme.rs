use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rocket_sync_db_pools::database;

pub mod schema;
pub mod user;
pub mod location;
pub mod farm;

#[database("pgfarm")]
pub struct FarmDB(PgConnection);

pub fn run_migrations(conn: &mut PgConnection) {
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    conn.run_pending_migrations(MIGRATIONS).expect("diesel migrations");
}