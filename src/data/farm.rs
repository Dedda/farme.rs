use crate::data::FarmDB;
use crate::schema::{farm_admins, farm_locations, farm_shop_types, farms, geolocations, opening_hours, shop_types};
use diesel::prelude::*;
use rocket::serde::Serialize;
use uuid::Uuid;
use crate::data::user::{FarmAdmin, User};

#[derive(Serialize, Identifiable, Queryable, Selectable)]
#[serde(crate = "rocket::serde")]
pub struct Farm {
    pub id: i32,
    pub name: String,
    pub ext_id: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = farms)]
pub struct NewFarm {
    pub name: String,
}

#[derive(Identifiable, Queryable, Selectable)]
pub struct Geolocation {
    pub id: i32,
    pub lat: f32,
    pub lon: f32,
}

#[derive(Identifiable, Queryable, Associations)]
#[diesel(belongs_to(Geolocation, foreign_key = location_id))]
#[diesel(belongs_to(Farm))]
#[diesel(table_name = farm_locations)]
pub struct FarmLocation {
    pub id: i32,
    pub farm_id: i32,
    pub location_id: i32,
}

#[derive(Serialize, Queryable, Selectable)]
#[serde(crate = "rocket::serde")]
pub struct ShopType {
    pub id: i32,
    pub name: String,
}

#[derive(Identifiable, Queryable, Associations)]
#[diesel(belongs_to(ShopType))]
#[diesel(belongs_to(Farm))]
#[diesel(table_name = farm_shop_types)]
pub struct FarmShopTypes {
    pub id: i32,
    pub shop_type_id: i32,
    pub farm_id: i32,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Contact {
    pub id: i32,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct FullFarm {
    pub id: i32,
    pub name: String,
    pub lat: f32,
    pub lon: f32,
    pub shop_types: Vec<ShopType>,
    pub opening_hours: Vec<OpeningHours>
}

#[derive(Identifiable, Serialize, Queryable, Selectable, Associations)]
#[diesel(check_for_backend())]
#[diesel(belongs_to(Farm))]
#[diesel(table_name = opening_hours)]
#[serde(crate = "rocket::serde")]
pub struct OpeningHours {
    pub id: i32,
    pub farm_id: i32,
    pub weekday: i32,
    pub open: chrono::NaiveTime,
    pub close: chrono::NaiveTime,
}

#[derive(Insertable)]
#[diesel(table_name = farm_admins)]
pub struct NewFarmAdmin {
    pub user_id: i32,
    pub farm_id: i32,
}

pub async fn list_farms(db: &FarmDB) -> QueryResult<Vec<Farm>> {
    db.run(move |conn| {
        farms::table.select(Farm::as_select()).load(conn)
    }).await
}

pub async fn get_farms_near(db: &FarmDB, lat: f32, lon: f32, radius: f32) -> QueryResult<Vec<Farm>> {
    db.run(move |conn| {
        let f = farms::table
            .inner_join(farm_locations::table)
            .inner_join(geolocations::table.on(farm_locations::location_id.eq(geolocations::id)))
            .filter(geolocations::lat.between(lat-radius, lat+radius))
            .filter(geolocations::lon.between(lon-radius, lon+radius))
            .select(Farm::as_select())
            .load::<Farm>(conn)?;
        Ok(f)
    }).await
}

pub async fn get_farms_owned_by(db: &FarmDB, user: &User) -> QueryResult<Vec<Farm>> {
    let user_id = user.id;
    db.run(move |conn| {
        let f = farms::table
            .inner_join(farm_admins::table.on(farm_admins::user_id.eq(user_id)))
            .select(Farm::as_select())
            .load::<Farm>(conn)?;
        Ok(f)
    }).await
}

pub async fn load_full_farm(db: &FarmDB, farm_id: i32) -> QueryResult<Option<FullFarm>> {
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

pub async fn create_farm(db: &FarmDB, owner: &User, new_farm: NewFarm) -> QueryResult<Farm> {
    let owner_id = owner.id;
    db.run(move |conn| {
        let new_id: i32 = diesel::insert_into(farms::table)
            .values(new_farm)
            .returning(farms::id)
            .get_result(conn)?;
        let farm = farms::table
            .select(Farm::as_select())
            .find(new_id)
            .first(conn)
            .optional()?
            .expect("");
        let owner = NewFarmAdmin {
            user_id: owner_id,
            farm_id: farm.id,
        };
        diesel::insert_into(farm_admins::table)
            .values(owner)
            .execute(conn)?;
        Ok(farm)
    }).await
}

pub async fn delete_farm(db: &FarmDB, user_id: i32, farm_id: i32) -> QueryResult<bool> {
    db.run(move |conn| {
        let owner = farm_admins::table.select(FarmAdmin::as_select())
            .filter(farm_admins::user_id.eq(user_id))
            .filter(farm_admins::farm_id.eq(farm_id))
            .first::<FarmAdmin>(conn)
            .optional()?;
        if let Some(owner) = owner {
            diesel::delete(farm_admins::table)
                .filter(farm_admins::id.eq(owner.id))
                .execute(conn)?;
            diesel::delete(farms::table)
                .filter(farms::id.eq(farm_id))
                .execute(conn)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }).await
}