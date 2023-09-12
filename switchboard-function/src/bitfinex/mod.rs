use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct BitfinexCandle {
    mts: i64,
    open: f64,
    close: f64,
    high: f64,
    low: f64,
    volume: f64,
}

pub fn bitfinex_close_average(candles: &Vec<BitfinexCandle>) -> Decimal {
    if candles.len() == 0 {
        return Decimal::ZERO;
    }

    let sum: Decimal = candles
        .iter()
        .map(|data| Decimal::from_f64(data.close).unwrap())
        .sum();

    let twap = sum / Decimal::from_usize(candles.len()).unwrap();
    twap
}
