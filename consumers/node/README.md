# Example Node.js Substream consumer

Streams Antelope substream from `/substreams/tokens/tokens-v0.0.1.spkg` package in the desired block range and dumps all found tokens into `tokens.csv` file

## Requirements

- node (v16+)
- ts-node
- buf (https://buf.build/)
- tokens-v0.0.1.spkg package generated from `/substreams/tokens` substream of this repository
- Antelope firehose v2 endpoint (using `eos.firehose.eosnation.io:9001` by default)

## Installation

Install dependecies:
```bash
npm i
```

## Codegen

Generate typescript bindings for protobufs:
```bash
npm run codegen
```

## Run

Run the client:
```bash
npm run start
```
or, with parameters:
```bash
ts-node ./src/index.ts run ../../substreams/tokens/tokens-v0.0.1.spkg --modules store_tokens --start-block 3200000 --stop-block 3210000
```
