use crate::api::Result as ApiResult;
use database::FarmDB;
use database::user::{User, check_login, username_by_identity, FarmOwnerStatus};
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use lazy_static::lazy_static;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Status;
use rocket::outcome::{IntoOutcome, try_outcome};
use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::Json;
use rocket::{Request, Response, async_trait, post};
use serde::{Deserialize, Serialize};

#[cfg(not(test))]
use std::env;
use crate::api::v1::error::ApiError::WrongCredentials;

#[cfg(not(test))]
lazy_static! {
    static ref JWT_SECRET: String = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");
}

#[cfg(test)]
lazy_static! {
    static ref JWT_SECRET: String = String::from("testsecret");
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![login_jwt]
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginCredentials {
    pub identity: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Claims {
    pub subject_id: String,
    exp: usize,
}

fn create_jwt(username: String) -> Result<String, jsonwebtoken::errors::Error> {
    let username = username.trim().to_lowercase();
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(15))
        .expect("invalid timestamp")
        .timestamp();

    let claims = Claims {
        subject_id: username,
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);
    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
}

#[post("/login-jwt", data = "<credentials>")]
pub async fn login_jwt(
    db: FarmDB,
    credentials: Json<LoginCredentials>,
) -> ApiResult<Option<String>> {
    let identity = credentials.identity.trim().to_lowercase();
    let username = if let Some(name) = username_by_identity(&db, identity).await.ok().flatten() {
        name
    } else {
        return Err(WrongCredentials);
    };
    if !check_login(&db, username.clone(), credentials.0.password).await? {
        return Err(WrongCredentials);
    };

    if let Ok(token) = create_jwt(username) {
        Ok(Some(token))
    } else {
        Ok(None)
    }
}

pub struct UserLogin(pub User);

#[async_trait]
impl<'r> FromRequest<'r> for UserLogin {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let db = try_outcome!(request.guard::<FarmDB>().await);
        let username = request
            .headers()
            .get_one("Authorization")
            .and_then(username_from_valid_jwt_token);
        if let Some(username) = username {
            database::user::by_username(&db, username)
                .await
                .ok()
                .flatten().map(UserLogin)
                .or_forward(Status::Unauthorized)
        } else {
            Outcome::Forward(Status::Unauthorized)
        }
    }
}

pub struct FarmOwner(pub User);

#[async_trait]
impl<'r> FromRequest<'r> for FarmOwner {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = try_outcome!(request.guard::<UserLogin>().await).0;
        if user.farmowner == FarmOwnerStatus::YES {
            Outcome::Success(FarmOwner(user))
        } else {
            Outcome::Forward(Status::Forbidden)
        }
    }
}

fn username_from_valid_jwt_token(jwt_token: &str) -> Option<String> {
    let decoding_key = DecodingKey::from_secret(JWT_SECRET.as_bytes());
    let validation = Validation::new(Algorithm::HS512);
    decode::<Claims>(jwt_token, &decoding_key, &validation)
        .ok()
        .map(|token_data| token_data.claims)
        .filter(|claims| claims.exp >= Utc::now().timestamp() as usize)
        .map(|claims| claims.subject_id)
        .map(|s| s.to_lowercase())
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
        if let Outcome::Success(user) = UserLogin::from_request(req).await
            && let Ok(token) = create_jwt(user.0.username)
        {
            res.set_raw_header("Authorization", token);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::create_jwt;

    #[test]
    fn jwt_creation() {
        let token = create_jwt(String::from("testuser")).expect("failed to create JWT");
        assert!(!token.is_empty());
    }
}
