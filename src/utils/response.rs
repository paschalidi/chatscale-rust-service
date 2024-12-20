use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ApiResponse<T: Serialize> {
    pub data: Option<T>,
    pub message: Option<String>,
}

pub struct ServerResponse;

impl ServerResponse {
    pub fn ok<T: Serialize>(data: T) -> Response {
        (
            StatusCode::OK,
            Json(ApiResponse {
                data: Some(data),
                message: None,
            }),
        )
            .into_response()
    }

    pub fn created<T: Serialize>(data: T) -> Response {
        (
            StatusCode::CREATED,
            Json(ApiResponse {
                data: Some(data),
                message: None,
            }),
        )
            .into_response()
    }

    // Error responses
    pub fn bad_request(message: &str) -> Response {
        (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<()> {
                data: None,
                message: Some(message.to_string()),
            }),
        )
            .into_response()
    }

    pub fn not_found(message: &str) -> Response {
        (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<()> {
                data: None,
                message: Some(message.to_string()),
            }),
        )
            .into_response()
    }

    pub fn server_error<E: std::fmt::Debug>(err: E, message: &str) -> Response {
        tracing::error!("server error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<()> {
                data: None,
                message: Some(message.to_string()),
            }),
        )
            .into_response()
    }
}
