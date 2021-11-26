use axum::{routing, Router};

mod handler;
mod types;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "tg_bot=debug");
    }
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", routing::post(handler::hook).get(handler::index));

    tracing::debug!("running");
    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
