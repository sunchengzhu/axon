<p align="center">
  <a href="https://github.com/axonweb3/axon">
    <img src="./docs/assets/logo/axon-01.png" width="450">
  </a>
  <p align="center">
    <a href="https://github.com/axonweb3/axon/releases"><img src="https://img.shields.io/github/v/release/axonweb3/axon?sort=semver"></a>
    <a href = "https://hub.docker.com/r/axonweb3/axon/tags"><img src = "https://img.shields.io/docker/pulls/axonweb3/axon"></a>
    <a href="https://github.com/axonweb3/axon/blob/main/LICENSE"><img src="https://img.shields.io/badge/License-MIT-green.svg"></a>
    <a href="https://github.com/axonweb3/axon"><img src="https://github.com/axonweb3/axon/actions/workflows/web3_compatible.yml/badge.svg?branch=main"></a>
    <a href="https://github.com/axonweb3/axon"><img src="https://img.shields.io/github/contributors/axonweb3/axon"></a>
    <a href="https://twitter.com/AxonWeb3"><img src="https://img.shields.io/twitter/follow/AxonWeb3?style=social"></a>
  </p>
  <p align="center">
     Developed by AxonWeb3<br>
  </p>
</p>

## Versioning

Note this is the branch for Axon v0.3.

[Axon v0.2 (beta)](https://github.com/axonweb3/axon/tree/0.2.0-beta) is still in operation but is not being actively developed.

## What is Axon

Axon is a high-performance Layer 2 framework with native cross-chain function. Built on top of the [Overlord](https://github.com/nervosnetwork/overlord) consensus protocol and the P2P network [Tentacle](https://github.com/nervosnetwork/tentacle)
, Axon supports hundreds of nodes and achieves thousands of TPS. Axon is also EVM-compatible with well-developed toolchains. Its high interoperability facilitates cross-chain communication among dApps.

## Highlights

### Developer-Friendly Design

Axon is compatible with [Ethereum](https://ethereum.org) so that all of the develop utilities can be used on it directly. Seeing is believing, there is a [15 minutes tutorial](https://docs.axonweb3.io/getting-started/for-dapp-devs/zero_to_axon_with_axon_cli) that will lead you to build your own chain and deploy your application.

### Native Cross-Chain Communication

Openness and mobility are the foundation of social development, so is blockchain. Cross-chain function enhances liquidity for the web3 ecosystem. Axon develops native cross-chain communication without any bridge. Each Axon-based chain can speak to [CKB](https://www.nervos.org), other Axon-based chains, and any [IBC](https://ibcprotocol.org) compatible chains. Axon will embed more cross-chain protocols in the future.

### Staking on Layer 1 CKB

Axon supports a Proof-of-Stake (PoS) consensus mechanism and requires each Axon-based appchain to issue native Extensible User Defined Tokens (xUDTs), which are designed and customized by the app chain's development team and released on CKB. For simplicity, the xUDTs on Axon-based chains are referred to as Axon tokens (AT tokens) below. Holders of AT tokens can stake to become validators and/or delegate their tokens to other validators in exchange for rewards. Unlike other sidechains where staking takes place on Layer 2, Axon's staking is grounded on Layer 1 CKB. Validators and other participants stake their native AT tokens on CKB, which uses a Proof-of-Work (PoW) consensus mechanism. This unique staking design helps Axon-based appchains enjoy the highest degree of decentralization and security from Layer 1 while maintain their high performance and sovereignty as independent Layer 2 networks.

## Roadmap

Most of the infrastructure has been done and some substantial features to be developed are as below:

1. Compatible with [IBC](https://github.com/cosmos/ibc) protocol.
2. Design a cross-chain protocol for EVM-based chains.
3. Compatible with more cross-chain protocols in the future.


## Getting Started

You could build Axon from source code, or use a [docker image](https://github.com/axonweb3/axon/pkgs/container/axon) to run an Axon node.

### Compile from Source
If you perfer to build Axon from source code, please make sure that [rust](https://www.rust-lang.org/), [clang](http://clang.org/), [openssl](https://www.openssl.org/), [m4](https://www.gnu.org/software/m4/) have already been installed. Then execute the following commands:v
```bash
# Clone the Axon repository from GitHub
git clone --depth=1 https://github.com/axonweb3/axon.git && cd axon
# Build Axon in release mode
cargo build --release
# Initialize the chain
target/release/axon init \
    --config     devtools/chain/config.toml \
    --chain-spec devtools/chain/specs/single_node/chain-spec.toml
# Start a single Axon node
target/release/axon run --config devtools/chain/config.toml
```

### Working with Docker
While compiling from source can take about 20 minutes, if you want a quick start, a [docker-compose](devtools/chain/docker-compose.yml) file is provided, see [quick-start.md](devtools/chain/quick-start.md).

### Multiple nodes
For running multiple nodes, first create toml files for each node and run different nodes in separate terminals or docker containers. The metadata in the genesis transactions should be updated to include all nodes' credentials. Here is [a simple Example](https://github.com/axonweb3/axon/blob/d835d64a7df7c10dcc5c2febcbc6c63bfa850810/.github/workflows/axon-start-with-short-genesis.yml#L85-L116).

## Toolchains

Apart from the framework, Axon has:

- [Axon Faucet](https://github.com/axonweb3/axon-faucet): the faucet for the Axon-based chains.
- [Axon Explorer](https://github.com/Magickbase/blockscan): a blockchain explorer for the Axon-based chains.

## Contributing

Please read the [CONTRIBUTING.md](./CONTRIBUTING.md) for details on code of conduct, and the process for submitting pull requests. And the security policy is described in [SECURITY.md](./SECURITY.md).

## Communication

The following ways are a great spot to ask questions about Axon:

* [Email](axon@axonweb3.io)
* [Twitter](https://twitter.com/AxonWeb3)

## Socials

All Axon related accounts are displayed via [linktree](https://linktr.ee/axonweb3).

## License

This project is licensed under the MIT License - see the [LICENSE.md](./LICENSE) file for details.
