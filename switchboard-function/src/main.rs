pub mod kraken;
pub use kraken::*;
pub mod coinbase;
pub use coinbase::*;
pub mod bitfinex;
pub use bitfinex::*;

use chrono::{Utc};
use ethers::{
    prelude::{abigen, SignerMiddleware},
    providers::{Http, Provider},
    types::Address,
};

use reqwest::Client;
use switchboard_evm;
use switchboard_evm::sdk::EVMFunctionRunner;
pub use switchboard_utils::reqwest;
use std::time::Instant;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

abigen!(
    Receiver,
    r#"[ function callback(uint256) ]"#,
);
static DEFAULT_URL: &str = "https://goerli-rollup.arbitrum.io/rpc";

pub async fn perform() -> Result<()> {
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

    let coinbase_url = "https://api.pro.coinbase.com/products/BTC-USD/candles?granularity=60";
    let client = Client::new();
    let coinbase_ohlc: Vec<CoinbaseCandle> = client
        .get(coinbase_url)
        .header("User-Agent", "null")
        .send()
        .await?
        .json()
        .await?;
    let coinbase_twap = coinbase_close_average(&coinbase_ohlc[..60]);

    println!(
        "BTC 1h TWAP: Kraken: {:?} Bitfinex: {:?} Coinbase: {:?}",
        kraken_twap, bitfinex_twap, coinbase_twap
    );
    let mut twaps = vec![kraken_twap, bitfinex_twap, coinbase_twap];
    twaps.sort();
    let mut lower_bound_median = twaps[twaps.len() / 2];
    lower_bound_median.rescale(8);

    // --- Send the callback to the contract with Switchboard verification ---
    let callback = receiver_contract.callback(lower_bound_median.mantissa().into());
    let expiration = (Utc::now().timestamp() + 120).into();
    let gas_limit = 5_500_000.into();
    function_runner.emit(receiver, expiration, gas_limit, vec![callback])?;
    Ok(())
}

#[tokio::main(worker_threads = 12)]
async fn main() -> Result<()> {
    perform().await?;
    Ok(())
}

/// Run `cargo test -- --nocapture`
#[cfg(test)]
mod tests {
    use crate::*;

    #[tokio::test]
    async fn test() -> Result<()> {
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
