use serde::Serialize;

use crate::{
    error::AppError,
    types::{request, Response},
    Result,
};

async fn invoke_api<T: Serialize>(data: &T, method: &str, token: &str) -> Result<Response> {
    let api_addr = format!("https://api.telegram.org/bot{}/{}", token, method);
    let res = reqwest::Client::new()
        .post(&api_addr)
        .form(data)
        .send()
        .await
        .map_err(AppError::from)?
        .text()
        .await
        .map_err(AppError::from)?;
    let res = serde_json::from_str(&res).map_err(AppError::from)?;
    Ok(res)
}

pub async fn send_text_message(token: &str, chat_id: u64, text: String) -> Result<Response> {
    let data = request::TextMessage { chat_id, text };
    invoke_api(&data, "sendMessage", token).await
}
