use crate::api::v1::error::{ValidationError as ValidationApiError};
use crate::api::Result as ApiResult;
use crate::data::farm::{get_farms_owned_by, Farm, FullFarm, NewFarm};
use crate::data::location::NewGeoLocation;
use crate::data::{farm, FarmDB};
use crate::validation::{StringLengthCriteria, StringValidator, Validator};
use rocket::serde::json::Json;
use rocket::{delete, get, post};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::api::v1::ident::FarmOwner;

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![list_farms, get_farms_near, get_full_farm, create_farm, get_owned, delete_farm]
}

#[derive(Serialize, Deserialize)]
struct ApiFarm {
    id: i32,
    name: String,
}

impl From<Farm> for ApiFarm {
    fn from(value: Farm) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}

#[derive(Serialize, Deserialize)]
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

impl NewApiFarm {
    fn validate(&self) -> Result<(), ValidationApiError> {
        let mut errors = HashMap::new();
        if let Some(err) = self.validate_name() {
            errors.insert("name".to_string(), err);
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(ValidationApiError::for_fields(errors))
        }
    }

    fn validate_name(&self) -> Option<Vec<String>> {
        let mut validator = StringValidator::new();
        validator.add_criteria(StringLengthCriteria::min(3));
        if let Err(err) = validator.validate(&self.name) {
            return Some(err.messages);
        }
        if self.name.chars().any(|c| !c.is_alphanumeric() && !  " -._".contains(c)) {
            return Some(vec!["Only letters, numbers and characters `-._` allowed".to_string()]);
        }
        if !self.name.chars().next().unwrap().is_alphabetic() {
            return Some(vec!["Has to begin with a letter".to_string()]);
        }
        None
    }
}

#[get("/")]
async fn list_farms(db: FarmDB) -> ApiResult<Json<Vec<ApiFarm>>> {
    let farms = crate::data::farm::list_farms(&db).await?;
    Ok(Json(farms.into_iter().map(ApiFarm::from).collect()))
}

#[get("/find_near?<lat>&<lon>&<radius>")]
async fn get_farms_near(db: FarmDB, lat: f32, lon: f32, radius: f32) -> ApiResult<Json<Vec<ApiFarm>>> {
    let farms = crate::data::farm::get_farms_near(&db, lat, lon, radius).await?;
    Ok(Json(farms.into_iter().map(ApiFarm::from).collect()))
}

#[get("/<farm_id>")]
async fn get_full_farm(db: FarmDB, farm_id: i32) -> ApiResult<Json<Option<FullFarm>>> {
    let full_farm = crate::data::farm::load_full_farm(&db, farm_id).await?;
    Ok(Json(full_farm))
}

#[post("/", data = "<farm>")]
async fn create_farm(db: FarmDB, farm_owner: FarmOwner, farm: Json<NewApiFarm>) -> ApiResult<Json<ApiFarm>> {
    let user = farm_owner.0;
    let new_location = NewGeoLocation {
        lat: farm.lat,
        lon: farm.lon,
    };
    farm.0.validate()?;
    let new_farm: NewFarm = farm.0.into();
    let new_farm = crate::data::farm::create_farm(&db, &user, new_farm).await?;
    crate::data::location::add_new_location_to_farm(&db, new_location, new_farm.id).await?;
    Ok(Json(new_farm.into()))
}

#[get("/owned")]
async fn get_owned(db: FarmDB, farm_owner: FarmOwner) -> ApiResult<Json<Vec<ApiFarm>>> {
    let farms = get_farms_owned_by(&db, &farm_owner.0).await?;
    Ok(Json(farms.into_iter().map(ApiFarm::from).collect()))
}

#[delete("/<farm_id>")]
async fn delete_farm(db: FarmDB, farm_id: i32, farm_owner: FarmOwner) -> ApiResult<()> {
    farm::delete_farm(&db, farm_owner.0.id, farm_id).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::api::v1::farms::{ApiFarm, NewApiFarm};
    use crate::api::v1::test_utils::{create_test_user, create_untracked_client, get_newest_farm, login_user};
    use crate::data::user::make_farmowner;
    use crate::data::{user, FarmDB};
    use rocket::http::{Header, Status};

    #[tokio::test]
    async fn farm_api_crud() {
        let client = create_untracked_client().await;
        let password = "Abc123!.";

        let user = create_test_user(&client, "farm_api_crud", &password).await;
        let db = FarmDB::get_one(client.rocket())
            .await
            .expect("failed to get db");
        let user_id = user.id;
        make_farmowner(&db, user_id).await.expect("failed to make user a farm owner");
        let token = login_user(&client, &user.username, &password).await;

        let new_farm = NewApiFarm {
            name: "F farm_api_crud".to_string(),
            lat: 1.5,
            lon: 3.0,
        };

        // create
        let req = client.post("/api/v1/farms");
        let response = req.body(
            serde_json::to_string(&new_farm).expect("failed to serialize new farm"),
        )
            .header(Header::new("Authorization", token.clone()))
            .dispatch().await;
        assert_eq!(response.status(), Status::Ok);
        let farm = response.into_string().await.expect("expected response");
        let farm: ApiFarm = serde_json::from_str(farm.as_str()).expect("expected valid farm");
        assert_eq!("F farm_api_crud", farm.name);
        let farm = get_newest_farm(&client).await;

        // read
        let req = client.get(format!("/api/v1/farms/{}", farm.id));
        let response = req
            .header(Header::new("Authorization", token.clone())).dispatch().await;
        let api_farm = response.into_json::<ApiFarm>().await.expect("failed to deserialize api farm");
        assert_eq!("F farm_api_crud", api_farm.name);

        // read owned list
        let req = client.get("/api/v1/farms/owned");
        let response = req
            .header(Header::new("Authorization", token.clone())).dispatch().await;
        let api_farms = response.into_json::<Vec<ApiFarm>>().await.expect("failed to deserialize owned farms list");
        assert_eq!(1, api_farms.len());

        // update

        // delete
        let req = client.delete(format!("/api/v1/farms/{}", farm.id));
        let response = req
            .header(Header::new("Authorization", token)).dispatch().await;
        assert_eq!(response.status(), Status::Ok);
        user::delete(&db, user_id).await.expect("failed to delete user");
    }
}
