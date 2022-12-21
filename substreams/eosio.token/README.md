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