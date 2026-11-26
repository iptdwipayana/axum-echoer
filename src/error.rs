use axum::{Json, extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse};

#[derive(Debug)]
pub enum Error {
    InvalidJson(StatusCode, String),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            Self::InvalidJson(status, message) => (status, message),
        };

        (
            status,
            Json(serde_json::json!({
                "message": message,
                "data": null,
            })),
        )
            .into_response()
    }
}

impl From<JsonRejection> for Error {
    fn from(value: JsonRejection) -> Self {
        Self::InvalidJson(value.status(), value.body_text())
    }
}
