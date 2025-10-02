use crate::api::Result as ApiResult;
use crate::api::v1::error::ApiError;
use crate::data::FarmDB;
use crate::data::farm::{Farm, FullFarm, NewFarm};
use crate::data::location::NewGeoLocation;
use crate::data::user::User;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde::{Deserialize, Serialize};

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![list_farms, get_farms_near, get_full_farm, create_farm]
}

#[derive(Serialize)]
struct ApiFarm {
    name: String,
}

impl From<Farm> for ApiFarm {
    fn from(value: Farm) -> Self {
        Self { name: value.name }
    }
}

#[derive(Deserialize)]
struct NewApiFarm {
    name: String,
    lat: f32,
    lon: f32,
}

impl From<NewApiFarm> for NewFarm {
    fn from(value: NewApiFarm) -> Self {
        Self { name: value.name }
    }
}

#[get("/")]
async fn list_farms(db: FarmDB) -> ApiResult<Json<Vec<Farm>>> {
    let farms = crate::data::farm::list_farms(db).await?;
    Ok(Json(farms))
}

#[get("/find_near?<lat>&<lon>&<radius>")]
async fn get_farms_near(db: FarmDB, lat: f32, lon: f32, radius: f32) -> ApiResult<Json<Vec<Farm>>> {
    let farms = crate::data::farm::get_farms_near(db, lat, lon, radius).await?;
    Ok(Json(farms))
}

#[get("/<farm_id>")]
async fn get_full_farm(db: FarmDB, farm_id: i32) -> ApiResult<Json<Option<FullFarm>>> {
    let full_farm = crate::data::farm::load_full_farm(db, farm_id).await?;
    Ok(Json(full_farm))
}

#[post("/create", data = "<farm>")]
async fn create_farm(db: FarmDB, user: User, farm: Json<NewApiFarm>) -> ApiResult<Json<ApiFarm>> {
    if user.farmowner == 0 {
        return Err(ApiError::MissingPrivilege(String::from(
            "user must have `farmowner` privilege",
        )));
    }
    let new_location = NewGeoLocation {
        lat: farm.lat,
        lon: farm.lon,
    };
    let new_farm: NewFarm = farm.0.into();
    let new_farm = crate::data::farm::create_farm(&db, &user, new_farm).await?;
    crate::data::location::add_new_location_to_farm(&db, new_location, new_farm.id).await?;
    Ok(Json(new_farm.into()))
}

#[cfg(test)]
mod tests {
    use diesel::{ExpressionMethods, RunQueryDsl};
    use crate::data::FarmDB;
    use crate::data::user::{NewUser, create_user};
    use crate::api::v1::test_utils::create_untracked_client;

    #[tokio::test]
    async fn farm_api_crud() {
        let client = create_untracked_client().await;

        let user = NewUser {
            firstname: "farm_api_crud".to_string(),
            lastname: "farm_api_crud".to_string(),
            username: "farm_api_crud".to_string(),
            email: "farm_api_crud@test.com".to_string(),
        };
        let db = FarmDB::get_one(client.rocket())
            .await
            .expect("failed to get db");
        let user = create_user(&db, user, "Abc123!.".to_string())
            .await
            .expect("failed to create user");

        let user_id = user.id;
        db.run(move |conn| {
            diesel::delete(crate::schema::users::table)
                .filter(crate::schema::users::id.eq(user_id))
                .execute(conn)
                .expect("Cannot delete user");
        }).await;
    }
}
