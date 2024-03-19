# Antelope `eosio.token` Substream

> Antelope `eosio.token` token transfers with params

### [Latest Releases](https://github.com/pinax-network/substreams/releases)


### Param
Param allow to filter messages and can be supplied to modules in the form of URL query.

> i.e. `to=swap.defi&contract=eosio.token&symcode=EOS` - filters EOS transfers to `swap.defi` account.

- `map_transfers` - filter transfers
  - `to` - receiver
  - `from` - sender
  - `symcode` - symbol code
  - `contract` - token contract
  - `to_or_from` - either receiver or sender
  - `quantity_lt` - quantity amount **less than**
  - `quantity_lte` - quantity amount **less than or equal to**
  - `quantity_gt` - quantity amount **greater than**
  - `quantity_gte` - quantity amount **greater than or equal to**
- `map_accounts` - filter account balance changes
  - `account` - account
  - `symcode` - symbol code
  - `contract` - token contract
  - `balance_lt` - balance amount **less than**
  - `balance_lte` - balance amount **less than or equal to**
  - `balance_gt` - balance amount **greater than**
  - `balance_gte` - balance amount **greater than or equal to**
  - `balance_delta_lt` - balance delta amount **less than**
  - `balance_delta_lte` - balance delta amount **less than or equal to**
  - `balance_delta_gt` - balance delta amount **greater than**
  - `balance_delta_gte` - balance delta amount **greater than or equal to**
- `map_stat` - filter token stats
  - `symcode` - symbol code
  - `contract` - token contract
  - `supply_lt` - supply amount **less than**
  - `supply_lte` - supply amount **less than or equal to**
  - `supply_gt` - supply amount **greater than**
  - `supply_gte` - supply amount **greater than or equal to**
  - `supply_delta_lt` - supply delta amount **less than**
  - `supply_delta_lte` - supply delta amount **less than or equal to**
  - `supply_delta_gt` - supply delta amount **greater than**
  - `supply_delta_gte` - supply delta amount **greater than or equal to**

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
  map_accounts:params[params] --> map_accounts;
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> map_accounts;
  map_stat[map: map_stat];
  map_stat:params[params] --> map_stat;
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> map_stat;
  graph_out[map: graph_out];
  map_transfers --> graph_out;
  kv_out[map: kv_out];
  map_accounts --> kv_out;
  map_stat --> kv_out;
  db_out[map: db_out];
  map_transfers --> db_out;
```

### Modules

```yaml
Package name: eosio_token
Version: v0.13.2
Doc: Antelope `eosio.token` based action traces & database operations.
Modules:
----
Name: map_transfers
Initial block: 0
Kind: map
Output Type: proto:antelope.eosio.token.v1.TransferEvents
Hash: 85d87a1e5aea3ff7e545fed84a1498b90b75ab27

Name: map_accounts
Initial block: 0
Kind: map
Output Type: proto:antelope.eosio.token.v1.Accounts
Hash: cb3a616f37c81921ada58e42d7b1bd477a08c627

Name: map_stat
Initial block: 0
Kind: map
Output Type: proto:antelope.eosio.token.v1.Stats
Hash: 280bcef38dfea42c581220b9de015f48bf43891f

Name: graph_out
Initial block: 0
Kind: map
Output Type: proto:substreams.entity.v1.EntityChanges
Hash: a19a41ac282f80a0b650641135f7b7ba125f576a

Name: kv_out
Initial block: 0
Kind: map
Output Type: proto:sf.substreams.sink.kv.v1.KVOperations
Hash: 811775e8de26eb222af7a8f35865b22556988981

Name: db_out
Initial block: 0
Kind: map
Output Type: proto:sf.substreams.sink.database.v1.DatabaseChanges
Hash: 2c57375d5313b67ef83e0e0fc2d01ba9d45456ba
```
