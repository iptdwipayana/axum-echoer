use axum::{Router, routing};

pub fn init() -> Router {
    use crate::handlers;

    Router::new()
        .route("/echo", routing::post(handlers::echo))
        .route("/health", routing::get(handlers::health))
}
