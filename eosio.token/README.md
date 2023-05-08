# Antelope `eosio.token` Substream

> Antelope `eosio.token` token transfers with params

### [Latest Releases](https://github.com/pinax-network/substreams/releases)


### Params
Params allow to filter messages and can be supplied to modules in the form of URL query, i.e. `to=swap.defi&contract=eosio.token` - filter EOS transfers to `swap.defi` account
- `map_transfers` - filter transfers
  - `to` - receiver
  - `from` - sender
  - `symcode` - symbol code
  - `contract` - token contract
  - `to_or_from` - either receiver or sender
- `map_accounts` - filter account balance changes
  - `account` - account
  - `symcode` - symbol code
  - `contract` - token contract
- `map_stat` - filter token stats
  - `symcode` - symbol code
  - `contract` - token contract

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
  graph_out[map: graph_out];
  map_transfers --> graph_out;
  kv_out[map: kv_out];
  map_accounts --> kv_out;
  map_stat --> kv_out;
```

### Modules

```yaml
Package name: eosio_token
Version: v0.10.0
Doc: Antelope `eosio.token` based action traces & database operations.
Modules:
----
Name: map_transfers
Initial block: 0
Kind: map
Output Type: proto:antelope.eosio.token.v1.TransferEvents
Hash: d331e4b9f5d02495638bd6e8caa638a6a8e56740

Name: map_accounts
Initial block: 0
Kind: map
Output Type: proto:antelope.eosio.token.v1.Accounts
Hash: b75af7789f4a7055b166c9ca61deca146f4ef23e

Name: map_stat
Initial block: 0
Kind: map
Output Type: proto:antelope.eosio.token.v1.Stats
Hash: 1adb9207a8728f7707ee6a70541c6525ba8c1b64

Name: graph_out
Initial block: 0
Kind: map
Output Type: proto:substreams.entity.v1.EntityChanges
Hash: 10fe3d0d70d4794af36a03cd67d55a428e40f21d

Name: kv_out
Initial block: 0
Kind: map
Output Type: proto:sf.substreams.sink.kv.v1.KVOperations
Hash: f0e646700ff89e5eb89e5708a4c63685ef5fad9e
```