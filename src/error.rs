#[derive(Debug)]
pub enum AppErrorType {
    HttpError,
    APIError,
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
    pub fn from_str(msg: &str, error_type: AppErrorType) -> Self {
        Self::new(Some(msg.to_string()), None, error_type)
    }
    pub fn api_error(msg: &str) -> Self {
        Self::from_str(msg, AppErrorType::APIError)
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
