pub mod kraken;
pub use kraken::*;
pub mod coinbase;
pub use coinbase::*;
pub mod bitfinex;
pub use bitfinex::*;

use reqwest::Client;
use serde_json::Value;
use ethers::{
    prelude::{abigen, SignerMiddleware},
    providers::{Http, Provider},
    types::Address,
};
use hex;
use serde::Deserialize;
use switchboard_evm;
use switchboard_evm::sdk::EVMFunctionRunner;
pub use switchboard_utils::reqwest;
use rust_decimal::Decimal;
use std::time::Instant;
use chrono::{Utc, Duration};

abigen!(
    Receiver,
    r#"[ function callback(uint256[], address[], bytes32[], bytes[]) ]"#,
);
static DEFAULT_URL: &str = "https://goerli-rollup.arbitrum.io/rpc";

pub fn kraken_twap(kraken_ohlc: &KrakenOHLCResponse, pair: &str, window: usize) -> Decimal {
    let mut close_prices: Vec<Decimal> = kraken_ohlc.parse(pair).unwrap().iter().map(|x| x.close).collect();
    close_prices.reverse();
    let hour_prices = &close_prices[..window];
    let avg: Decimal = hour_prices.iter().sum::<Decimal>() / Decimal::from(hour_prices.len());
    avg
}

pub async fn perform() -> Result<(), Box<dyn std::error::Error>> {
    // --- Initialize clients ---
    let function_runner = EVMFunctionRunner::new()?;
    let receiver: Address = env!("EXAMPLE_PROGRAM").parse()?;
    let provider = Provider::<Http>::try_from(DEFAULT_URL)?;
    let signer = function_runner.enclave_wallet.clone();
    let client = SignerMiddleware::new_with_provider_chain(provider, signer).await?;
    let receiver_contract = Receiver::new(receiver, client.into());

    // --- Logic Below ---
    // DERIVE CUSTOM SWITCHBOARD PRICE
    let kraken_url = "https://api.kraken.com/0/public/OHLC?pair=BTCUSD&interval=1";
    let kraken_ohlc: KrakenOHLCResponse = reqwest::get(kraken_url).await?.json().await?;
    let kraken_twap = kraken_twap(&kraken_ohlc, "XXBTZUSD", 60);

    let bitfinex_url = "https://api-pub.bitfinex.com/v2/candles/trade:1m:tBTCUSD/hist?limit=60";
    let bitfinex_ohlc: Vec<BitfinexCandle> = reqwest::get(bitfinex_url).await?.json().await?;
    let bitfinex_twap = bitfinex_close_average(&bitfinex_ohlc);

    let end_time = Utc::now();
    let start_time = end_time - Duration::minutes(60);
    let coinbase_url = format!(
        "https://api.pro.coinbase.com/products/BTC-USD/candles?granularity=60&start={}&end={}",
        start_time.to_rfc3339(),
        end_time.to_rfc3339()
    );
    let client = Client::new();
    let coinbase_ohlc: Vec<CoinbaseCandle> = client.get(coinbase_url)
        .header("User-Agent", "null")
        .send()
        .await?
        .json().await?;
    let coinbase_twap = coinbase_close_average(&coinbase_ohlc);



    println!("BTC 1h TWAP: Kraken: {:?} Bitfinex: {:?} Coinbase: {:?}", kraken_twap, bitfinex_twap, coinbase_twap);
    Ok(())
}

#[tokio::main(worker_threads = 12)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    perform().await?;
    Ok(())
}

/// Run `cargo test -- --nocapture`
#[cfg(test)]
mod tests {
    use crate::*;

    #[tokio::test]
    async fn test() -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        switchboard_evm::test::init_test_runtime();
        perform().await?;
        let duration = start.elapsed().as_secs();
        if duration > 5 {
            println!("Warning: your function takes excessive runtime. Oracles may opt to kill your function before completion");
        }
        Ok(())
    }
}
