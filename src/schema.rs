// @generated automatically by Diesel CLI.

diesel::table! {
    contact (id) {
        id -> Integer,
        farm_id -> Integer,
        email -> Nullable<Text>,
        phone -> Nullable<Text>,
        address -> Nullable<Text>,
    }
}

diesel::table! {
    farm_admins (id) {
        id -> Integer,
        user_id -> Integer,
        farm_id -> Integer,
    }
}

diesel::table! {
    farm_locations (id) {
        id -> Integer,
        farm_id -> Integer,
        location_id -> Integer,
    }
}

diesel::table! {
    farm_shop_types (id) {
        id -> Integer,
        farm_id -> Integer,
        shop_type_id -> Integer,
    }
}

diesel::table! {
    farms (id) {
        id -> Integer,
        name -> Text,
        created -> Timestamp,
    }
}

diesel::table! {
    geolocations (id) {
        id -> Integer,
        lat -> Float,
        lon -> Float,
    }
}

diesel::table! {
    opening_hours (id) {
        id -> Integer,
        farm_id -> Integer,
        weekday -> Integer,
        open -> Time,
        close -> Time,
    }
}

diesel::table! {
    shop_types (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        firstname -> Text,
        lastname -> Text,
        username -> Text,
        email -> Text,
        password -> Text,
        sysadmin -> Integer,
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

diesel::allow_tables_to_appear_in_same_query!(
    contact,
    farm_admins,
    farm_locations,
    farm_shop_types,
    farms,
    geolocations,
    opening_hours,
    shop_types,
    users,
);
