use crate::config::Settings;
use crate::logger::log_event;
use crate::strategies::{evaluate_ssl_ema, Candle, SignalDirection};
use serde_json::json;

/// Ejecuta todas las pruebas disponibles
pub async fn run_all_tests() {
    log_event("test_runner", "-", "Ejecutando todos los tests", json!({}));
    run_ssl_ema_test().await;
}

/// Test unitario para la estrategia SSL + EMA
pub async fn run_ssl_ema_test() {
    log_event("test", "ssl_ema", "Iniciando test de estrategia SSL + EMA", json!({}));

    let settings = crate::config::load();

    let candles = generate_sample_data();

    if let Some(signal) = evaluate_ssl_ema(&candles) {
        log_event("signal", "ssl_ema", "Señal generada", json!({
            "entry": signal.entry_price,
            "stop_loss": signal.stop_loss,
            "tipo": format!("{:?}", signal.direction),
            "estrategia": signal.strategy
        }));
    } else {
        log_event("no_signal", "ssl_ema", "No se generó señal con los datos actuales", json!({}));
    }
}

/// Genera un conjunto simulado de velas
fn generate_sample_data() -> Vec<Candle> {
    let mut candles = vec![];
    let mut price = 100.0;

    for i in 0..250 {
        let open = price;
        let high = price + rand::random::<f64>() * 2.0;
        let low = price - rand::random::<f64>() * 2.0;
        let close = (high + low) / 2.0;
        price = close;

        candles.push(Candle {
            timestamp: i,
            open,
            high,
            low,
            close,
        });
    }

    candles
}
