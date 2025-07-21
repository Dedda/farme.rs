use derive_more::From;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::{Request, Response};
use serde::Serialize;
use std::collections::HashMap;
use std::io::Cursor;

#[derive(From)]
pub enum ApiError {
    Database(diesel::result::Error),
    WrongCredentials,
    Validation(ValidationError),
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _request: &'r Request<'_>) -> rocket::response::Result<'static> {
        match self {
            ApiError::Database(_error) => Response::build().status(Status::InternalServerError).ok(),
            ApiError::WrongCredentials => Response::build().status(Status::Unauthorized).ok(),
            ApiError::Validation(validation) => {
                let body = serde_json::to_string(&validation).expect("validation");
                Response::build().status(Status::BadRequest).sized_body(body.len(), Cursor::new(body)).ok()
            },
        }
    }
}

#[derive(Serialize)]
pub struct ValidationError {
    message: String,
    invalid_fields: HashMap<String, Vec<String>>,
}

impl ValidationError {
    pub fn new(message: String, invalid_fields: HashMap<String, Vec<String>>) -> Self {
        Self {
            message,
            invalid_fields,
        }
    }

    pub fn for_fields(fields: HashMap<String, Vec<String>>) -> Self {
        Self {
            message: String::from("Invalid data"),
            invalid_fields: fields,
        }
    }
}
