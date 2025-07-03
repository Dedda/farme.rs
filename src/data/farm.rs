use crate::data::FarmDB;
use crate::schema::{farm_locations, farm_shop_types, farms, geolocations, opening_hours, shop_types};
use diesel::prelude::*;
use rocket::serde::Serialize;

#[derive(Serialize, Identifiable, Queryable, Selectable)]
#[serde(crate = "rocket::serde")]
pub struct Farm {
    id: i32,
    name: String,
}

#[derive(Insertable)]
#[diesel(table_name = farms)]
pub struct NewFarm {
    name: String,
}

#[derive(Identifiable, Queryable, Selectable)]
pub struct Geolocation {
    id: i32,
    lat: f32,
    lon: f32,
}

#[derive(Identifiable, Queryable, Associations)]
#[diesel(belongs_to(Geolocation, foreign_key = location_id))]
#[diesel(belongs_to(Farm))]
#[diesel(table_name = farm_locations)]
pub struct FarmLocation {
    id: i32,
    farm_id: i32,
    location_id: i32,
}

#[derive(Serialize, Queryable, Selectable)]
#[serde(crate = "rocket::serde")]
pub struct ShopType {
    id: i32,
    name: String,
}

#[derive(Identifiable, Queryable, Associations)]
#[diesel(belongs_to(ShopType))]
#[diesel(belongs_to(Farm))]
#[diesel(table_name = farm_shop_types)]
pub struct FarmShopTypes {
    id: i32,
    shop_type_id: i32,
    farm_id: i32,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Contact {
    id: i32,
    email: Option<String>,
    phone: Option<String>,
    address: Option<String>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct FullFarm {
    id: i32,
    name: String,
    lat: f32,
    lon: f32,
    shop_types: Vec<ShopType>,
    opening_hours: Vec<OpeningHours>
}

#[derive(Identifiable, Serialize, Queryable, Selectable, Associations)]
#[diesel(check_for_backend())]
#[diesel(belongs_to(Farm))]
#[diesel(table_name = opening_hours)]
#[serde(crate = "rocket::serde")]
pub struct OpeningHours {
    id: i32,
    farm_id: i32,
    weekday: i32,
    open: chrono::NaiveTime,
    close: chrono::NaiveTime,
}

pub async fn list_farms(db: FarmDB) -> QueryResult<Vec<Farm>> {
    db.run(move |conn| {
        farms::table.select(Farm::as_select()).load(conn)
    }).await
}

pub async fn get_farms_near(db: FarmDB, lat: f32, lon: f32, radius: f32) -> QueryResult<Vec<Farm>> {
    db.run(move |conn| {
        let f = farms::table
            .inner_join(farm_locations::table)
            .inner_join(geolocations::table.on(farm_locations::location_id.eq(geolocations::id)))
            .filter(geolocations::lat.ge(lat - radius))
            .filter(geolocations::lat.le(lat + radius))
            .filter(geolocations::lon.ge(lon - radius))
            .filter(geolocations::lon.le(lon + radius))
            .select(Farm::as_select())
            .load::<Farm>(conn)?;
        Ok(f)
    }).await
}


pub async fn load_full_farm(db: FarmDB, farm_id: i32) -> QueryResult<Option<FullFarm>> {
    db.run(move |conn| {
        farms::table
            .select(Farm::as_select())
            .find(farm_id)
            .first(conn)
            .optional()?.map(|farm| {
            let location = FarmLocation::belonging_to(&farm)
                .inner_join(geolocations::table)
                .select(Geolocation::as_select())
                .get_result(conn)?;
            let shop_types = FarmShopTypes::belonging_to(&farm)
                .inner_join(shop_types::table)
                .select(ShopType::as_select())
                .load(conn)?;
            let hours: Vec<OpeningHours> = OpeningHours::belonging_to(&farm)
                .select(OpeningHours::as_select())
                .load(conn)?;

            Ok(Some(FullFarm {
                id: farm_id,
                name: farm.name,
                lat: location.lat,
                lon: location.lon,
                shop_types,
                opening_hours: hours,
            }))
        }).unwrap_or(Ok(None))
    }).await
}