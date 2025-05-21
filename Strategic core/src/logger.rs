use chrono::Utc;
use log::info;
use serde_json::{json, Value};

pub fn init() {
    env_logger::init();
}

pub fn log_event(event_type: &str, symbol: &str, message: &str, data: Value) {
    let log_entry = json!({
        "timestamp": Utc::now().to_rfc3339(),
        "event": event_type,
        "symbol": symbol,
        "message": message,
        "data": data
    });
    info!("{}", log_entry.to_string());
}
