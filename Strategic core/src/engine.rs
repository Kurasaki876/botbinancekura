use crate::binance::{filter_operable_symbols, get_usdt_balance};
use crate::config::Settings;
use crate::logger::log_event;
use rand::seq::SliceRandom;
use serde_json::json;

pub async fn run_engine(settings: &Settings) {
    log_event("engine", "-", "Iniciando ciclo operativo", json!({}));

    // 1. Obtener saldo disponible
    let balance = get_usdt_balance(settings).await;
    log_event("balance", "-", "Saldo USDT disponible", json!({ "usdt": balance }));

    // 2. Escanear símbolos operables
    let symbols = filter_operable_symbols(settings).await;
    log_event("scanner", "-", "Símbolos operables encontrados", json!({ "total": symbols.len() }));

    for symbol in symbols {
        log_event("cycle", &symbol, "Analizando símbolo", json!({}));

        // 3. Selección de estrategia según capital disponible
        let strategy = match select_strategy(settings, balance) {
            Some(s) => s,
            None => {
                log_event(
                    "skip",
                    &symbol,
                    "Capital insuficiente para todas las estrategias",
                    json!({ "capital": balance }),
                );
                continue;
            }
        };

        // 4. Validación IA condicional
        let ia_approved = if settings.core.use_ai_validation {
            simulate_ai_validation(&symbol, strategy).await
        } else {
            log_event("ia_bypass", &symbol, "IA desactivada desde configuración", json!({
                "estrategia": strategy
            }));
            true
        };

        // 5. Ejecutar orden si IA aprobó
        if ia_approved {
            log_event("execute", &symbol, "Orden ejecutada (simulada)", json!({
                "estrategia": strategy,
                "capital_usdt": balance / 10.0
            }));
        } else {
            log_event("reject", &symbol, "IA rechazó la operación", json!({
                "estrategia": strategy
            }));
        }
    }
}

fn select_strategy(settings: &Settings, capital: f64) -> Option<&'static str> {
    let mut strategies = vec![];

    if settings.core.strategies.ssl_ema.enabled && capital >= 5.0 {
        strategies.push("ssl_ema");
    }
    if settings.core.strategies.trend_meter_ema_atr.enabled && capital >= 10.0 {
        strategies.push("trend_meter_ema_atr");
    }
    if settings.core.strategies.supertrend_qqe_trend_a.enabled && capital >= 30.0 {
        strategies.push("supertrend_qqe_trend_a");
    }
    if settings.core.strategies.squeeze_momentum_atr.enabled && capital >= 50.0 {
        strategies.push("squeeze_momentum_atr");
    }

    strategies.choose(&mut rand::thread_rng()).copied()
}

async fn simulate_ai_validation(symbol: &str, strategy: &str) -> bool {
    // Simulación: retorna true con probabilidad del 75%
    let r: f64 = rand::random();
    log_event("ia_check", symbol, "Simulación de validación IA", json!({
        "estrategia": strategy,
        "probabilidad": r
    }));
    r > 0.25
}

// Exporta la función para usarla desde main.rs
pub async fn run(settings: Settings) {
    run_engine(&settings).await;
}
