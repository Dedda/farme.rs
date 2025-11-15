use std::fmt::Display;
use std::str::FromStr;
use derive_more::From;
use diesel::PgConnection;
use diesel::result::Error;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rocket_sync_db_pools::database;

pub mod schema;
pub mod user;
pub mod location;
pub mod farm;

#[derive(Debug, From)]
pub struct DatabaseError(pub String);

impl From<Error> for DatabaseError {
    fn from(value: Error) -> Self {
        Self(value.to_string())
    }
}

pub type DbResult<T> = Result<T, DatabaseError>;

trait FromDbString {
    fn from_str(db_str: String) -> DbResult<Self> where Self: Sized;
}

impl<T, E> FromDbString for T where T: FromStr<Err = E> + Sized, E: Display {
    fn from_str(db_str: String) -> DbResult<Self>
    where
        Self: Sized
    {
        db_str.parse().map_err(|err| format!("Unable to parse string: {}", err).into())
    }
}

#[database("pgfarm")]
pub struct FarmDB(PgConnection);

pub fn run_migrations(conn: &mut PgConnection) {
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    conn.run_pending_migrations(MIGRATIONS).expect("diesel migrations");
}