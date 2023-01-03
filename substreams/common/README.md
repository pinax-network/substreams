# Antelope `Common` Substream

> Antelope **action traces** & **database operations**.

### Substream

| Name                | Version     | IPFS hash |
|---------------------|-------------|-----------|
| `common.spkg`       | **v0.1.0**  | `Qmdf7GT6jaT9NB3XPLvss8YxuHiSAC1PP1xm9UqLbuouYT`

### Map Modules

| Name                  | Description                    | Hash
|-----------------------|--------------------------------|---------|
| `map_action_traces`   | all action traces              | 143674a69c5c417d47bc0c520f4d94435695b577
| `map_db_ops`          | all database operations        | 6ed37c8532510ed6d3032b953c62146c9a7b1483

## Mermaid graph

```mermaid
graph TD;
  map_action_traces[map: map_action_traces]
  sf.antelope.type.v2.Block[source: sf.antelope.type.v2.Block] --> map_action_traces
  map_db_ops[map: map_db_ops]
  sf.antelope.type.v2.Block[source: sf.antelope.type.v2.Block] --> map_db_ops
```

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
    string account = 5;
    string receiver = 6;
    string name = 7;

    // action data
    string json_data = 8;
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
    string code = 5;               // contract name (ex: "eosio.token")
    string table_name = 6;         // table name (ex: "accounts")
    string scope = 7;              // scope name (ex: "EOS")
    string primary_key = 8;        // primary key (ex: "myaccount")

    // table data
    bytes old_data = 9;      // old data (bytes)
    bytes new_data = 10;      // new data (bytes)
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
#[path = "antelope.common.v1.rs"]
#[allow(dead_code)]
pub mod common;
```

### Build & Pack

```bash
$ cargo build --target wasm32-unknown-unknown --release
$ substreams pack ./substreams.yaml
```
