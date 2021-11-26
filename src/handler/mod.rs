use axum::{extract::Extension, Json};

use crate::{bot, error::AppError, model::AppState, types::Update, Result};

mod command;

pub async fn hook(
    Json(payload): Json<Update>,
    Extension(state): Extension<AppState>,
) -> Result<String> {
    let msg = format!("{:?}", payload);
    tracing::debug!("received: {}", msg);

    let msg_text = payload.message.text.unwrap_or("".to_string());

    let reply_msg = match msg_text.as_str() {
        "/website" => command::website(),
        _ => echo(msg_text),
    };

    let res = bot::send_text_message(&state.bot.token, payload.message.chat.id, reply_msg)
        .await
        .map_err(log_error(String::from("bot_send_text_message")))?;

    let result = format!("{:?}", res);
    tracing::debug!("sendMessage: {}", &result);
    Ok(result)
}

pub async fn index() -> &'static str {
    "A telegram bot from axum.rs"
}

fn echo(msg: String) -> String {
    format!("ECHO: {}", msg)
}

fn log_error(handler_name: String) -> Box<dyn Fn(AppError) -> AppError> {
    Box::new(move |err| {
        tracing::error!("{}: {:?}", handler_name, err);
        err
    })
}
