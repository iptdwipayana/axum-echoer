mod error;
mod handlers;
mod routers;

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
