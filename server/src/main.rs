mod api;
mod validation;

use crate::api::v1::ident;
use api::v1::ident::JwtRefreshFairing;
use dotenvy::dotenv;
use rocket::fs::FileServer;
use rocket::http::Method;
use rocket::{launch, routes, Build, Rocket};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use std::env;
use rocket::fairing::AdHoc;
use database::FarmDB;

#[launch]
fn rocket() -> Rocket<Build> {
    dotenv().ok();
    let r = Rocket::build()
        .attach(stage_database())
        .attach(make_cors())
        .attach(JwtRefreshFairing);
    api::v1::mount(r)
        .mount("/", webapp())
        .mount("/", routes![ident::login_jwt])
}

fn make_cors() -> Cors {
    CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_headers(AllowedHeaders::all())
        .expose_headers(["Authorization"].iter().map(ToString::to_string).collect())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Delete]
                .into_iter()
                .map(From::from)
                .collect()
        ).allow_credentials(true)
        .to_cors().unwrap()
}

fn webapp() -> FileServer {
    let webapp_path = if let Ok(path) = env::var("WEBAPP_PATH") {
        println!("WEBAPP_PATH set. Using webapp path: {}", &path);
        path
    } else {
        // Very ugly workaround for diverging working directories of `cargo run` and `cargo test`
        #[cfg(not(test))]
        let default_path = "web/dist/farmers/browser";
        #[cfg(test)]
        let default_path = "../web/dist/farmers/browser";
        println!("WEBAPP_PATH not set. Using default webapp path: {}", &default_path);
        default_path.to_string()
    };
    FileServer::from(webapp_path)
}

pub fn stage_database() -> AdHoc {
    AdHoc::on_ignite("Diesel Postgres Stage", |rocket| async {
        rocket.attach(FarmDB::fairing())
            .attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
    })
}

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    FarmDB::get_one(&rocket).await
        .expect("database connection")
        .run(|conn| {
            database::run_migrations(conn);
        }).await;

    rocket
}

