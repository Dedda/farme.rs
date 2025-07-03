use rocket::response::Debug;

pub mod v1;

pub type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;
