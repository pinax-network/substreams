# Antelope `Common` Substream

> Antelope **action traces** & **database operations**.

### [Latest Releases](https://github.com/pinax-network/substreams/releases)

### Quickstart

```bash
$ make
$ make run
```

### KV

- `block.number:<number>` -> `BlockTimestamp`
- `block.timestamp:<timestamp>` -> `BlockId`

**proto**
```protobuf
message BlockId {
  string id = 1;
  uint64 number = 2;
}

message BlockTimestamp {
  string timestamp = 1;
}
```

### Params

Params allow to filter messages and can be supplied to modules in the form of URL query, i.e. `contract=eosio.token&action=transfer` - filter `eosio.token` contract `transfer` actions.

| module       | key        | description |
|--------------|------------|-------------|
| `map_transaction_traces` | `contract` | filter by contract(s)
| `map_transaction_traces` | `action`   | filter by action(s)
| `map_transaction_traces` | `receiver`   | filter by receiver(s)
| `map_action_traces` | `contract` | filter by contract(s)
| `map_action_traces` | `action`   | filter by action(s)
| `map_action_traces` | `receiver`   | filter by receiver(s)
| `map_db_ops` | `contract` | filter by contract(s)
| `map_db_ops` | `table`   | filter by action(s)

## Mermaid graph

```mermaid
graph TD;
  map_block_header[map: map_block_header];                                                                                                                    
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> map_block_header;                                                                          
  map_blockroot_merkle[map: map_blockroot_merkle];
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> map_blockroot_merkle;
  map_transaction_traces[map: map_transaction_traces];
  map_transaction_traces:params[params] --> map_transaction_traces;
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> map_transaction_traces;
  map_action_traces[map: map_action_traces];
  map_action_traces:params[params] --> map_action_traces;
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> map_action_traces;
  map_db_ops[map: map_db_ops];
  map_db_ops:params[params] --> map_db_ops;                                  
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> map_db_ops;
  kv_out[map: kv_out];        
  sf.substreams.v1.Clock[source: sf.substreams.v1.Clock] --> kv_out;
```

### Modules

```yaml
Package name: common
Version: v0.6.0
Doc: Antelope based action traces & database operations.                                                                                                      
Modules:
----
Name: map_block_header
Initial block: 2
Kind: map
Output Type: proto:sf.antelope.type.v1.BlockHeader
Hash: 33951d63d323eb9a2bae9d09e9df8a7ebe84362b

Name: map_blockroot_merkle
Initial block: 2
Kind: map
Output Type: proto:sf.antelope.type.v1.BlockRootMerkle
Hash: 797ec4ed64818cbdd92ec75dabadd81ef3ad1dbb

Name: map_transaction_traces
Initial block: 2
Kind: map
Output Type: proto:sf.antelope.type.v1.TransactionTraces
Hash: f2541a11641027caec2d60de4b56f41c35837283

Name: map_action_traces
Initial block: 2
Kind: map
Output Type: proto:sf.antelope.type.v1.ActionTraces
Hash: b4c28e9904a2ea44ddf4eb891bc5a15040acc118

Name: map_db_ops
Initial block: 2
Kind: map
Output Type: proto:sf.antelope.type.v1.DBOps
Hash: c5acb9bb18e65fb6ab8d375053c1257bcb3b8a16

Name: kv_out
Initial block: 0
Kind: map
Output Type: proto:sf.substreams.sink.kv.v1.KVOperations
Hash: 9a2f97c2cf63229d2bec92b338007ad44baa87d0
```