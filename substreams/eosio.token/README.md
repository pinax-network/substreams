# Antelope `eosio.token` Substream

> Antelope `eosio.token` related action events

### Substream

| Name                | Version     | IPFS hash |
|---------------------|-------------|-----------|
| `eosio-token.spkg`  | **v0.1.0**  | `QmXhHkjuqCFvxEaYDrcURZMhD7y9RNSfNWmXHtX8ramEHL`
| `eosio-token.spkg`  | **v0.1.1**  | `QmSPj4yghnFuosBeyZ2Cc5EY3agSP4Qg2g6v1Rh4tnWSgx`

### Map Modules

| Name                         | Description
|------------------------------|-----------------------|
| `map_actions`                | all `eosio.token` actions 
| `map_transfers`              | all `eosio.token` **transfer** actions 

### Store Modules

| Policy | Value | Name                     | Description
|--------|-------|--------------------------|-----------------------|
| add    | int64 | `store_transfers_amount` | `eosio.token` transfer quantity
| add    | int64 | `store_transfers_count`  | `eosio.token` transfer counts

### Store Keys

- `<account>-<symcode>`
- `<account>-<symcode>-from-<from>`
- `<account>-<symcode>-to-<to>`

### Protobuf

```proto
message Actions {
  repeated Action actions = 1;
}

message Action {
  uint32 block_num = 1;
  google.protobuf.Timestamp timestamp = 2;
  string transaction_id = 3;
  string account = 4;
  string name = 5;
  string json_data = 6;
}
```

### Quickstart

```
$ substreams run -e eos.firehose.eosnation.io:9001 substreams.yaml map_actions -s 284958698
```

### Build

To include **/src/pb/mod.rs**

```rs
#[path = "antelope.eosio.token.v1.rs"]
#[allow(dead_code)]
pub mod eosio_token;
```

```bash
$ substreams protogen ./substreams.yaml --exclude-paths="sf/antelope,sf/substreams,google"
$ cargo build --target wasm32-unknown-unknown --release
$ substreams pack ./substreams.yaml
```
