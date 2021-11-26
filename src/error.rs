use std::convert::Infallible;

use axum::{
    body::{Bytes, Full},
    http::StatusCode,
    response::IntoResponse,
};

#[derive(Debug)]
pub enum AppErrorType {
    HttpError,
    SerdeError,
}
#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub error_type: AppErrorType,
}

impl AppError {
    pub fn new(message: Option<String>, cause: Option<String>, error_type: AppErrorType) -> Self {
        Self {
            message,
            cause,
            error_type,
        }
    }
    pub fn from_err(cause: impl ToString, error_type: AppErrorType) -> Self {
        Self::new(None, Some(cause.to_string()), error_type)
    }
    #[allow(unused)]
    pub fn from_str(msg: &str, error_type: AppErrorType) -> Self {
        Self::new(Some(msg.to_string()), None, error_type)
    }
}
impl std::error::Error for AppError {}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        Self::from_err(err, AppErrorType::HttpError)
    }
}
impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        Self::from_err(err, AppErrorType::SerdeError)
    }
}

impl IntoResponse for AppError {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> axum::http::Response<Self::Body> {
        let msg = self.message.unwrap_or("有错误发生".to_string());
        (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
    }
}
