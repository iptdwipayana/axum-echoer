const APP_HOST: &str = "0.0.0.0";
const APP_PORT: &str = "3000";

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind(format!("{APP_HOST}:{APP_PORT}"))
        .await
        .unwrap();

    axum::serve(listener, routers::init())
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await
        .unwrap();
}

mod error {
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
}

mod routers {
    use axum::{Router, routing};

    use crate::handlers;

    pub fn init() -> Router {
        Router::new()
            .route("/echo", routing::post(handlers::echo))
            .route("/health", routing::get(handlers::health))
    }
}

mod handlers {
    use axum::{Json, extract::rejection::JsonRejection, http::StatusCode};
    use serde::Serialize;
    use serde_json::Value;

    use crate::error::Error;

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

    pub async fn echo(
        body: Result<Json<Value>, JsonRejection>,
    ) -> Result<(StatusCode, Json<Response<EchoResponse>>), Error> {
        let Json(body) = body?;
        Ok((
            StatusCode::OK,
            Json(Response {
                message: "success".into(),
                data: Some(EchoResponse { body }),
            }),
        ))
    }

    pub async fn health() -> (StatusCode, Json<Response<HealthResponse>>) {
        (
            StatusCode::OK,
            Json(Response {
                message: "healthy".into(),
                data: Some(HealthResponse {
                    status: "ok".into(),
                }),
            }),
        )
    }
}
