use crate::data::user::{NewUser, User};
use crate::data::FarmDB;
use crate::ident::LoginCredentials;
use crate::validation::{RegexValidator, RequiredCharacterGroupCriteria, StringCriteria, StringLengthCriteria, StringValidator, Validator};
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
        let mut errors = HashMap::new();
        if let Some(err) = self.validate_password() {
            errors.insert("password".to_string(), err);
        }
        if let Some(err) = self.validate_email() {
            errors.insert("email".to_string(), err);
        }
        if let Some(err) = self.validate_username() {
            errors.insert("username".to_string(), err);
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(NewUserError {
                message: "validation error".to_string(),
                invalid_fields: errors,
                status: Status::BadRequest,
            })
        }
    }

    fn validate_username(&self) -> Option<Vec<String>> {
        let mut validator = StringValidator::new();
        validator.add_criteria(StringLengthCriteria::min(3));
        if let Err(err) =  validator.validate(&self.password) {
            return Some(err.messages);
        }
        if self.username.chars().any(|c| !c.is_alphanumeric()) {
            return Some(vec!["Only letters and numbers allowed".to_string()]);
        }
        if !self.username.chars().next().unwrap().is_alphabetic() {
            return Some(vec!["Has to begin with a letter".to_string()]);
        }
        None
    }

    fn validate_email(&self) -> Option<Vec<String>> {
        let validatpr = RegexValidator::new(include_str!("email_regex.txt")).expect("Cannot parse email regex");
        if let Err(err) = validatpr.validate(&self.email) {
            Some(vec![err])
        } else {
            None
        }
    }

    fn validate_password(&self) -> Option<Vec<String>> {
        let mut validator = StringValidator::new();
        validator.add_criteria(StringLengthCriteria::min(8));
        validator.add_criteria(RequiredCharacterGroupCriteria::range('a', 'z'));
        validator.add_criteria(RequiredCharacterGroupCriteria::range('A', 'Z'));
        validator.add_criteria(RequiredCharacterGroupCriteria::range('0', '9'));
        validator.add_criteria(RequiredCharacterGroupCriteria::new("!?.-_#$&".chars().collect()));
        if let Err(err) =  validator.validate(&self.password) {
            Some(err.messages)
        } else {
            None
        }
    }
}

#[derive(Serialize)]
struct ApiUser {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub email: String,
}

impl From<User> for ApiUser {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            firstname: u.firstname,
            lastname: u.lastname,
            username: u.username,
            email: u.email,
        }
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct NewUserError {
    message: String,
    invalid_fields: HashMap<String, Vec<String>>,
    status: Status,
}

impl<'r, 'a: 'r> Responder<'r, 'a> for NewUserError {
    fn respond_to(self, _request: &'r Request<'_>) -> response::Result<'a> {
        let fields = serde_json::to_string(&self.invalid_fields).unwrap();
        Ok(Response::build()
            .status(self.status)
            .sized_body(fields.len(), Cursor::new(fields))
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
async fn create_user(db: FarmDB, user: Json<NewApiUser>) -> Result<Json<ApiUser>, NewUserError> {
    user.validate()?;
    let password = user.password.clone();
    let user = crate::data::user::create_user(db, user.0.into(), password).await?;
    Ok(Json(user.into()))
}

#[cfg(test)]
mod tests {
    use crate::data::FarmDB;
    use crate::{api::v1::users::NewApiUser, data::user::User};
    use diesel::{ExpressionMethods, RunQueryDsl};
    use rocket::http::{ContentType, Status};
    use rocket::local::asynchronous::Client;
    use crate::data::user::check_login;

    #[tokio::test]
    async fn create_user() {
        let rocket = crate::rocket()
            .ignite()
            .await
            .expect("cannot launch rocket");
        let client = Client::untracked(rocket)
            .await
            .expect("valid rocket instance");
        let new_api_user = NewApiUser {
            firstname: "Firstuser".to_string(),
            lastname: "Lastuser".to_string(),
            username: "testusername".to_string(),
            email: "test@test.com".to_string(),
            password: "na9e8#aKsd".to_string(),
        };
        let req = client.post("/api/v1/users/create");
        let response = req
            .body(serde_json::to_string(&new_api_user).expect("failed to serialize user"))
            .dispatch().await;
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
        let user: User = response.into_json().await.expect("failed to deserialize json");
        assert_eq!(user.firstname, "Firstuser");
        assert_eq!(user.lastname, "Lastuser");
        assert_eq!(user.username, "testusername");
        assert_eq!(user.email, "test@test.com");
        let id = user.id;
        let db = FarmDB::get_one(client.rocket()).await.expect("failed to get db");
        let password_check = check_login(&db, new_api_user.username, new_api_user.password)
            .await
            .expect("failed to check user login");
        assert!(password_check);
        // Delete created user
        db.run(move |conn| {
            diesel::delete(crate::schema::users::table)
                .filter(crate::schema::users::id.eq(id))
                .execute(conn)
                .expect("Cannot delete user");
        }).await;
    }
}