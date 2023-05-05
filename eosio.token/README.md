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
  db_out[map: db_out]
  map_transfers --> db_out
  kv_out[map: kv_out]
  map_accounts --> kv_out
  map_stat --> kv_out
```

### Modules

```yaml
Package name: eosio_token
Version: v0.4.3
Doc: Antelope `eosio.token` based action traces & database operations.
Modules:
----
Name: map_transfers
Initial block: 0
Kind: map
Output Type: proto:antelope.eosio.token.v1.TransferEvents
Hash: fb506f775bc26e4b712e88b7114d8707fb293c77

Name: map_accounts
Initial block: 0
Kind: map
Output Type: proto:antelope.eosio.token.v1.Accounts
Hash: e8ef91e0d9e5fc3d0d0f783e0061d8b681998f76

Name: map_stat
Initial block: 0
Kind: map
Output Type: proto:antelope.eosio.token.v1.Stats
Hash: c3b29d315f8b7c241863c049c28411cb15f42191

Name: log_out
Initial block: 0
Kind: map
Output Type: proto:pinax.substreams.sink.winston.v1.LoggerOperations
Hash: d45c04ae8efb844708292b5ed4309dd52bf1f85b

Name: db_out
Initial block: 0
Kind: map
Output Type: proto:sf.substreams.sink.database.v1.DatabaseChanges
Hash: d006c72bdb50e27e9f14ebfbaae9c99d8a132400

Name: kv_out
Initial block: 0
Kind: map
Output Type: proto:sf.substreams.sink.kv.v1.KVOperations
Hash: e5edb476b4cd46b756be5d146646070d0a12b95e
```