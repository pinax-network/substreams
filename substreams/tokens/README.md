# Antelope `Tokens` Substream

> Extracts token supply from Antelope `eosio.token` based tokens.

| Version | IPFS hash |
|---------|-----------|
| v0.1.0  | QmXhHkjuqCFvxEaYDrcURZMhD7y9RNSfNWmXHtX8ramEHL

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
  int64 amount = 6;
  string issuer = 7;
}
```

### Maps

`map_tokens` - reads in a full antelope block and transforms it into a Tokens object.

### Stores

`store_tokens` - reads in the Tokens object and stores it into a KV store where the key is the token as `<symcode>,<precision>@<contract>`, i.e. `WAX,8@eosio.token` and the value the full Token object.

### Running the Substream

```
substreams run -e eos.firehose.eosnation.io:9001 substreams.yaml store_tokens
```
