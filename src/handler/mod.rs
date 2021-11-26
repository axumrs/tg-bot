use axum::Json;

use crate::types::Update;

pub async fn hook(msg: String) -> String {
    let msg = format!("recieved: {:?}", msg);
    tracing::debug!("{}", msg);
    msg
}

pub async fn index() -> &'static str {
    "A telegram bot from axum.rs"
}
