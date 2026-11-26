use axum::{Json, extract::rejection::JsonRejection, http::StatusCode};
use serde::Serialize;
use serde_json::Value;

type ApiResponse<T> = Result<(StatusCode, Json<Response<T>>), crate::error::Error>;

#[derive(Debug, Serialize)]
pub struct Response<T> {
    message: String,
    data: Option<T>,
}

#[derive(Debug, Serialize)]
pub struct EchoResponse {
    body: Value,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    status: String,
}

pub async fn echo(body: Result<Json<Value>, JsonRejection>) -> ApiResponse<EchoResponse> {
    let Json(body) = body?;
    Ok((
        StatusCode::OK,
        Json(Response {
            message: "success".into(),
            data: Some(EchoResponse { body }),
        }),
    ))
}

pub async fn health() -> ApiResponse<HealthResponse> {
    Ok((
        StatusCode::OK,
        Json(Response {
            message: "healthy".into(),
            data: Some(HealthResponse {
                status: "ok".into(),
            }),
        }),
    ))
}
