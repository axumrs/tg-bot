use axum::Json;

use crate::types::Update;

pub async fn hook(msg: Option<Json<Update>>) -> String {
    let msg = format!("{:?}", msg);
    tracing::debug!("{}", msg);
    msg
}

pub async fn index() -> &'static str {
    "A telegram bot from axum.rs"
}
