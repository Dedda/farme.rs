use std::io::Cursor;
use base64::{DecodeError, Engine};
use base64::engine::general_purpose::URL_SAFE;
use rocket::{async_trait, Request, Response};
use rocket::http::Status;
use rocket::request::{FromParam};
use rocket::response::Responder;
use uuid::Uuid;

pub struct ExtId(pub Uuid);

#[derive(Debug)]
pub struct ExtIdError(pub String);

impl From<DecodeError> for ExtIdError {
    fn from(err: DecodeError) -> Self {
        Self(format!("Error decoding url safe base64: {}", err))
    }
}

impl From<uuid::Error> for ExtIdError {
    fn from(err: uuid::Error) -> Self {
        Self(format!("Error decoding uuid: {}", err))
    }
}

#[async_trait]
impl<'r> FromParam<'r> for ExtId {
    type Error = ExtIdError;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        let decoded = URL_SAFE.decode(param)?;
        if let Some(slice) = decoded.as_slice().chunks(16).next() {
            let uuid = Uuid::from_slice(slice)?;
            Ok(Self(uuid))
        } else {
            Err(ExtIdError(format!("Illegal chunk size, expected 16 bytes: {}", param)))
        }
    }
}

impl<'r> Responder<'r, 'static> for ExtIdError {
    fn respond_to(self, _request: &'r Request<'_>) -> rocket::response::Result<'static> {
        Response::build().status(Status::BadRequest).sized_body(self.0.len(), Cursor::new(self.0)).ok()
    }
}