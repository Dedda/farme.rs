use rocket::{Build, Rocket};

mod farms;
mod users;

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount("/api/v1/farms", farms::routes())
        .mount("/api/v1/users", users::routes())
}
