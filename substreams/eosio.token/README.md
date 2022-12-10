# Antelope `eosio.token` Substream

| Name                | Version | IPFS hash |
|---------------------|---------|-----------|
| `eosio.token.proto` | v0.0.1  | <IPFS>
| `eosio.token.spkg`  | v0.0.1  | <IPFS>

### Maps

| type | Name              | Description
|------|-------------------|------------------|
| map  | `map_eosio_token` | reads in a full antelope block and transforms it into a Tokens object.

### Quickstart

```
substreams run -e eos.firehose.eosnation.io:9001 substreams.yaml store_tokens
```