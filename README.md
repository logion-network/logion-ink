# logion-ink
Logion Smart Contract library for Ink!

## Development setup

Refer to Astar Docs [ink! environment](https://docs.astar.network/docs/build/environment/ink_environment) for details.

In a nutshell, for Ubuntu:

```shell
cargo install cargo-dylint dylint-link
cargo install cargo-contract --force --locked
```

## Test

```shell
cd contracts/logion_psp34
cargo test
```

## Build

```shell
cd contracts/logion_psp34
cargo build contract --release
```

## Deploy

### Prerequisite
* Get some SHY to your Polkadot account with the [Shibuya faucet](https://portal.astar.network/shibuya-testnet/assets).

### Manual, with Contract UI
* Go to [Contract UI](https://contracts-ui.substrate.io/?rpc=wss://rpc.shibuya.astar.network) and connect to Astar Shibuya.
* Click "Add New Contract".
* Click "Upload New Contract".
* Choose an Account with enough SHY.
* Choose a Contract Name.
* Under "Upload Contract Bundle" select `logion_psp34.contract` under `target/ink/logion_psp34`.
* Provide a value for `nonce`, `collectionLocId` and `certHost`.
* Click "Next" and review the summary.
* Click "Upload and Instantiate".
* Sign

### Manual, with polkadot.js
A similar procedure is available on Polkadot-JS:
* Use [polkadot.js](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frpc.shibuya.astar.network#/contracts) to connect to Shibuya.
* Click "Developers > Contracts" to access a UI similar to Contract UI.
* Follow the same steps.

**WARNING** On 8/9/2023 attempt, this method did not succeed to deploy.

## Use a Contract

In order to use a contract that someone else deployed, you need to know the **Contract Address** and the **ABI metadata** (json):

* Go to [Contract UI](https://contracts-ui.substrate.io/?rpc=wss://rpc.shibuya.astar.network) and connect to Astar Shibuya.
* Click "Add New Contract".
* Click "Use On-Chain Contract Address".
* Provide the Contract Address, Name
* Click "Upload Metadata" and select `logion_psp34.json` under `target/ink/logion_psp34`.
* Click "Add Contract".

## Publish

_Note: It's not possible at the moment (Sep 2023) to publish a package to crates.io, due to the dependency to openbrush._

1. Update the `version` in [Cargo.toml](logics/Cargo.toml) 
2. Commit, tag (`v0.1.0` for `version = "0.1.0"`) and push.

## Deployment Log
| Date       | URL                                                                                        | Contract Address                                |
|------------|--------------------------------------------------------------------------------------------|-------------------------------------------------|
| 2023-09-06 | https://contracts-ui.substrate.io/contract/XyNVZ92vFrYf4rCj8EoAXMRWRG7okRy7gxhn167HaYQZqTc | XyNVZ92vFrYf4rCj8EoAXMRWRG7okRy7gxhn167HaYQZqTc |
| 2023-09-08 | https://contracts-ui.substrate.io/contract/ZEoCdLf7zXHNw2BEtzL6mptzHKKFWAc2jYyeuRkVQUW7ddw | ZEoCdLf7zXHNw2BEtzL6mptzHKKFWAc2jYyeuRkVQUW7ddw |
| 2023-09-08 | https://contracts-ui.substrate.io/contract/ai7fX8imUNnUaJfPxQMHhGagp9Bx7YGKxF3Xdnoe7tcUXbU | ai7fX8imUNnUaJfPxQMHhGagp9Bx7YGKxF3Xdnoe7tcUXbU |
| 2023-09-13 | https://contracts-ui.substrate.io/contract/YvRHB73h5oa1x5ALrTrz5iA8g58iZausvrg9B5BmYnD2Em4 | YvRHB73h5oa1x5ALrTrz5iA8g58iZausvrg9B5BmYnD2Em4 |

