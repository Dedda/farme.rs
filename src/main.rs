pub mod data;
pub mod schema;
mod api;
mod ident;
mod validation;

use dotenv::dotenv;
use rocket::http::Method;
use rocket::{launch, routes, Build, Rocket};
use rocket_cors::{AllowedOrigins, Cors, CorsOptions};

#[launch]
fn rocket() -> Rocket<Build> {
    dotenv().ok();
    let r = Rocket::build()
        .attach(data::stage())
        .attach(make_cors());
    api::v1::mount(r)
        .mount("/", routes![ident::login_jwt])
}

fn make_cors() -> Cors {
    CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post]
                .into_iter()
                .map(From::from)
                .collect()
        ).allow_credentials(true)
        .to_cors().unwrap()

}