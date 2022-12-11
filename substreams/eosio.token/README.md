# Antelope `eosio.token` Substream

> Antelope `eosio.token` related action events

| Name                | Version     | IPFS hash |
|---------------------|-------------|-----------|
| `eosio.token.proto` | **v0.1.0**  | `QmZTXiX2NyMjWyR6d58jJAbW2ZCg2awiCjZRbb6JyustxX`
| `eosio.token.spkg`  | **v0.1.0**  | `QmbN4xycZ2omL68cuzNp2Civ1QTRR6mJ7P7kBQuACsEp1H`

### Maps

| type | Name          | Description
|------|---------------|------------------|
| map  | `map_actions` | `eosio.token` action events

### Quickstart

```
substreams run -e eos.firehose.eosnation.io:9001 substreams.yaml map_actions
```