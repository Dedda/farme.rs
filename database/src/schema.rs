// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "farm_admin_status"))]
    pub struct FarmAdminStatus;
}

diesel::table! {
    contact (id) {
        id -> Int4,
        farm_id -> Int4,
        email -> Nullable<Text>,
        phone -> Nullable<Text>,
        address -> Nullable<Text>,
    }
}

diesel::table! {
    farm_admins (id) {
        id -> Int4,
        user_id -> Int4,
        farm_id -> Int4,
    }
}

diesel::table! {
    farm_locations (id) {
        id -> Int4,
        farm_id -> Int4,
        location_id -> Int4,
    }
}

diesel::table! {
    farm_shop_types (id) {
        id -> Int4,
        farm_id -> Int4,
        shop_type_id -> Int4,
    }
}

diesel::table! {
    farms (id) {
        id -> Int4,
        name -> Text,
        created -> Timestamp,
        ext_id -> Uuid,
    }
}

diesel::table! {
    geolocations (id) {
        id -> Int4,
        lat -> Float4,
        lon -> Float4,
    }
}

diesel::table! {
    opening_hours (id) {
        id -> Int4,
        farm_id -> Int4,
        weekday -> Int4,
        open -> Time,
        close -> Time,
    }
}

diesel::table! {
    shop_types (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    user_settings (id) {
        id -> Int4,
        user_id -> Int4,
        setting_name -> Text,
        setting_value -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::FarmAdminStatus;

    users (id) {
        id -> Int4,
        firstname -> Text,
        lastname -> Text,
        username -> Text,
        email -> Text,
        password -> Text,
        sysadmin -> Int4,
        farmowner -> FarmAdminStatus,
        ext_id -> Uuid,
    }
}

diesel::joinable!(contact -> farms (farm_id));
diesel::joinable!(farm_admins -> farms (farm_id));
diesel::joinable!(farm_admins -> users (user_id));
diesel::joinable!(farm_locations -> farms (farm_id));
diesel::joinable!(farm_locations -> geolocations (location_id));
diesel::joinable!(farm_shop_types -> farms (farm_id));
diesel::joinable!(farm_shop_types -> shop_types (shop_type_id));
diesel::joinable!(opening_hours -> farms (farm_id));
diesel::joinable!(user_settings -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    contact,
    farm_admins,
    farm_locations,
    farm_shop_types,
    farms,
    geolocations,
    opening_hours,
    shop_types,
    user_settings,
    users,
);
