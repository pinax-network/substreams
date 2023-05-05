# Antelope `eosio.token` Substream

> Antelope `eosio.token` token transfers with params

### [Latest Releases](https://github.com/pinax-network/substreams/releases)

### Quickstart

```bash
$ make
$ make gui        # all transfers
$ make param      # swap.defi transfers
```

### Mermaid graph

```mermaid
graph TD;
  map_transfers[map: map_transfers];
  map_transfers:params[params] --> map_transfers;
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> map_transfers;
  map_accounts[map: map_accounts];
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> map_accounts;
  map_stat[map: map_stat];
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> map_stat;
  log_out[map: log_out];
  map_transfers --> log_out;
  db_out[map: db_out];
  map_transfers --> db_out;
  kv_out[map: kv_out];
  map_accounts --> kv_out;
  map_stat --> kv_out;
```

### Modules

```yaml
Package name: eosio_token
Version: v0.7.0
Doc: Antelope `eosio.token` based action traces & database operations.
Modules:
----
Name: map_transfers
Initial block: 0
Kind: map
Output Type: proto:antelope.eosio.token.v1.TransferEvents
Hash: b6ef404f2e10055d96675fc1050fff0096fd7e68

Name: map_accounts
Initial block: 0
Kind: map
Output Type: proto:antelope.eosio.token.v1.Accounts
Hash: 599cfd62f88ba42b91da5116203939a169fe4879

Name: map_stat
Initial block: 0
Kind: map
Output Type: proto:antelope.eosio.token.v1.Stats
Hash: 6410da38054c9e33c2cfecff09b5027140511986

Name: log_out
Initial block: 0
Kind: map
Output Type: proto:pinax.substreams.sink.winston.v1.LoggerOperations
Hash: 75b483baa168c2d95ed3e3de769e81a4c0cb4436

Name: db_out
Initial block: 0
Kind: map
Output Type: proto:sf.substreams.sink.database.v1.DatabaseChanges
Hash: 867d38761adc34081aa168755c583e952e6f3fe9

Name: kv_out
Initial block: 0
Kind: map
Output Type: proto:sf.substreams.sink.kv.v1.KVOperations
Hash: 365bacadad89c5408a99456580e51175cd9043c6
```