use axum::Json;

use crate::types::Update;

pub async fn hook(Json(update): Json<Update>) -> String {
    let msg = format!("recieved: {:?}", update);
    tracing::debug!("{}", msg);
    msg
}

pub async fn index() -> &'static str {
    "A telegram bot from axum.rs"
}
