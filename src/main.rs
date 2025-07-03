pub mod data;
pub mod schema;
mod api;
mod ident;

use rocket::{launch, Build, Rocket};
use rocket_cors::{AllowedOrigins, Cors, CorsOptions};

#[launch]
fn rocket() -> Rocket<Build> {
    let r = Rocket::build()
        .attach(data::stage())
        .attach(make_cors());
    api::v1::mount(r)
}

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:4200",
    ]);
    CorsOptions {
        allowed_origins,
        ..Default::default()
    }.to_cors().unwrap()
}