Antelope blocktivity stats.
===========================

Substream to gather blocktivity stats on Antelope chains. For each hour this substream accumulates the count of successful
transactions and actions, as well as accuulated cpu and net usage stats. 

### Models

```protobuf
message Blocktivity {
  uint32 block_num = 1;                     // block number
  google.protobuf.Timestamp timestamp = 2;  // block creation timestamp (UTC)
  uint32 trx_count = 3;                     // number of successfully executed transactions in this block
  uint32 act_count = 4;                     // number of successfully executed actions in this block
  uint32 cpu_usage_us = 5;                  // cpu_usage_us of this block
  uint32 net_usage_words = 6;               // net_usage_words of this block
}
```

### Maps

`map_blocktivity` - Transforms a full block into a `Blocktivity` object

### Stores

Each store will get the `Blocktivity` objects, calculate the key (the timestamp of the hour of this block) and then 
accumulates the data in multiple `StoreAddInt64` stores.

`store_trx_count` - Adds up the number of transactions per key (hour)
`store_act_count` - Adds up the number of actions per key (hour)
`store_cpu_usage_us` - Adds up the cpu_usage_us values per key (hour)
`store_net_usage_words` - Adds up the net_usage_words per key (hour)
