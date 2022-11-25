Antelope blockmeta by timestamp
===============================

Exemplary Substream that extracts meta information from Antelope blocks and stores them into a timestamp keyed KV store.
This allows getting block information (such as the block number or id) as well as some stats (like the number of
transactions) for any timestamp.

### Models

BlockMeta protobuf:

```protobuf
message BlockMeta {
  uint32 block_num = 1;
  string block_id = 2;
  google.protobuf.Timestamp timestamp = 3;
  uint32 trx_count = 10;
}
```

### Maps

`map_blockmeta` - reads in a full antelope block and transforms it into a BlockMeta object.

### Stores

`store_blockmeta` - reads in the BlockMeta object and stores it into a KV store where the key is the BlockMeta's
timestamp
and the value the full BlockMeta object.

### Running the Substream

```
substreams run -e waxtest.firehose.eosnation.io:9001 substreams.yaml store_blockmeta
```
