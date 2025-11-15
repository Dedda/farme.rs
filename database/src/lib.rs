use derive_more::From;
use diesel::PgConnection;
use diesel::result::Error;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use rocket_sync_db_pools::database;
use std::fmt::Display;
use std::str::FromStr;

pub mod farm;
pub mod location;
pub mod schema;
pub mod user;

#[derive(Debug, From)]
pub struct DatabaseError(pub String);

impl From<Error> for DatabaseError {
    fn from(value: Error) -> Self {
        Self(value.to_string())
    }
}

pub type DbResult<T> = Result<T, DatabaseError>;

trait FromDbString {
    fn from_str(db_str: String) -> DbResult<Self>
    where
        Self: Sized;
}

impl<T, E> FromDbString for T
where
    T: FromStr<Err = E> + Sized,
    E: Display,
{
    fn from_str(db_str: String) -> DbResult<Self>
    where
        Self: Sized,
    {
        db_str
            .parse()
            .map_err(|err| format!("Unable to parse string: {}", err).into())
    }
}

trait ToDbString {
    fn to_db_string(&self) -> DbResult<String>;
}

impl<T> ToDbString for T
where
    T: ToString,
{
    fn to_db_string(&self) -> DbResult<String> {
        Ok(self.to_string())
    }
}

#[database("pgfarm")]
pub struct FarmDB(PgConnection);

pub fn run_migrations(conn: &mut PgConnection) {
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    conn.run_pending_migrations(MIGRATIONS)
        .expect("diesel migrations");
}
