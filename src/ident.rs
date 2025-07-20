use crate::data::user::{check_login, username_by_identity, User};
use crate::data::FarmDB;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Status;
use rocket::outcome::{try_outcome, IntoOutcome};
use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::Json;
use rocket::{async_trait, post, Request, Response};
use serde::{Deserialize, Serialize};

#[cfg(not(test))]
use std::env;
#[cfg(not(test))]
lazy_static! {
    static ref JWT_SECRET: String = {
        env::var("JWT_SECRET").expect("JWT_SECRET must be set.")
    };
}

#[cfg(test)]
lazy_static! {
    static ref JWT_SECRET: String = String::from("testsecret");
}

#[derive(Serialize, Deserialize)]
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
    let username = username.trim().to_lowercase();
    let expiration = Utc::now().checked_add_signed(Duration::minutes(15)).expect("invalid timestamp").timestamp();

    let claims = Claims {
        subject_id: username,
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET.as_bytes()))
}

#[post("/login-jwt", data = "<credentials>")]
pub async fn login_jwt(db: FarmDB, credentials: Json<LoginCredentials>) -> crate::api::Result<Option<String>> {
    let identity = credentials.identity.trim().to_lowercase();
    let username = if let Some(name) = username_by_identity(&db, identity).await.ok().flatten() {
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

#[async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let db = try_outcome!(request.guard::<FarmDB>().await);
        let username = request.headers()
            .get_one("Authorization")
            .and_then(|header|  decode::<Claims>(header, &DecodingKey::from_secret(JWT_SECRET.as_bytes()), &Validation::new(Algorithm::HS512)).ok())
            .map(|token_data| token_data.claims)
            .filter(|claims| claims.exp >= Utc::now().timestamp() as usize)
            .map(|claims| claims.subject_id)
            .map(|s| s.to_lowercase());
        let auth_header = request.headers()
            .get_one("Authorization");
        dbg!(&auth_header);
        dbg!(&username);
        if let Some(username) = username {
            crate::data::user::by_username(&db, username).await.ok()
                .flatten()
                .or_forward(Status::Unauthorized)
        } else {
            Outcome::Forward(Status::Unauthorized)
        }
    }
}

pub struct JwtRefreshFairing;

#[async_trait]
impl Fairing for JwtRefreshFairing {

    fn info(&self) -> Info {
        Info {
            name: "JWT Refresh Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        if let Outcome::Success(user) =  User::from_request(req).await {
            if let Ok(token) = create_jwt(user.username) {
                res.set_raw_header("Authorization", token);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::create_jwt;

    #[test]
    fn jwt_creation() {
        let token = create_jwt(String::from("testuser"))
            .expect("failed to create JWT");
        assert!(!token.is_empty());
    }
}