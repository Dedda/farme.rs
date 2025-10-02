use rocket::fairing::AdHoc;
use rocket::{Build, Rocket};
use rocket_sync_db_pools::database;
use rocket_sync_db_pools::diesel;

pub mod farm;
pub mod user;

#[database("pgfarm")]
pub struct FarmDB(diesel::PgConnection);

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    FarmDB::get_one(&rocket).await
        .expect("database connection")
        .run(|conn| { conn.run_pending_migrations(MIGRATIONS).expect("diesel migrations"); })
        .await;

    rocket
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel Postgres Stage", |rocket| async {
        rocket.attach(FarmDB::fairing())
            .attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
    })
}