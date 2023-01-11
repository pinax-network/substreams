# `Substreams` for **Antelope**

[![Build Status](https://github.com/EOS-Nation/substreams-antelope/actions/workflows/test.yml/badge.svg)](https://github.com/EOS-Nation/substreams-antelope/actions/workflows/test.yml)
![License](https://img.shields.io/github/license/EOS-Nation/substreams-antelope)

> `Substreams` made for Antelope based chains.

### Quickstart

```
$ substreams run -e eos.firehose.eosnation.io:9001 substreams.yaml map_action_traces -s 284958698
```

## Substreams

| Substream      | Description |
|----------------|-------------|
| [`eosio.tokens`](substreams/eosio.token)  | Maps & Store module for `eosio.token` events
| [`common`](substreams/common)  | Common Maps module for all events

### Endpoints

| Chain       | Host     |
|-------------|----------|
| EOS         | `eos.firehose.eosnation.io:9001`
| WAX         | `wax.firehose.eosnation.io:9001`
| UX          | `ux.firehose.eosnation.io:9001`
| Telos       | `telos.firehose.eosnation.io:9001`
| Ore         | `ore.firehose.eosnation.io:9001`

### Endpoints (Testnets)

| Chain         | Host     |
|---------------|----------|
| WAX Testnet   | `waxtest.firehose.eosnation.io:9001`
| Jungle 4      | `jungle4.firehose.eosnation.io:9001`
| Kylin         | `kylin.firehose.eosnation.io:9001`
| Telos Testnet | `telostest.firehose.eosnation.io:9001`
| Ore Stage     | `orestage.firehose.eosnation.io:9001`

### Further resources

- [Substreams documentation](https://substreams.streamingfast.io)

### Build Protobuf

Generate protobuf code

```
$ substreams protogen ./substreams.yaml --exclude-paths="sf/antelope,sf/substreams,google"
```

To include **/src/pb/mod.rs**

```rs
#[path = "antelope.common.v1.rs"]
#[allow(dead_code)]
pub mod common;
```

### Build & Pack

```bash
$ cargo build --target wasm32-unknown-unknown --release
$ substreams pack ./substreams.yaml
```
