pub mod kraken;
pub use kraken::*;

use chrono::Utc;
use ethers::{
    prelude::{abigen, SignerMiddleware},
    providers::{Http, Provider},
    types::Address,
};
use hex;
use serde::Deserialize;
use switchboard_evm::sdk::EVMFunctionRunner;
pub use switchboard_utils::reqwest;
use rust_decimal::Decimal;

abigen!(
    Receiver,
    r#"[ function callback(uint256[], address[], bytes32[], bytes[]) ]"#,
);
static DEFAULT_URL: &str = "https://goerli-rollup.arbitrum.io/rpc";

#[derive(Debug, Deserialize)]
pub struct DeribitRespnseInner {
    pub mark_iv: f64,
    pub timestamp: u64,
}
#[derive(Debug, Deserialize)]
pub struct DeribitResponse {
    pub result: DeribitRespnseInner,
}

#[tokio::main(worker_threads = 12)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = 0;
    // --- Initialize clients ---
    let function_runner = EVMFunctionRunner::new()?;
    let receiver: Address = env!("EXAMPLE_PROGRAM").parse()?;
    let provider = Provider::<Http>::try_from(DEFAULT_URL)?;
    let signer = function_runner.enclave_wallet.clone();
    let client = SignerMiddleware::new_with_provider_chain(provider, signer).await?;
    let receiver_contract = Receiver::new(receiver, client.into());

    // --- Logic Below ---
    // DERIVE CUSTOM SWITCHBOARD PRICE
    Ok(())
}

/// Run `cargo test -- --nocapture`
#[cfg(test)]
mod tests {
    use crate::*;

    #[tokio::test]
    async fn test() -> Result<(), Box<dyn std::error::Error>> {
        let kraken_url = "https://api.kraken.com/0/public/OHLC?pair=BTCUSD&interval=1";
        let kraken_ohlc: KrakenOHLCResponse = reqwest::get(kraken_url).await?.json().await?;
        let mut close_prices: Vec<Decimal> = kraken_ohlc.parse("XXBTZUSD").unwrap().iter().map(|x| x.close).collect();
        close_prices.reverse();
        let hour_prices = &close_prices[..60];
        let avg: Decimal = hour_prices.iter().sum::<Decimal>() / Decimal::from(hour_prices.len());
        println!("{:?}", avg);
        Ok(())
    }
}
