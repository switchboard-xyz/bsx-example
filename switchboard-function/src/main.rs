mod exchange_api;
pub use exchange_api::*;
use ethers::{
    prelude::{abigen, SignerMiddleware},
    providers::{Http, Provider},
    types::I256,
};
use rand;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use reqwest::Error;
use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::time::SystemTime;
use std::result::Result;
use switchboard_common;
use switchboard_common::SbFunctionError;
use switchboard_evm::*;
use switchboard_evm::sdk::EvmFunctionRunner;

// define the abi for the callback
// -- here it's just a function named "callback", expecting the feed names, values, and timestamps
// -- we also include a view function for getting all feeds
// running `npx hardhat typechain` will create artifacts for the contract
// this in particular is found at
// SwitchboardPushReceiver/artifacts/contracts/src/SwitchboardPushReceiver/Receiver/Receiver.sol/Receiver.json
abigen!(Receiver, "./src/abi/Receiver.json",);

static CLIENT_URL: &str = env!("RPC_URL");
static RECEIVER: &str = env!("RECEIVER_ADDRESS");

#[sb_error]
enum SbError {
    ParseError = 1,
    FetchError,
}

// Define the Switchboard Function - resulting in a vector of tx's to be sent to the contract
#[sb_function(expiration_seconds = 120, gas_limit = 5_500_000)]
async fn sb_function(
    client: SbMiddleware,
    _: Address,
    _: NoParams,
) -> Result<Vec<FnCall>, SbError> {

    // get the receiver contract
    let receiver: Address = RECEIVER.parse().map_err(|_| SbError::ParseError)?;
    let receiver_contract = Receiver::new(receiver, client.into());

    // get time in seconds
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // get all feeds
    let feeds = receiver_contract.get_all_feeds().call().await.unwrap();


    // take feed.feed_name and map it to feed.latest_result
    let mut feed_map = HashMap::<[u8; 32], I256>::new();
    for feed in feeds {
        feed_map.insert(feed.feed_name, feed.latest_result.value);
    }

    // get fresh prices
    let mut feed_updates = get_prices().await;

    // check if we're still registering feeds (significantly more expensive in gas cost)
    // -- if so, only use the first 20 elements of the feed_updates
    // allow up to 1 registration alongside updates so we don't block updates for an entire run if a feed is added
    let registering_feeds: bool = feed_map.len() < feed_updates.len() - 1;

    // get list of feed names that weren't received in get_feed_data
    let mut missing_feeds = Vec::<[u8; 32]>::new();
    for key in feed_map.keys() {
        // add if the feed_updates doesn't contain the key and length < 10
        if !feed_updates.contains_key(key) && missing_feeds.len() < 10 {
            missing_feeds.push(*key);
        }
    }

    // delete all entries with a diff less than 0.1
    for (key, value) in feed_updates.clone() {
        if feed_map.contains_key(&key) {
            let diff = get_percentage_diff(*feed_map.get(&key).unwrap(), value);
            // %0.01 diff should triger an update
            if registering_feeds || diff < Decimal::from_str("0.1").unwrap() {
                feed_updates.remove(&key);
            }
        }
    }

    // get a vec of feed names and values remaining
    let mut feed_names = Vec::<[u8; 32]>::new();
    let mut feed_values = Vec::<I256>::new();

    // setup feeds for shuffling
    let mut randomness = [0; 32];
    Gramine::read_rand(&mut randomness).unwrap();
    let mut rng = rand::rngs::StdRng::from_seed(randomness);
    let mut feed_updates: Vec<([u8; 32], I256)> = feed_updates.into_iter().collect();

    // only shuffle feeds if we're at the stage where we're submitting results
    if !registering_feeds {
        feed_updates.shuffle(&mut rng);
    }

    for (key, value) in feed_updates {

        // only use the first 30 elements of the feed_updates
        // -- this is to prevent the transaction from going over the gas limit
        if feed_names.len() >= 20 && registering_feeds {
            break;
        }
        if feed_names.len() >= 50 && !registering_feeds {
            break;
        }
        feed_names.push(key);
        feed_values.push(value);
    }

    // send the callback to the contract
    let callback =
        receiver_contract.callback(feed_names.clone(), feed_values.clone(), current_time.into());

    // get the calls from the output results
    let mut callbacks = vec![callback];

    // add the missing feeds to the callback to mark them as stale
    if !registering_feeds && missing_feeds.len() > 0 {
        let callback_missing_feeds = receiver_contract.failure_callback(missing_feeds.clone());
        callbacks.push(callback_missing_feeds);
    }

    // Return the Vec of callbacks to be run by the Switchboard Function on-chain
    Ok(callbacks)
}

fn get_percentage_diff(a: I256, b: I256) -> Decimal {
    let a = Decimal::from(a.as_i128());
    let b = Decimal::from(b.as_i128());
    (Decimal::min(a, b) / Decimal::max(a, b)).abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use exchange_api::*;

    #[tokio::test]
    async fn test() {
        let mut feed_updates = get_prices().await;
        for (key, value) in feed_updates.clone() {
            println!("{}: {}", String::from_utf8(key.to_vec()).unwrap(), value);
        }


    }
    
}
