# Fluxus Contracts

Here we have all the Fluxus smart contract code. This repository has three main parts:

 -> Token-Contract

 -> Exchange-Contract

 -> Farming-Contract.

&nbsp;

## Contracts

| Contract                   | Description                                                                                          |
| -------------------------- | ---------------------------------------------------------------------------------------------------- |
| [test-token](test-token)   | Contract to do a simple fungible token.                                                              |
| [ref-router](ref-router)   | Main exchange contract, that allows to deposit and withdraw tokens, exchange them via various pools. |
| [fluxus-farm](fluxus-farm) | Main farm contract, that allows to create farms and do stake operations.                             |
|                            |                                                                                                      |

&nbsp;


## Essential installation

1. Install `rustup` via https://rustup.rs/

Run the following:
```
rustup default stable
rustup target add wasm32-unknown-unknown
```

2. Install `near-cli` via https://docs.near.org/docs/tools/near-cli

Run the following:
```
npm install -g near-cli
```
&nbsp;

## Testing

To run contract unit tests, use:

For the exchange part:
```
cd ref-router
cargo test 
```

For the farming part:
```
cd fluxus-farm
cargo test 
```

&nbsp;

## Compiling

To compile the code, you can use:

For the exchange part:
```
cd ref-router
./build_local.sh
```
For the farming part:
```
cd fluxus-farm
./build_local.sh
```
For the token part:
```
cd test-token
./build.sh
```

&nbsp;

## Deploying to TestNet

To deploy the contracts to TestNet, you can use the following:

Deploying the farm contract in a dev near account:
```
near dev-deploy --wasmFile ../target/wasm32-unknown-unknown/release/ref_farming.wasm
```

Deploying the exchange contract in a dev near account:
```
near dev-deploy --wasmFile ../target/wasm32-unknown-unknown/release/ref-router.wasm
```

Deploying the token contract in a dev near account:
```
near dev-deploy --wasmFile ../target/wasm32-unknown-unknown/release/test_token.wasm
```


You can see the contract Id after use de dev-deploy command. Copy it because it will be used later. 
Remember thar this contract Id can also be found in the folder `neardev`.

It is also cool to say that if you want to deploy something in an specific contract Id, you need to use the:
```
near deploy
```
instead the:
```
near dev-deploy
```

Example:
```
near deploy --wasmFile ../target/wasm32-unknown-unknown/release/ref-router.wasm --accountId juninho123.testnet
```
