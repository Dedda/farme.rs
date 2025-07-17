use std::env;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use rocket::post;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::data::user::{check_login, username_by_identity};
use crate::data::FarmDB;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginCredentials {
    pub identity: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub subject_id: String,
    exp: usize,
}

fn create_jwt(username: String) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set."); // 👈 New!

    let expiration = Utc::now().checked_add_signed(Duration::seconds(60)).expect("invalid timestamp").timestamp();

    let claims = Claims {
        subject_id: username,
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(secret.as_bytes()))
}

#[post("/login-jwt", data = "<credentials>")]
pub async fn login_jwt(db: FarmDB, credentials: Json<LoginCredentials>) -> crate::api::Result<Option<String>> {
    let username = if let Some(name) = username_by_identity(&db, credentials.0.identity.clone()).await? {
        name
    } else {
        return Ok(None)
    };
    if !check_login(&db, username.clone(), credentials.0.password).await? {
        return Ok(None)
    };

    if let Ok(token) = create_jwt(username) {
        Ok(Some(token))
    } else {
        Ok(None)
    }
}