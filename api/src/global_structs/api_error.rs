use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use strum_macros::AsRefStr;

#[derive(Clone, Debug, Serialize, AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum ApiError {
    // LoginFail,

    // Auth Errors
    // AuthFailNoAuthTokenCookie,
    // AuthFailTokenWrongFormat,
    // AuthFailCtxNotInRequestExt,

    // Model Errors
    ResourceActionFailIdNotFound { id: String },
    ResourceActionFailInvalidId { id: String },
    ResourceActionFailNoDbConnection,
    // ResourceActionFail,

    // Others
    InternalError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        response.extensions_mut().insert(self);

        response
    }
}

impl ApiError {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            // Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

            // Self::AuthFailNoAuthTokenCookie
            // | Self::AuthFailTokenWrongFormat
            // | Self::AuthFailCtxNotInRequestExt => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),
            Self::ResourceActionFailIdNotFound { .. }
            | Self::ResourceActionFailInvalidId { .. } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }
            // Fallback
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(Debug, AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    // LOGIN_FAIL,
    // NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}
