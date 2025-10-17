use crate::data::farm::Farm;
use crate::data::FarmDB;
use crate::schema::{farm_admins, users};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Selectable, Identifiable, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub sysadmin: i32,
    pub farmowner: i32,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewUser {
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub email: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = users)]
pub struct DefaultUserChange {
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub email: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct InsertableUser {
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Identifiable, Queryable, Associations)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Farm))]
pub struct FarmAdmin {
    pub id: i32,
    pub user_id: i32,
    pub farm_id: i32,
}

pub async fn create_user(db: &FarmDB, user: NewUser, password: String) -> QueryResult<User> {
    let salt = SaltString::generate(&mut OsRng);
    let password = Argon2::default().hash_password(password.as_bytes(), &salt).unwrap().to_string();
    let user = InsertableUser {
        firstname: user.firstname,
        lastname: user.lastname,
        username: user.username,
        email: user.email,
        password,
    };
    db.run(move |conn| {
        let new_id: i32 = diesel::insert_into(users::table)
            .values(user)
            .returning(users::id)
            .get_result(conn)?;
        let user: User = users::table
            .select(User::as_select())
            .find(new_id)
            .first(conn)
            .optional()?
            .expect("");
        Ok(user)
    }).await
}

pub async fn default_user_change(db: &FarmDB, user: DefaultUserChange) -> QueryResult<()> {
    db.run(move |conn| {
        let username = user.username.clone();
        diesel::update(users::table)
            .filter(users::username.eq(&username))
            .set(user)
            .execute(conn)
    }).await?;
    Ok(())
}

pub async fn password_change(db: &FarmDB, username: String, password: String) -> QueryResult<()> {
    let salt = SaltString::generate(&mut OsRng);
    let password = Argon2::default().hash_password(password.as_bytes(), &salt).unwrap().to_string();
    db.run(move |conn| {
        diesel::update(users::table)
            .filter(users::username.eq(username))
            .set(users::password.eq(&password))
            .execute(conn)
    }).await?;
    Ok(())
}

pub async fn check_login(db: &FarmDB, username: String, password: String) -> QueryResult<bool> {
    if let Some(hash) = db.run(move |conn| {
        users::table
            .select(users::password)
            .filter(users::username.eq(username))
            .first::<String>(conn)
            .optional()
    }).await? {
        let hash = argon2::password_hash::PasswordHash::new(hash.as_str()).unwrap();
        Ok(Argon2::default().verify_password(password.as_bytes(), &hash).is_ok())
    } else {
        Ok(false)
    }
}

pub async fn username_by_identity(db: &FarmDB, identity: String) -> QueryResult<Option<String>> {
    let username = if identity.contains("@") {
        let found: Option<String> = db.run(move |conn| {
            users::table
                .select(users::username)
                .filter(users::email.eq(identity))
                .first::<String>(conn)
                .optional()
        }).await?;
        if let Some(found) = found {
            found
        } else {
            return Ok(None);
        }
    } else {
        identity
    };
    Ok(Some(username))
}

pub async fn by_username(db: &FarmDB, username: String) -> QueryResult<Option<User>> {
    db.run(move |conn| {
    users::table
        .select(User::as_select())
        .filter(users::username.eq(username))
        .first(conn)
        .optional()
    }).await
}

pub async fn make_farmowner(db: &FarmDB, user_id: i32) -> QueryResult<()> {
    db.run(move |conn| {
        diesel::update(users::table.filter(users::id.eq(user_id)))
            .set(users::farmowner.eq(1))
            .execute(conn)
    }).await?;
    Ok(())
}

pub async fn delete(db: &FarmDB, user_id: i32) -> QueryResult<()> {
    db.run(move |conn| {
        diesel::delete(users::table)
            .filter(users::id.eq(user_id))
            .execute(conn)
    }).await?;
    Ok(())
}