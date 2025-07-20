use rocket::{Build, Rocket};

mod farms;
mod users;
pub mod ident;
pub mod error;

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount("/api/v1/farms", farms::routes())
        .mount("/api/v1/users", users::routes())
        .mount("/api/v1/ident", ident::routes())
}

