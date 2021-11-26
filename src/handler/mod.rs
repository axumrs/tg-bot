use axum::{extract::Extension, Json};

use crate::{
    bot,
    error::AppError,
    model::AppState,
    types::{MsgType, Update},
    Result,
};

mod command;

pub async fn hook(
    Json(payload): Json<Update>,
    Extension(state): Extension<AppState>,
) -> Result<String> {
    let msg = format!("{:?}", payload);
    tracing::debug!("received: {}", msg);

    let msg_text = payload.message.text.unwrap_or("".to_string());

    let msg_type = match msg_text.as_str() {
        "/website" => MsgType::Text(command::website()),
        "/logo" => MsgType::Photo(command::logo()),
        _ => MsgType::Text(echo(msg_text.clone())),
    };

    let res = match msg_type {
        MsgType::Text(reply_msg) => {
            bot::send_text_message(&state.bot.token, payload.message.chat.id, reply_msg).await
        }
        MsgType::Photo(reply_msg) => {
            bot::send_photo_message(&state.bot.token, payload.message.chat.id, reply_msg).await
        }
    }
    .map_err(log_error(msg_text));

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
