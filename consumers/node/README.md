# Node.js `Substream` Consumer

[![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/EOS-Nation/substreams-monorepo/blob/main/LICENSE)
[![NodeJS Consumer](https://github.com/EOS-Nation/substreams-monorepo/actions/workflows/node-consumer.yml/badge.svg)](https://github.com/EOS-Nation/substreams-monorepo/actions/workflows/node-consumer.yml)

## Requirements

- [Node.js (LTS or Current)](https://nodejs.org/en/)
- [Buf - Protocol Buffers](https://buf.build/)
- [Antelope Firehose V2](https://eos.firehose.eosnation.io)
  > `eos.firehose.eosnation.io:9001` by default

## Environment Variables

```env
# Required
PACKAGE="../../substreams/tokens/tokens-v0.0.1.spkg"
MODULES=store_tokens
START_BLOCK_NUM=2
STOP_BLOCK_NUM=1000

# (Optional)
FIREHOSE_HOST=eos.firehose.eosnation.io:9001
API_TOKEN=""
```

## Quickstart

```js
import fs from "fs";
import substream, { printBlock, parseBlockData } from "substream-consumer";

// Initialize Process (write to JSONL file)
let writer: fs.WriteStream;
function init(startBlock, stopBlock) {
    console.log("Substream Initialized");
    writer = fs.createWriteStream(`tokens_${startBlock}-${stopBlock}.csv`);
}

// Process Block Data
function processBlock(response) {
    const block = parseBlockData(response);
    if (!block) return;
    printBlock(block.clock);

    for ( const output of block.outputs ) {
        writer.write(JSON.stringify(output) + "\n");
    }
}

// Substream completed
function done() {
    console.log("Substream Finished");
    writer.end();
}

// Run Substream process
(async () => {
    await substream({init, processBlock, done});
    process.exit();
})();
```

## Tests

```bash
$ npm ci
$ npm test
```