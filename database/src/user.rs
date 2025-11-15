use std::io::Write;
use crate::farm::Farm;
use crate::{DbResult, FarmDB};
use crate::schema::{farm_admins, users};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use diesel::deserialize::FromSql;
use diesel::{AsExpression, FromSqlRow};
use diesel::pg::{Pg, PgValue};
use diesel::prelude::*;
use diesel::serialize::{IsNull, Output, ToSql};
use uuid::Uuid;
use crate::schema;

pub mod settings;

#[derive(Debug, FromSqlRow, PartialEq, Eq, Clone, AsExpression)]
#[diesel(sql_type = schema::sql_types::FarmAdminStatus)]
pub enum FarmOwnerStatus {
    NO,
    YES,
    REQUESTED,
}

impl ToSql<schema::sql_types::FarmAdminStatus, Pg> for FarmOwnerStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match self {
            FarmOwnerStatus::NO => out.write_all(b"NO")?,
            FarmOwnerStatus::YES => out.write_all(b"YES")?,
            FarmOwnerStatus::REQUESTED => out.write_all(b"REQUESTED")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<schema::sql_types::FarmAdminStatus, Pg> for FarmOwnerStatus {
    fn from_sql(bytes: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"NO" => Ok(FarmOwnerStatus::NO),
            b"YES" => Ok(FarmOwnerStatus::YES),
            b"REQUESTED" => Ok(FarmOwnerStatus::REQUESTED),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Clone, Selectable, Identifiable, Queryable)]
pub struct User {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub sysadmin: i32,
    pub farmowner: FarmOwnerStatus,
    pub ext_id: Uuid,
}

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

#[derive(Identifiable, Queryable, Associations, Selectable)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Farm))]
pub struct FarmAdmin {
    pub id: i32,
    pub user_id: i32,
    pub farm_id: i32,
}

pub async fn create_user(db: &FarmDB, user: NewUser, password: String) -> DbResult<User> {
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

pub async fn default_user_change(db: &FarmDB, user: DefaultUserChange) -> DbResult<()> {
    db.run(move |conn| {
        let username = user.username.clone();
        diesel::update(users::table)
            .filter(users::username.eq(&username))
            .set(user)
            .execute(conn)
    }).await?;
    Ok(())
}

pub async fn password_change(db: &FarmDB, username: String, password: String) -> DbResult<()> {
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

pub async fn check_login(db: &FarmDB, username: String, password: String) -> DbResult<bool> {
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

pub async fn username_by_identity(db: &FarmDB, identity: String) -> DbResult<Option<String>> {
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

pub async fn by_username(db: &FarmDB, username: String) -> DbResult<Option<User>> {
    let user = db.run(move |conn| {
    users::table
        .select(User::as_select())
        .filter(users::username.eq(username))
        .first(conn)
        .optional()
    }).await?;
    Ok(user)
}

async fn set_farmowner_status(db: &FarmDB, user_id: i32, status: FarmOwnerStatus) -> DbResult<()> {
    db.run(move |conn| {
        diesel::update(users::table)
            .filter(users::id.eq(user_id))
            .set(users::farmowner.eq(status))
            .execute(conn)
    }).await?;
    Ok(())
}

pub async fn make_farmowner(db: &FarmDB, user_id: i32) -> DbResult<()> {
    set_farmowner_status(db, user_id, FarmOwnerStatus::YES).await
}

pub async fn request_farm_admin_status(db: &FarmDB, user_id: i32) -> DbResult<()> {
    set_farmowner_status(db, user_id, FarmOwnerStatus::REQUESTED).await
}

pub async fn delete(db: &FarmDB, user_id: i32) -> DbResult<()> {
    db.run(move |conn| {
        diesel::delete(users::table)
            .filter(users::id.eq(user_id))
            .execute(conn)
    }).await?;
    Ok(())
}
