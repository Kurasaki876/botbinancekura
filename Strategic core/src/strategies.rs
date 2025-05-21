use crate::logger::log_event;
use serde_json::json;

#[derive(Debug)]
pub struct Candle {
    pub timestamp: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

#[derive(Debug)]
pub enum SignalDirection {
    Long,
    Short,
}

#[derive(Debug)]
pub struct TradeSignal {
    pub direction: SignalDirection,
    pub entry_price: f64,
    pub stop_loss: f64,
    pub strategy: &'static str,
}

pub fn evaluate_ssl_ema(candles: &[Candle]) -> Option<TradeSignal> {
    if candles.len() < 201 {
        return None; // Necesitamos al menos 200 velas para la EMA
    }

    let close_prices: Vec<f64> = candles.iter().map(|c| c.close).collect();
    let highs: Vec<f64> = candles.iter().map(|c| c.high).collect();
    let lows: Vec<f64> = candles.iter().map(|c| c.low).collect();

    // EMA 200 (tendencia)
    let ema200 = ema(&close_prices, 200);
    let last_price = close_prices.last().unwrap();
    let last_ema = *ema200.last().unwrap();

    // SMA High/Low para canal SSL
    let sma_high = sma(&highs, 10);
    let sma_low = sma(&lows, 10);

    // HLV: detección de cruce SSL
    let mut hlv = vec![0];
    for i in 1..sma_high.len() {
        let prev = *hlv.last().unwrap();
        let hlv_val = if close_prices[i] > sma_high[i] {
            1
        } else if close_prices[i] < sma_low[i] {
            -1
        } else {
            prev
        };
        hlv.push(hlv_val);
    }

    let last_hlv = *hlv.last().unwrap();
    let prev_hlv = hlv[hlv.len() - 2];

    let last_sma_high = sma_high.last().unwrap();
    let last_sma_low = sma_low.last().unwrap();

    let strategy_name = "ssl_ema";

    // Señal LONG: precio > EMA200 y cruce +1
    if *last_price > last_ema && prev_hlv == -1 && last_hlv == 1 {
        let sl = last_sma_low;
        log_event("signal", "-", "SSL EMA señal de compra", json!({
            "precio": last_price,
            "sl": sl,
            "ema": last_ema
        }));
        return Some(TradeSignal {
            direction: SignalDirection::Long,
            entry_price: *last_price,
            stop_loss: *sl,
            strategy: strategy_name,
        });
    }

    // Señal SHORT: precio < EMA200 y cruce -1
    if *last_price < last_ema && prev_hlv == 1 && last_hlv == -1 {
        let sl = last_sma_high;
        log_event("signal", "-", "SSL EMA señal de venta", json!({
            "precio": last_price,
            "sl": sl,
            "ema": last_ema
        }));
        return Some(TradeSignal {
            direction: SignalDirection::Short,
            entry_price: *last_price,
            stop_loss: *sl,
            strategy: strategy_name,
        });
    }

    None
}

// Función de promedio móvil simple
fn sma(data: &[f64], period: usize) -> Vec<f64> {
    data.windows(period)
        .map(|w| w.iter().sum::<f64>() / period as f64)
        .collect()
}

// Función de promedio móvil exponencial
fn ema(data: &[f64], period: usize) -> Vec<f64> {
    let mut result = vec![];
    let k = 2.0 / (period as f64 + 1.0);
    let mut ema_prev = data.iter().take(period).sum::<f64>() / period as f64;
    result.push(ema_prev);
    for price in &data[period..] {
        ema_prev = price * k + ema_prev * (1.0 - k);
        result.push(ema_prev);
    }
    result
}
