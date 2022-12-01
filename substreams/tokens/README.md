Antelope tokens substream
===============================

Exemplary Substream that extracts tokens from Antelope blocks and stores them into a KV store.

### Models

Tokens protobuf:

```protobuf
message Tokens {
  repeated Token tokens = 1;
}

message Token {
  uint32 block_num = 1;
  google.protobuf.Timestamp timestamp = 2;
  string contract = 3;
  string symcode = 4;
  uint32 precision = 5;
}
```

### Maps

`map_tokens` - reads in a full antelope block and transforms it into a Tokens object.

### Stores

`store_tokens` - reads in the Tokens object and stores it into a KV store where the key is the token as `<symcode>,<precision>@<contract>`, i.e. `WAX,8@eosio.token` and the value the full Token object.

### Running the Substream

```
substreams run -e waxtest.firehose.eosnation.io:9001 substreams.yaml store_tokens
```
