# Switchboard Function Example

<div align="center">
  <img src="https://github.com/switchboard-xyz/sbv2-core/raw/main/website/static/img/icons/switchboard/avatar.png" />

  <h1>Switchboard<br>BSX Function Example</h1>

  <p>
    <a href="https://discord.gg/switchboardxyz">
      <img alt="Discord" src="https://img.shields.io/discord/841525135311634443?color=blueviolet&logo=discord&logoColor=white" />
    </a>
    <a href="https://twitter.com/switchboardxyz">
      <img alt="Twitter" src="https://img.shields.io/twitter/follow/switchboardxyz?label=Follow+Switchboard" />
    </a>
  </p>
</div>

## Table of Content

- [Prerequisites](#prerequisites)
  - [Installing Docker](#installing-docker)
  - [Docker Setup](#docker-setup)
- [Components](#components)
  - [Contract](#contract)
  - [Switchboard Function](#switchboard-function)
  - [Publishing and Initialization](#publishing-and-initialization)
  - [Adding Funding to Function](#adding-funding-to-function)
  - [Printing Function Data](#printing-function-data)
- [Writing Switchboard Rust Functions](#writing-switchboard-rust-functions)
  - [Setup](#setup)
  - [Minimal Example](#minimal-switchboard-function)
  - [Testing your function](#testing-your-function)
  - [Deploying and maintenance](#deploying-and-maintenance)
- [Writing Receiver Contracts](#writing-receiver-contracts)
  - [Receiver Example](#receiver-example)

## Prerequisites

Before you can build and run the project, you'll need to have Docker installed on your system. Docker allows you to package and distribute applications as lightweight containers, making it easy to manage dependencies and ensure consistent behavior across different environments. Switchboard Functions are built and run within containers, so you'll need a docker daemon running to publish a new function.

### Installing Docker

If you don't have Docker installed, you can follow these steps to get it up and running:

1. **Linux**: Depending on your Linux distribution, you might need to use different package managers. For Ubuntu, you can use `apt`:

   ```bash
   sudo apt update
   sudo apt install docker.io
   ```

   For other distributions, consult your package manager's documentation.

2. **macOS**: You can install Docker Desktop for macOS by downloading the installer from the [Docker website](https://www.docker.com/products/docker-desktop) and following the installation instructions.

3. **Windows**: Similarly, you can install Docker Desktop for Windows from the [Docker website](https://www.docker.com/products/docker-desktop) and follow the provided instructions.

### Docker Setup

After installing Docker, make sure it's running by opening a terminal/command prompt and running:

```bash
docker --version
```

This should display the installed Docker version, confirming that Docker is installed and running properly.

You'll need to login to docker. If you don't yet have an account, you'll need one to publish images to dockerhub. You can sign up at [https://hub.docker.com](https://hub.docker.com).

```bash
docker login --username <your-username> --password <your-password>
```

### Install script dependencies

```bash
pnpm i
```

### Install Switchboard cli

```bash
npm i -g @switchboard-xyz/cli@latest
sb evm function --help
```

## Components

### Notable files
- When looking contract side, the heart of the example code lives in https://github.com/switchboard-xyz/bsx-example/blob/main/contracts/src/example1/receiver/Receiver.sol
- When looking function side, the portion to focus on lives in https://github.com/switchboard-xyz/bsx-example/blob/main/switchboard-function/src/main.rs

### Contract

This example contract acts as the ingestor of the switchboard-function in this directory to fetch implied volatility parameters via deribit. The example contract is an example of the [ERC2535 diamond contract pattern](https://autifynetwork.com/exploring-erc-2535-the-diamond-standard-for-smart-contracts/) so it can be extended and upgraded for your needs.

When you deploy this contract, it will await to be bound to a switchboard function calling into it.

#### Picking a network and setting up your environment

- set the `SWITCHBOARD_ADDRESS` env variable to target whichever address is appropriate for the network you're targeting
- for arbitrum testnet, this is: `0xA3c9F9F6E40282e1366bdC01C1D30F7F7F58888e`

To first deploy the contract, run:

```bash
# ex:
# pnpm deploy:coredaotestnet
# pnpm deploy:coredaomain
# pnpm deploy:arbitrumtestnet
pnpm deploy:${NETWORK_NAME}
```

More deploy commands are available in [package.json](./package.json) scripts.

You will see the last line of this script output

```bash
export EXAMPLE_PROGRAM=<RECEIVER_ADDRESS>
```

### Switchboard Function

Export the address to your environment and navigate to `./switchboard-function/`

The bulk of the function logic can be found in [./switchboard-function/src/main.rs](switchboard-function/src/main.rs).

Build functions from the `switchboard-function/` directory with

```bash
cd switchboard-function
make build
```

### Publishing and Initialization

You'll also need to pick a container name that your switchboard function will use on dockerhub.

```bash
export CONTAINER_NAME=your_docker_username/switchboard-function
export EXAMPLE_PROGRAM=<RECEIVER_ADDRESS>
```

Here, set the name of your container and deploy it using:

```bash
cd switchboard-function
export CONTAINER_NAME=your_docker_username/switchboard-function
export EXAMPLE_PROGRAM=<RECEIVER_ADDRESS>
make publish
```

`NOTE: Make sure your docker build is publically readable`

After this is published, you are free to make your function account to set the rate of run for the function.

### Initializing the function

You can use the Switchboard cli to bind this docker container to an on-chain representation:

```bash
export SWITCHBOARD_ADDRESS_ARBITRUM_TESTNET=0xA3c9F9F6E40282e1366bdC01C1D30F7F7F58888e
export QUEUE_ADDRESS=0x54f8A91bE5baAD3E2368b00A11bF4012EA6b031F # default testnet queue
export MEASUREMENT=<YOUR CONTAINER MEASUREMENT>
sb evm function create ${QUEUE_ADDRESS?} --container ${CONTAINER_NAME?} --schedule "*/30 * * * * *" --containerRegistry dockerhub  --mrEnclave ${MEASUREMENT?} --name "BSX_example" --fundAmount 0.025 --chain arbitrum --account /path/to/signer --network testnet --programId ${SWITCHBOARD_ADDRESS_ARBITRUM_TESTNET?}
```

### Adding Funding to Function

Add funds to your function by doing the following:

```bash
sb evm function fund $FUNCTION_ID --fundAmount .1 --chain $CHAIN --account /path/to/signer --network $CLUSTER --programId $SWITCHBOARD_ADDRESS_ARBITRUM_TESTNET
```

### Printing the state of your callback

This repo contains an example script to view the current verified deribit implied volatility info
currently in contract:

```bash
npx hardhat run --network arbitrumTestnet scripts/get_state.ts
```

## Writing Switchboard Rust Functions

In order to write a successfully running switchboard function, you'll need to import `switchboard-evm` to use the libraries which communicate the function results (which includes transactions to run) to the Switchboard Verifiers that execute these metatransactions.

### Setup

Cargo.toml

```toml
[package]
name = "function-name"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "function-name"
path = "src/main.rs"

[dependencies]
tokio = "^1"
futures = "0.3"

# at a minimum you'll need to include the following packages
ethers = { version = "2.0.7", features = ["legacy"] }
switchboard-evm = "0.3.9"
```

### Minimal Switchboard Function

main.rs

```rust
use ethers::{
    prelude::{abigen, SignerMiddleware, ContractCall},
    providers::{Http, Provider},
    types::{U256},
};
use rand;
use std::sync::Arc;
use std::time::{SystemTime, Duration};
use switchboard_evm::{
    sdk::{EVMFunctionRunner, EVMMiddleware},
};

#[tokio::main(worker_threads = 12)]
async fn main() {

    // define the abi for the functions in the contract you'll be calling
    // -- here it's just a function named "callback", expecting a random u256
    abigen!(
        Receiver,
        r#"[
            function callback(uint256)
        ]"#,
    );

    // Generates a new enclave wallet, pulls in relevant environment variables
    let function_runner = EVMFunctionRunner::new().unwrap();

    // set the gas limit and expiration date
    let gas_limit = 1000000;
    let expiration_time_seconds = 60;
    let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_secs() + 64;


    // create a client, wallet and middleware. This is just so we can create the contract instance and sign the txn.
    // @TODO: update the provider to whichever network you're using
    let provider = Provider::<Http>::try_from("https://goerli-rollup.arbitrum.io/rpc").unwrap();
    let client = Arc::new(
        SignerMiddleware::new_with_provider_chain(provider.clone(), function_runner.enclave_wallet.clone())
            .await
            .unwrap(),
    );

    // @TODO: your target contract address here
    // In the push receiver example this is set via environment variable
    let contract_address = "0x1cEA45f047FEa89887B79ce28723852f288eE09B"
        .parse::<ethers::types::Address>()
        .unwrap();
    let receiver_contract = Receiver::new(contract_address, client);

    // generate a random number U256
    let random: [u64; 4] = rand::random();
    let random = U256(random);

    // call function
    let contract_fn_call: ContractCall<EVMMiddleware<_>, _> =
        receiver_contract.callback(random);

    // create a vec of contract calls to pass to the function runner
    let calls = vec![contract_fn_call.clone()];

    // Emit the result
    // This will encode and sent the function data and run them as metatransactions
    // (in a single-tx) originating from the switchboard contract with the functionId encoded as the sender
    // https://eips.ethereum.org/EIPS/eip-2771
    function_runner.emit(
        contract_address,
        current_time.try_into().unwrap(),
        gas_limit.into(),
        calls,
    ).unwrap();
}
```

### Testing your function

We can't guarantee that the function will run on the blockchain, but we can test that it compiles and runs locally.

Run the following to test your function:

```bash
export EXAMPLE_PROGRAM=${SWITCHBOARD_ADDRESS?} # can be any valid address
cargo test -- --nocapture # Note: this will include a warning about a missing quote which can be safely ignored.
```

Successful output:

```bash
WARNING: Error generating quote. Function will not be able to be transmitted correctly.
FN_OUT: 7b2276657273696f6e223a312c2271756f7465223a5b5d2c22666e5f6b6579223a5b3134342c32332c3233322c34342c39382c32302c39372c3232392c3138392c33302c3235322c3133362c37362c332c3136382c3130362c3138322c34352c3137352c3137325d2c227369676e6572223a5b3135382c32332c3137302c3133322c3230302c3130322c35302c38352c31302c3134382c3235322c35372c3132362c372c31372c32352c37322c3131342c38322c3134365d2c22666e5f726571756573745f6b6579223a5b5d2c22666e5f726571756573745f68617368223a5b5d2c22636861696e5f726573756c745f696e666f223a7b2245766d223a7b22747873223a5b7b2265787069726174696f6e5f74696d655f7365636f6e6473223a313639313633383836332c226761735f6c696d6974223a2235353030303030222c2276616c7565223a2230222c22746f223a5b38332c3130372c3135352c35382c39382c3132382c37332c3233392c3134382c3133332c3133342c33392c3131382c31362c34382c3235302c3130372c3133382c3234382c3135375d2c2266726f6d223a5b3135382c32332c3137302c3133322c3230302c3130322c35302c38352c31302c3134382c3235322c35372c3132362c372c31372c32352c37322c3131342c38322c3134365d2c2264617461223a5b3136302c3232332c3131392c3130362...
```

The `error` above simply means your function could not produce a secure signature for the test since it was run outside the enclave.

### Deploying and Maintenance

After you publish the function and create it on the blockchain, you must keep the function escrow account funded to cover gas fees. Revisions to the function can be made by deploying a new version and updating the function config on-chain.

## Writing Receiver Contracts

While Switchboard Functions can call back into any number of on-chain functions, it's useful to limit access to some privileged functions to just _your_ Switchboard Function.

In order to do this you'll need to know the switchboard address you're using, and which functionId will be calling into the function in question.

### Receiver Example

Recipient.sol

```sol
//SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

// EIP2771 Context
// Inherited by all contracts that are recipients of switchboard callbacks
contract Recipient {
  address immutable switchboard;

  constructor(address _switchboard) {
    switchboard = _switchboard;
  }

  // get the encoded sender if this message is coming from the switchboard contract
  // if things are working as intended, the sender will be the functionId

  function getEncodedFunctionId() internal view returns (address payable signer) {
    signer = payable(msg.sender);
    if (msg.data.length >= 20 && signer == switchboard) {
      assembly {
        signer := shr(96, calldataload(sub(calldatasize(), 20)))
      }
    }
  }
}
```

Example.sol

```sol
//SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import { Recipient } from "./Recipient.sol";

contract ReceiverExample is Recipient {
  uint256 public randomValue;
  address functionId;

  event NewRandomValue(uint256 value);

  constructor(
    address _switchboard // Switchboard contract address
  ) Recipient(_switchboard) {}

  function callback(uint256 value) external {
    // extract the sender from the callback, this validates that the switchboard contract called this function
    address msgSender = getEncodedFunctionId();

    if (functionId == address(0)) {
      // set the functionId if it hasn't been set yet
      functionId = msgSender;
    }

    // make sure the encoded caller is our function id
    if (msgSender != functionId) {
      revert("Invalid sender");
    }

    // set the random value
    randomValue = value;

    // emit an event
    emit NewRandomValue(value);
  }
}
```
