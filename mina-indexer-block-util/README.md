# mina-indexer-block-util

Keep your Mina block data up to date!

This tool is intended to support indexing of the [Mina blockchain](https://github.com/MinaProtocol/mina) via the [mina-indexer](https://github.com/Granola-Team/mina-indexer), but can be used by anyone who wants to download Mina blocks from a GCP bucket via gsutil.

## Quick start

Clone the repo

```sh
git clone git@github.com:Isaac-DeFrain/fn.git
cd fn
```

To stay up-to-date with `mainnet` blocks from the o1-labs [`mina_network_block_data` bucket](https://console.cloud.google.com/storage/browser/mina_network_block_data), see `Info` level logs, and you don't mind if these blocks are stored in your `$HOME` dir, do

```sh
RUST_LOG=info cargo run --release --bin mina-indexer-block-util -- new-only
```

For more options, see

```sh
cargo run --bin mina-indexer-block-util -- --help
```
