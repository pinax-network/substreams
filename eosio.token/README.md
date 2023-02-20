# Antelope `eosio.token` Substream

> Antelope `eosio.token` token transfers

### [Latest Releases](https://github.com/pinax-network/substreams/releases)

### Quickstart

```bash
$ make
$ make run
```

### Mermaid graph

```mermaid
graph TD;
  map_transfers[map: map_transfers]
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> map_transfers
  map_accounts[map: map_accounts]
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> map_accounts
  map_stat[map: map_stat]
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> map_stat
  log_out[map: log_out]
  map_transfers --> log_out
```

### Modules

```yaml
Package name: eosio_token
Version: v0.4.1
Doc: Antelope `eosio.token` based action traces & database operations.
Modules:
----
Name: map_transfers
Initial block: 0
Kind: map
Output Type: proto:antelope.eosio.token.v1.TransferEvents
Hash: b10a4a6d64558e639a1f571f256fae2ddf5f4edf

Name: map_accounts
Initial block: 0
Kind: map
Output Type: proto:antelope.eosio.token.v1.Accounts
Hash: cce87730d9af37f64afa65d6fa706c7fe121b680

Name: map_stat
Initial block: 0
Kind: map
Output Type: proto:antelope.eosio.token.v1.Stats
Hash: da0cf92815bce9889ea89e54dd636b7e11f3c9d2

Name: log_out
Initial block: 0
Kind: map
Output Type: proto:pinax.substreams.sink.winston.v1.LoggerOperations
Hash: 159780ff24f2b1b00dd7001bfe54726382e1be96
```