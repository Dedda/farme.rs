use diesel::prelude::*;
use crate::{DbResult, FarmDB};
use crate::schema::{geolocations, farm_locations};

#[derive(Selectable)]
#[diesel(table_name = geolocations)]
pub struct GeoLocation {
    pub id: i32,
    pub lat: f32,
    pub lon: f32,
}

#[derive(Insertable)]
#[diesel(table_name = geolocations)]
pub struct NewGeoLocation {
    pub lat: f32,
    pub lon: f32,
}

#[derive(Selectable)]
pub struct FarmLocation {
    pub id: i32,
    pub farm_id: i32,
    pub location_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = farm_locations)]
struct NewFarmLocation {
    farm_id: i32,
    location_id: i32,
}

pub async fn add_new_location_to_farm(db: &FarmDB, location: NewGeoLocation, farm_id: i32) -> DbResult<()> {
    db.run(move |conn| {
        let new_id: i32 = diesel::insert_into(geolocations::table)
            .values(location)
            .returning(geolocations::id)
            .get_result(conn)?;
        diesel::insert_into(farm_locations::table)
            .values(&NewFarmLocation { farm_id, location_id: new_id })
            .execute(conn)?;
        Ok(())
    }).await
}