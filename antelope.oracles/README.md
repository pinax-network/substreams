# Antelope Oracles Substream

> Antelope Oracles prices from [`delphioracle`](https://bloks.io/account/delphioracle) and [`oracle.defi`](https://bloks.io/account/oracle.defi)

### [Latest Releases](https://github.com/pinax-network/substreams/releases)

### Quickstart

```bash
$ make delphioracle # quotes
$ make oracle.defi # prices
```

**delphioracle**
```proto
message Datapoint {
  uint64 id = 1;
  uint64 median = 2;
  string owner = 3;
  string timestamp = 4;
  uint64 value = 5;
}

message Quote {
  string pair = 1;
  Datapoint value = 2;
}
```

**oracle.defi**
```proto
message Price {
    uint64 id = 1;
    string contract = 2;
    string coin = 3;
    uint32 precision = 4;
    uint64 acc_price = 5;
    uint64 last_price = 6;
    uint64 avg_price = 7;
    string last_update = 8;
}
```

### Mermaid graph

```mermaid
graph TD;
  map_prices[map: map_prices];
  map_prices:params[params] --> map_prices;
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> map_prices;
  map_quotes[map: map_quotes];
  map_quotes:params[params] --> map_quotes;
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> map_quotes;
  kv_out[map: kv_out];
  map_quotes --> kv_out;
  sf.substreams.v1.Clock[source: sf.substreams.v1.Clock] --> kv_out;
  db_out[map: db_out];
  map_quotes --> db_out;
```

### Modules

```yaml
Package name: antelope_oracles
Version: v0.0.6
Doc: Antelope `eosio.token` based action traces & database operations.
Modules:
----
Name: map_prices
Initial block: 0
Kind: map
Output Type: proto:antelope.oracles.v1.Prices
Hash: d5ec501aa597007bb1b63f729263f3a152b749ac

Name: map_quotes
Initial block: 0
Kind: map
Output Type: proto:antelope.oracles.v1.Quotes
Hash: 76429076938ef322d2bc68412e9137eb40f4ef53

Name: kv_out
Initial block: 0
Kind: map
Output Type: proto:sf.substreams.sink.kv.v1.KVOperations
Hash: 2e4bf0e6984be6caa57d81f6afe826f8027d79eb

Name: db_out
Initial block: 0
Kind: map
Output Type: proto:sf.substreams.sink.database.v1.DatabaseChanges
Hash: 72a43f091ecbccc84af59cfc66b512c11779a3d3
```