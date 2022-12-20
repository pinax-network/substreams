# Antelope `eosio.token` Substream

> Antelope `eosio.token` related action events

### Substream

| Name                | Version     | IPFS hash |
|---------------------|-------------|-----------|
| `eosio.token.spkg`  | **v0.1.0**  | `QmXhHkjuqCFvxEaYDrcURZMhD7y9RNSfNWmXHtX8ramEHL`

### Modules

| type | Name                         | Description
|------|------------------------------|-----------------------|
| map  | `map_actions`                | `eosio.token` based actions 
| map  | `map_transfers`              | `eosio.token` based **transfer** actions 
| map  | `map_transfers_eosio_token`  | `eosio.token` **transfer** actions 
| map  | `map_transfers_accounts`     | `eosio.token` based **transfer** actions from accounts

### Protobuf

- **package:** `antelope.eosio.token.v1`

| Name                | Version     | IPFS hash |
|---------------------|-------------|-----------|
| `actions.proto`     | **v0.1.0**  | `QmWthaEr1Zde3g7CdoWpPqL4fCvptHZHFq4evBNoWppotP`
| `image.bin`         | **v0.1.0**  | `QmQ5Ym23TdDBKdPhhszFxhVLf3ujrhj2L6zU3twuGAk7yc`
| `image.json`        | **v0.1.0**  | `QmagbLrPqgr1a9ZeZv9Ur52T2Q2D5qxGa6RxMH9cRg3rdo`


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
substreams run -e eos.firehose.eosnation.io:9001 substreams.yaml map_actions
```