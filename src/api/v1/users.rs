use crate::data::user::{NewUser, User};
use crate::data::FarmDB;
use crate::ident::LoginCredentials;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{post, response, routes, Request, Response};
use serde::Deserialize;
use std::collections::HashMap;
use std::io::Cursor;

pub fn routes() -> Vec<rocket::Route> {
    routes![login_jwt, create_user]
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct NewApiUser {
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl From<NewApiUser> for NewUser {
    fn from(value: NewApiUser) -> Self {
        Self {
            firstname: value.firstname,
            lastname: value.lastname,
            username: value.username,
            email: value.email,
        }
    }
}

impl NewApiUser {
    pub fn validate(&self) -> Result<(), NewUserError> {
        Ok(())
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct NewUserError {
    message: String,
    invalid_fields: HashMap<String, String>,
    status: Status,
}

impl<'r, 'a: 'r> Responder<'r, 'a> for NewUserError {
    fn respond_to(self, _request: &'r Request<'_>) -> response::Result<'a> {
        Ok(Response::build()
            .status(self.status)
            .sized_body(self.message.len(), Cursor::new(self.message))
            .finalize())
    }
}

impl From<diesel::result::Error> for NewUserError {
    fn from(_value: diesel::result::Error) -> Self {
        Self {
            message: "Databse error".to_string(),
            invalid_fields: HashMap::new(),
            status: Status::InternalServerError,
        }
    }
}

#[post("/login-jwt", data = "<credentials>")]
async fn login_jwt(db: FarmDB, credentials: Json<LoginCredentials>) -> crate::api::Result<Option<String>> {
    crate::ident::login_jwt(db, credentials).await
}

#[post("/create", data = "<user>")]
async fn create_user(db: FarmDB, user: Json<NewApiUser>) -> Result<Json<User>, NewUserError> {
    user.validate()?;
    let password = user.password.clone();
    let user = crate::data::user::create_user(db, user.0.into(), password).await?;
    Ok(Json(user))
}

#[cfg(test)]
mod tests {
    use crate::{api::v1::users::NewApiUser, data::user::User};
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;

    #[test]
    fn crud_user() {
        let rocket = crate::rocket();
        let mut client = Client::tracked(rocket).expect("valid rocket instance");
        let new_api_user = NewApiUser {
            firstname: "Firstuser".to_string(),
            lastname: "Lastuser".to_string(),
            username: "testusername".to_string(),
            email: "test@test.com".to_string(),
            password: "na9e8#aKsd".to_string(),
        };
        let req = client.post("/api/v1/users/create");
        let response = req.body(serde_json::to_string(&new_api_user).expect("failed to serialize user")).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
        let user: User = response.into_json().expect("failed to deserialize json");
        assert_eq!(user.firstname, "Firstuser");
        assert_eq!(user.lastname, "Lastuser");
        assert_eq!(user.username, "testusername");
        assert_eq!(user.email, "test@test.com");
    }
}