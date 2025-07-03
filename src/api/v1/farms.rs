use rocket::get;
use rocket::serde::json::Json;
use crate::data::farm::{Farm, FullFarm};
use crate::data::FarmDB;

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![list_farms, get_farms_near, get_full_farm]
}

#[get("/")]
async fn list_farms(db: FarmDB) -> crate::api::Result<Json<Vec<Farm>>> {
    let farms = crate::data::farm::list_farms(db).await?;
    Ok(Json(farms))
}

#[get("/find_near?<lat>&<lon>&<radius>")]
async fn get_farms_near(db: FarmDB, lat: f32, lon: f32, radius: f32) -> crate::api::Result<Json<Vec<Farm>>> {
    let farms = crate::data::farm::get_farms_near(db, lat, lon, radius).await?;
    Ok(Json(farms))
}

#[get("/<farm_id>")]
async fn get_full_farm(db: FarmDB, farm_id: i32) -> crate::api::Result<Json<Option<FullFarm>>> {
    let full_farm = crate::data::farm::load_full_farm(db, farm_id).await?;
    Ok(Json(full_farm))
}
