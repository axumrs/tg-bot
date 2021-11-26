use axum::{extract::Extension, Json};

use crate::{
    model::AppState,
    types::{request, Update},
};

pub async fn hook(Json(payload): Json<Update>, Extension(state): Extension<AppState>) -> String {
    let msg = format!("{:?}", payload);
    tracing::debug!("received: {}", msg);

    // 回复信息
    let send_data = request::TextMessage {
        chat_id: payload.message.chat.id,
        text: format!("ECHO: {}", payload.message.text),
    };
    let api_addr = format!(
        "https://api.telegram.org/bot{}/{}",
        state.bot.token.clone(),
        "sendMessage"
    );
    let res = reqwest::Client::new()
        .post(&api_addr)
        .form(&send_data)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    tracing::debug!("sendMessage: {}", &res);
    format!("{:?}", res)
}

pub async fn index() -> &'static str {
    "A telegram bot from axum.rs"
}
