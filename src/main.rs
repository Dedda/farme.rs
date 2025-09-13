pub mod data;
pub mod schema;
mod api;
mod validation;

use dotenvy::dotenv;
use api::v1::ident::JwtRefreshFairing;
use rocket::http::Method;
use rocket::{launch, routes, Build, Rocket};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use api::v1::ident;

#[launch]
fn rocket() -> Rocket<Build> {
    dotenv().ok();
    let r = Rocket::build()
        .attach(data::stage())
        .attach(make_cors())
        .attach(JwtRefreshFairing);
    api::v1::mount(r)
        .mount("/", routes![ident::login_jwt])
}

fn make_cors() -> Cors {
    CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_headers(AllowedHeaders::all())
        .expose_headers(["Authorization"].iter().map(ToString::to_string).collect())
        .allowed_methods(
            vec![Method::Get, Method::Post]
                .into_iter()
                .map(From::from)
                .collect()
        ).allow_credentials(true)
        .to_cors().unwrap()

}