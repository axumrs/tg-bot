use crate::config;

#[derive(Clone)]
pub struct AppState {
    pub bot: config::TgBotConfig,
}
