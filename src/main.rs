use axum::{routing, AddExtensionLayer, Router};
use dotenv::dotenv;

mod config;
mod handler;
mod model;
mod types;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "tg_bot=debug");
    }
    tracing_subscriber::fmt::init();

    dotenv().ok();
    let cfg = config::Config::from_env().expect("初始化配置失败");

    let app = Router::new()
        .route("/", routing::post(handler::hook).get(handler::index))
        .layer(AddExtensionLayer::new(model::AppState {
            bot: cfg.tg_bot.clone(),
        }));

    tracing::debug!("Web服务运行在：{}", &cfg.web.addr);

    axum::Server::bind(&cfg.web.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
