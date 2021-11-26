use serde::Serialize;

use crate::types::request;

async fn invoke_api<T: Serialize>(data: &T, method: &str, token: &str) -> String {
    let api_addr = format!("https://api.telegram.org/bot{}/{}", token, method);
    let res = reqwest::Client::new()
        .post(&api_addr)
        .form(data)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    res
}

pub async fn send_text_message(token: &str, chat_id: u64, text: String) -> String {
    let data = request::TextMessage { chat_id, text };
    invoke_api(&data, "sendMessage", token).await
}
