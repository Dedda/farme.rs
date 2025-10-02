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

#[cfg(test)]
pub mod test_utils {
    use rocket::local::asynchronous::Client;

    pub async fn create_untracked_client() -> Client {
        let rocket = crate::rocket()
            .ignite()
            .await
            .expect("cannot launch rocket");
        Client::untracked(rocket)
            .await
            .expect("valid rocket instance")
    }
}