use axum::http::StatusCode;
use axum::Json;
use axum_extra::either::Either;
use serde::Serialize;

pub mod valid_json;

pub type ApiResponse<T> = (StatusCode, Either<Json<T>, Json<ApiError>>);

#[derive(Serialize)]
pub struct ApiError {
    pub status: u16,
    pub message: String
}

pub fn ok<T: Serialize>(data: T) -> ApiResponse<T> {
    (StatusCode::OK, Either::E1(Json(data)))
}

pub fn api_error<T: Serialize>(status: StatusCode, message: &str) -> ApiResponse<T> {
    (status, Either::E2(Json(ApiError {
        status: status.as_u16(),
        message: String::from(message)
    })))
}

impl From<StatusCode> for ApiError {
    fn from(status: StatusCode) -> Self {
        ApiError {
            status: status.as_u16(),
            message: status.canonical_reason().unwrap_or("Unknown error").to_string()
        }
    }
}

unsafe impl Send for ApiError {}
unsafe impl Sync for ApiError {}