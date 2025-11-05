use crate::api::v1::error::{ApiError, ValidationError as ValidationApiError};
use crate::api::v1::ident::{LoginCredentials, UserLogin};
use crate::api::Result as ApiResult;
use database::user::{self, check_login, username_by_identity, DefaultUserChange, FarmOwnerStatus, NewUser, User};
use database::FarmDB;
use crate::validation::{
    EmailValidator, PasswordValidator, StringLengthCriteria, StringValidator, Validator,
};
use base64::engine::general_purpose::URL_SAFE;
use base64::Engine;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{get, post, routes};
use serde::Deserialize;
use std::collections::HashMap;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        login_jwt,
        create_user,
        current_user,
        no_current_user,
        change_user,
        change_password,
        delete_current_user,
        request_farm_admin_status,
    ]
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
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
    pub fn sanitize(&mut self) {
        self.firstname = self.firstname.trim().to_string();
        self.lastname = self.lastname.trim().to_string();
        self.username = self.username.trim().to_lowercase();
        self.email = self.email.trim().to_lowercase();
    }

    pub fn validate(&self) -> Result<(), ValidationApiError> {
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
            Err(ValidationApiError::for_fields(errors))
        }
    }

    fn validate_username(&self) -> Option<Vec<String>> {
        let mut validator = StringValidator::new();
        validator.add_criteria(StringLengthCriteria::min(3));
        if let Err(err) = validator.validate(&self.password) {
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
        EmailValidator
            .validate(&self.email)
            .err()
            .map(|err| err.messages)
    }

    fn validate_password(&self) -> Option<Vec<String>> {
        validate_password(&self.password)
    }
}

fn validate_password(password: &str) -> Option<Vec<String>> {
    PasswordValidator
        .validate(password)
        .err()
        .map(|err| err.messages)
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ApiFarmOwnerStatus {
    No,
    Yes,
    Requested,
}

impl From<FarmOwnerStatus> for ApiFarmOwnerStatus {
    fn from(value: FarmOwnerStatus) -> Self {
        match value {
            FarmOwnerStatus::NO => ApiFarmOwnerStatus::No,
            FarmOwnerStatus::YES => ApiFarmOwnerStatus::Yes,
            FarmOwnerStatus::REQUESTED => ApiFarmOwnerStatus::Requested,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ApiUser {
    pub id: String,
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub email: String,
    pub farmowner: ApiFarmOwnerStatus,
}

impl From<User> for ApiUser {
    fn from(u: User) -> Self {
        Self {
            id: URL_SAFE.encode(u.ext_id),
            firstname: u.firstname,
            lastname: u.lastname,
            username: u.username,
            email: u.email,
            farmowner: ApiFarmOwnerStatus::from(u.farmowner),
        }
    }
}

#[post("/login-jwt", data = "<credentials>")]
async fn login_jwt(db: FarmDB, credentials: Json<LoginCredentials>) -> ApiResult<Option<String>> {
    crate::api::v1::ident::login_jwt(db, credentials).await
}

#[post("/create", data = "<user>")]
async fn create_user(db: FarmDB, user: Json<NewApiUser>) -> ApiResult<Json<ApiUser>> {
    let mut user = user.into_inner();
    user.sanitize();
    user.validate()?;
    let password = user.password.clone();
    let user = user::create_user(&db, user.into(), password).await?;
    Ok(Json(user.into()))
}

#[post("/change", data = "<changed>")]
async fn change_user(db: FarmDB, user: UserLogin, changed: Json<NewApiUser>) -> ApiResult<()> {
    let user = user.0;
    let mut changed = changed.into_inner();
    if !check_login(&db, changed.username.clone(), changed.password.clone()).await? {
        return Err(ApiError::WrongCredentials);
    }
    if user.username.ne(&changed.username) {
        return Err(ValidationApiError::new(
            "Cannot change user".to_string(),
            HashMap::from([(
                "username".to_string(),
                vec!["May not change username".to_string()],
            )]),
        )
        .into());
    }
    changed.sanitize();
    changed.validate()?;
    if user.email.ne(&changed.email) {
        check_email_availability(&db, user, &changed).await?;
    }
    user::default_user_change(
        &db,
        DefaultUserChange {
            firstname: changed.firstname,
            lastname: changed.lastname,
            username: changed.username,
            email: changed.email,
        },
    )
    .await?;
    Ok(())
}

async fn check_email_availability(db: &FarmDB, user: User, changed: &NewApiUser) -> ApiResult<()> {
    if let Some(found) = username_by_identity(db, changed.email.clone()).await?
        && !user.username.eq(&found)
    {
        return Err(ValidationApiError::for_fields(HashMap::from([(
            "email".to_string(),
            vec!["Email already in use".to_string()],
        )]))
        .into());
    }
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct PasswordChangeRequest {
    pub old_password: String,
    pub new_password: String,
}

#[post("/change-password", data = "<change_request>")]
async fn change_password(
    db: FarmDB,
    user: UserLogin,
    change_request: Json<PasswordChangeRequest>,
) -> ApiResult<()> {
    let user = user.0;
    if let Some(errors) = validate_password(&change_request.new_password) {
        return Err(ValidationApiError::for_fields(HashMap::from([(
            "password".to_string(),
            errors,
        )]))
        .into());
    }
    check_login(
        &db,
        user.username.clone(),
        change_request.old_password.clone(),
    )
    .await?;
    user::password_change(&db, user.username, change_request.new_password.clone()).await?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct DeleteAuth {
    password: String,
}

#[post("/delete-current", data = "<delete_auth>")]
async fn delete_current_user(db: FarmDB, user: UserLogin, delete_auth: Json<DeleteAuth>) -> ApiResult<()> {
    let user = user.0;
    if !check_login(&db, user.username, delete_auth.into_inner().password).await? {
        return Err(ApiError::WrongCredentials)
    }
    user::delete(&db, user.id).await?;
    Ok(())
}

#[get("/current-user", format = "json")]
async fn current_user(user: UserLogin) -> Option<Json<ApiUser>> {
    let user = user.0;
    Some(Json(ApiUser::from(user)))
}

#[get("/current-user", format = "json", rank = 2)]
async fn no_current_user() -> Status {
    Status::Unauthorized
}

#[post("/request-admin")]
async fn request_farm_admin_status(db: FarmDB, user: UserLogin) -> ApiResult<()> {
    let user = user.0;
    user::request_farm_admin_status(&db, user.id).await?;
    user::make_farmowner(&db, user.id).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::api::v1::test_utils::{create_untracked_client, get_current_user, login_user, WithAuthorization};
    use crate::api::v1::users::{ApiFarmOwnerStatus, NewApiUser};
    use crate::api::v1::users::PasswordChangeRequest;
    use crate::api::v1::users::{ApiUser, DeleteAuth};
    use database::user;
    use database::user::check_login;
    use database::FarmDB;
    use rocket::http::{ContentType, Status};

    #[test]
    fn sanitize_new_api_user() {
        let mut user = NewApiUser {
            firstname: " Test ".to_string(),
            lastname: " User ".to_string(),
            username: " Testuser ".to_string(),
            email: " Test@test.com ".to_string(),
            password: "".to_string(),
        };
        user.sanitize();
        assert_eq!(
            user,
            NewApiUser {
                firstname: "Test".to_string(),
                lastname: "User".to_string(),
                username: "testuser".to_string(),
                email: "test@test.com".to_string(),
                password: "".to_string(),
            }
        );
    }

    #[tokio::test]
    async fn user_api_crud() {
        let client = create_untracked_client().await;

        let new_api_user = NewApiUser {
            firstname: "Firstuser".to_string(),
            lastname: "Lastuser".to_string(),
            username: "testusername".to_string(),
            email: "test@test.com".to_string(),
            password: "na9e8#aKsd".to_string(),
        };
        // create user via API
        let req = client.post("/api/v1/users/create");
        let response = req
            .body(serde_json::to_string(&new_api_user).expect("failed to serialize user"))
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
        let user: ApiUser = response
            .into_json()
            .await
            .expect("failed to deserialize json");
        assert_eq!(user.firstname, "Firstuser");
        assert_eq!(user.lastname, "Lastuser");
        assert_eq!(user.username, "testusername");
        assert_eq!(user.email, "test@test.com");
        let db = FarmDB::get_one(client.rocket())
            .await
            .expect("failed to get db");
        let password = new_api_user.password.clone();
        let password_check = check_login(&db, new_api_user.username, new_api_user.password)
            .await
            .expect("failed to check user login");
        assert!(password_check);
        // login via API
        let token = login_user(&client, &user.username, &password).await;
        // change user via API
        let changed_user = NewApiUser {
            firstname: "Firsty".to_string(),
            lastname: "Lasty".to_string(),
            username: "testusername".to_string(),
            email: "test123@test456.com".to_string(),
            password: password.clone(),
        };
        let req = client.post("/api/v1/users/change");
        let response = req
            .body(serde_json::to_string(&changed_user).expect("failed to serialize change user"))
            .auth(&token)
            .dispatch()
            .await;
        let token = response
            .headers()
            .get_one("Authorization")
            .expect("no authorization header")
            .to_string();
        assert_eq!(response.status(), Status::Ok);
        // check user changes
        let changed = user::by_username(&db, "testusername".to_string())
            .await
            .expect("failed to get user by username")
            .expect("failed to find changed user by username");
        assert_eq!(changed.firstname, "Firsty");
        assert_eq!(changed.lastname, "Lasty");
        assert_eq!(changed.username, "testusername");
        assert_eq!(changed.email, "test123@test456.com");
        // change password via API
        let req = client.post("/api/v1/users/change-password");
        let response = req
            .body(
                serde_json::to_string(&PasswordChangeRequest {
                    old_password: password,
                    new_password: "na9e8#aKsO".to_string(),
                })
                .expect("failed to serialize password change"),
            )
            .auth(&token)
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Ok);
        let password_check = check_login(&db, "testusername".to_string(), "na9e8#aKsO".to_string())
            .await
            .expect("failed to check user login");
        assert!(password_check);
        // request farm admin status
        let req = client.post("/api/v1/users/request-admin");
        let response = req
            .auth(&token)
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Ok);
        let current_user = get_current_user(&client, token.clone()).await;
        assert_eq!(current_user.farmowner, ApiFarmOwnerStatus::Yes);
        // delete created user
        let req = client.post("/api/v1/users/delete-current");
        let response = req
            .body(
                serde_json::to_string(&DeleteAuth {
                    password: "na9e8#aKsO".to_string(),
                })
                .expect("failed to serialize auth"),
            )
            .auth(&token)
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Ok);
    }
}
