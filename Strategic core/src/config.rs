use config::{Config, Environment, File};
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    #[serde(default)]
    pub binance_api_key: String,
    #[serde(default)]
    pub binance_api_secret: String,
    #[serde(default)]
    pub trade_mode: String,
    #[serde(rename = "settings")]
    pub core: CoreSettings,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CoreSettings {
    pub use_ai_validation: bool,
    pub min_volume: f64,
    pub max_spread: f64,
    pub timeframes: Vec<String>,
    pub strategies: Strategies,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Strategies {
    pub ssl_ema: StrategyEnabled,
    pub trend_meter_ema_atr: StrategyEnabled,
    pub supertrend_qqe_trend_a: StrategyEnabled,
    pub squeeze_momentum_atr: StrategyEnabled,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StrategyEnabled {
    pub enabled: bool,
}

pub fn load() -> Settings {
    dotenv().ok();

    let config = Config::builder()
        .add_source(File::with_name("Config").required(false))
        .add_source(Environment::default().separator("_"))
        .build()
        .unwrap();

    config.try_deserialize::<Settings>().unwrap()
}