use crate::data::user::{self, NewUser, User};
use crate::data::FarmDB;
use crate::ident::LoginCredentials;
use crate::mailing::{EmailError, EmailService, EmailValidationRequest};
use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{post, response, routes, Request, Response, State};
use serde::Deserialize;
use std::collections::HashMap;
use std::io::Cursor;

pub fn routes() -> Vec<rocket::Route> {
    routes![login_jwt, create_user]
}

#[derive(Deserialize)]
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

impl From<EmailError> for NewUserError {
    fn from(_value: EmailError) -> Self {
        Self {
            message: "Email error".to_string(),
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
async fn create_user(db: FarmDB, email: &State<EmailService>, user: Json<NewApiUser>) -> Result<Json<User>, NewUserError> {
    user.validate()?;
    let password = user.password.clone();
    let user = user::create_user(db, user.0.into(), password).await?;
    let validation_request = EmailValidationRequest::new(String::new(), &user);
    email.send_validation_request(validation_request)?;
    Ok(Json(user))
}
