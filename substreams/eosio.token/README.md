# Antelope `eosio.token` Substream

> Antelope `eosio.token` based **action traces** & **database operations**.

### Substream

| Name                | Version     | IPFS hash |
|---------------------|-------------|-----------|
| `eosio-token.spkg`  | **v0.2.0**  | `QmahVkBjZcQQREtK7KyheioErp47uwojWm3jAW3Wq69AEq`

### Map Modules

| Name                  | Description
|-----------------------|--------------------------------------|
| `map_action_traces`   | all `eosio.token` action traces
| `map_db_ops`          | all `eosio.token` database operations

### Protobuf

```proto
syntax = "proto3";

package antelope.common.v1;

import "google/protobuf/timestamp.proto";

message ActionTraces {
  repeated ActionTrace action_traces = 1;
}

message ActionTrace {
    // trace information
    uint32 block_num = 1;
    google.protobuf.Timestamp timestamp = 2;
    string trx_id = 3;
    uint32 action_ordinal = 4;

    // action
    string account = 11;
    string receiver = 12;
    string name = 13;

    // action data
    string json_data = 20;
}

message DatabaseOperations {
  repeated DatabaseOperation db_ops = 1;
}

message DatabaseOperation {
  // trace information
  uint32 block_num = 1;
  google.protobuf.Timestamp timestamp = 2;
  string trx_id = 3;
  uint32 action_index = 4;

  // database operation
  string code = 10;               // contract name (ex: "eosio.token")
  string table_name = 11;         // table name (ex: "accounts")
  string scope = 12;              // scope name (ex: "EOS")
  string primary_key = 13;        // primary key (ex: "myaccount")

  // table data
  bytes old_data = 20;      // old data (bytes)
  bytes new_data = 21;      // new data (bytes)
}
```

### Quickstart

```
$ substreams run -e eos.firehose.eosnation.io:9001 substreams.yaml map_action_traces -s 284958698
```

### Build Protobuf

Generate protobuf code

```
$ substreams protogen ./substreams.yaml --exclude-paths="sf/antelope,sf/substreams,google"
```

To include **/src/pb/mod.rs**

```rs
#[path = "antelope.eosio.token.v1.rs"]
#[allow(dead_code)]
pub mod eosio_token;
```

### Build & Pack

```bash
$ cargo build --target wasm32-unknown-unknown --release
$ substreams pack ./substreams.yaml
```
