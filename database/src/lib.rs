use diesel::PgConnection;
use diesel::result::Error;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rocket_sync_db_pools::database;

pub mod schema;
pub mod user;
pub mod location;
pub mod farm;

#[derive(Debug)]
pub struct DatabaseError(pub String);

impl From<Error> for DatabaseError {
    fn from(value: Error) -> Self {
        Self(value.to_string())
    }
}

pub type DbResult<T> = Result<T, DatabaseError>;

#[database("pgfarm")]
pub struct FarmDB(PgConnection);

pub fn run_migrations(conn: &mut PgConnection) {
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    conn.run_pending_migrations(MIGRATIONS).expect("diesel migrations");
}