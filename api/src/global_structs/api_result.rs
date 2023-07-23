use crate::global_structs::api_error::ApiError;

pub type ApiResult<T> = core::result::Result<T, ApiError>;
