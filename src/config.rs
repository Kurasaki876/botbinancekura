use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub use_ai_validation: bool,
    pub min_volume: u64,
    pub max_spread: f64,
    pub timeframes: Vec<String>,
    pub strategy: StrategyConfig,
}

#[derive(Debug, Deserialize)]
pub struct StrategyConfig {
    pub ssl_ema: StrategyEnabled,
    pub trend_meter_ema_atr: StrategyEnabled,
    pub ma_insilico: StrategyEnabled,
    pub supertrend_qqe_trend_a: StrategyEnabled,
}

#[derive(Debug, Deserialize)]
pub struct StrategyEnabled {
    pub enabled: bool,
}

pub fn load() -> Settings {
    dotenv::dotenv().ok();
    let config_str = fs::read_to_string("Config.toml").expect("Failed to read Config.toml");
    toml::from_str(&config_str).expect("Failed to parse Config.toml")
}
