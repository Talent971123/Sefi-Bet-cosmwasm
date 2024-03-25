# Dropspace-Cosmwasm-Contracts

A brief description of what this project does and who it's for mint nft using cosmwasm sdk based on cw721 and cw721-base.

## Documentation

[Documentation](https://github.com/CosmWasm/cw-nfts/tree/main)

## Getting started

You will need Rust 1.65+ with wasm32-unknown-unknown target installed.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Documentation for rust set up.

[rust installed](https://www.rust-lang.org/tools/install)

## Build and Test

You can run unit test on this via:

```bash
cargo test
```

you can run build on this via:

```bash
cargo build
```

## Compile && Deploy the code

To compile the comswasm code, we need to install docker.

[docker install](https://docs.docker.com/engine/install/ubuntu/)

### Compile

1. To compile the smart contract for deployment, usd the following:

```bash
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.13.0
```

This will create a .wasm binary file in artifacts/ folder.

### Deploy

2. To store the binary file on blockchain, use wasmd.

```bash
RES=$(wasmd tx wasm store artifacts/sefi.wasm --from wallet $TXFLAG -y --output json -b block)
echo $RES | jq .
```
