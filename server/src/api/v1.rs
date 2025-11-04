use rocket::{Build, Rocket};

mod farms;
mod users;
pub mod ident;
pub mod error;
mod types;

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount("/api/v1/farms", farms::routes())
        .mount("/api/v1/users", users::routes())
        .mount("/api/v1/ident", ident::routes())
}

#[cfg(test)]
pub mod test_utils {
    use crate::api::v1::ident::LoginCredentials;
    use crate::data::farm::Farm;
    use crate::data::user::{create_user, NewUser, User};
    use crate::data::FarmDB;
    use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
    use rocket::http::{ContentType, Header, Status};
    use rocket::local::asynchronous::Client;
    use crate::api::v1::users::ApiUser;

    pub async fn create_untracked_client() -> Client {
        let rocket = crate::rocket()
            .ignite()
            .await
            .expect("cannot launch rocket");
        Client::untracked(rocket)
            .await
            .expect("valid rocket instance")
    }

    pub async fn login_user(client: &Client, username: &str, password: &str) -> String {
        let req = client.post("/login-jwt");
        let response = req
            .body(
                serde_json::to_string(&LoginCredentials {
                    identity: username.to_string(),
                    password: password.to_string(),
                })
                    .expect("failed to serialize login credentials"),
            )
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Text));
        let token = response
            .into_string()
            .await
            .expect("cannot read login response body");
        assert!(!token.is_empty());
        token
    }

    pub fn new_test_user(test_name: &str) -> NewUser {
        NewUser {
            firstname: test_name.to_string(),
            lastname: test_name.to_string(),
            username: test_name.to_string(),
            email: format!("{}@test.com", test_name),
        }
    }

    pub async fn create_test_user(client: &Client, test_name: &str, password: &str) -> User {
        let db = FarmDB::get_one(client.rocket())
            .await
            .expect("failed to get db");
        let user = new_test_user(test_name);

        create_user(&db, user, password.to_string())
            .await
            .expect("failed to create user")
    }

    pub async fn get_newest_farm(client: &Client) -> Farm {
        let db = FarmDB::get_one(client.rocket())
            .await
            .expect("failed to get db");
        db.run(move |conn| {
            crate::schema::farms::table.select(Farm::as_select())
                .order_by(crate::schema::farms::created.desc())
                .first::<Farm>(conn)
                .expect("failed to get farm")
        }).await
    }

    pub async fn get_current_user(client: &Client, token: String) -> ApiUser {
        let response = client
            .get("/api/v1/users/current-user")
            .header(Header::new("Authorization", token))
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Ok);
        let user = response.into_json::<ApiUser>().await.expect("failed to deserialize user");
        user
    }
}