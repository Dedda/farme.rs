use crate::api::v1::error::ApiError;

pub mod v1;

pub type Result<T> = std::result::Result<T, ApiError>;
