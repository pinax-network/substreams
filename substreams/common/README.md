# Antelope `Common` Substream

> Antelope **action traces** & **database operations**.

## Mermaid graph

```mermaid
graph TD;
  map_block[map: map_block]
  sf.antelope.type.v2.Block[source: sf.antelope.type.v2.Block] --> map_block
  map_block_header[map: map_block_header]
  sf.antelope.type.v2.Block[source: sf.antelope.type.v2.Block] --> map_block_header
  map_blockroot_merkle[map: map_blockroot_merkle]
  sf.antelope.type.v2.Block[source: sf.antelope.type.v2.Block] --> map_blockroot_merkle
  map_transaction_traces[map: map_transaction_traces]
  sf.antelope.type.v2.Block[source: sf.antelope.type.v2.Block] --> map_transaction_traces
  map_action_traces[map: map_action_traces]
  sf.antelope.type.v2.Block[source: sf.antelope.type.v2.Block] --> map_action_traces
  map_db_ops[map: map_db_ops]
  sf.antelope.type.v2.Block[source: sf.antelope.type.v2.Block] --> map_db_ops
  store_timestamp[store: store_timestamp]
  sf.antelope.type.v2.Block[source: sf.antelope.type.v2.Block] --> store_timestamp
```

### Substream

| Name                 | IPFS hash |
|----------------------|-----------|
| `common-v0.3.0.spkg` | `QmdUPvbKoccXiZSHgsC5mpZMXQt6tuMsoYYf3foke2X5uV`
| `common-v0.2.0.spkg` | `QmfE7kdRAPihhvij4ej3rUM2Sp3PcXQ9rTFCQPhPGB5dr5`

### Modules

```yaml
Name: map_block
Initial block: 2
Kind: map
Output Type: proto:sf.antelope.type.v1.Block
Hash: 7fa2f6dfcc529c5c6b5475c45dadcce440237d1a

Name: map_block_header
Initial block: 2
Kind: map
Output Type: proto:sf.antelope.type.v1.BlockHeader
Hash: 8c5632a9f90e83154f64483515d422c48ef03633

Name: map_blockroot_merkle
Initial block: 2
Kind: map
Output Type: proto:sf.antelope.type.v1.BlockRootMerkle
Hash: 616305d3627edb11b97b8c7573fa23a487976d54

Name: map_transaction_traces
Initial block: 2
Kind: map
Output Type: proto:sf.antelope.type.v1.TransactionTraces
Hash: d49b88f6a4d44408b748f0490b43250640000762

Name: map_action_traces
Initial block: 2
Kind: map
Output Type: proto:sf.antelope.type.v1.ActionTraces
Hash: 179fab7cdd8f362130524d5169bb95b0e481cd1b

Name: map_db_ops
Initial block: 2
Kind: map
Output Type: proto:sf.antelope.type.v1.DBOps
Hash: 0fec3d3a2a72032e94c1b2d1372f37dd45931e31

Name: store_timestamp
Initial block: 2
Kind: store
Value Type: int64
Update Policy: UPDATE_POLICY_SET
Hash: 14828f8b73f412d36aeafef255c62e29d1135f2c
```