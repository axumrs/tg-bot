use serde::Deserialize;

#[derive(Deserialize)]
pub struct WebConfig {
    pub addr: String,
}

#[derive(Deserialize, Clone)]
pub struct TgBotConfig {
    pub token: String,
    pub webhook: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub web: WebConfig,
    pub tg_bot: TgBotConfig,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}
