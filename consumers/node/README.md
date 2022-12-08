# Node.js `Substream` Consumer

[![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/EOS-Nation/substreams-monorepo/blob/main/LICENSE)
[![Node.js Substreams](https://github.com/EOS-Nation/substreams-monorepo/actions/workflows/node-consumer.yml/badge.svg)](https://github.com/EOS-Nation/substreams-monorepo/actions/workflows/node-consumer.yml)

> General purpose SDK library

## Requirements

- [Node.js (LTS or Current)](https://nodejs.org/en/)
- [Buf - Protocol Buffers](https://buf.build/)
- [Antelope Firehose V2](https://eos.firehose.eosnation.io)
  > `eos.firehose.eosnation.io:9001` by default

## Environment Variables

```env
# Required
PACKAGE=QmYtDPjGUwQUAukhEKxeKqL9UeF1H9a2d1sNSMAQGDwVZe
MODULES=store_tokens
START_BLOCK_NUM=2
STOP_BLOCK_NUM=1001

# (Optional)
FIREHOSE_HOST=eos.firehose.eosnation.io:9001
API_TOKEN=""
```

## Quickstart

```js
import fs from "fs";
import * as substreams from "substreams-consumer";

// Initialize Process (write to JSONL file)
let writer;
function init(startBlock, stopBlock) {
    writer = fs.createWriteStream(`tokens_${startBlock}-${stopBlock}.jsonl`);
}

// Process Block Data
function processBlock(response) {
    const block = substreams.parseBlockData(response);
    if (!block) return;
    substreams.printBlock(block.clock);

    for ( const output of block.outputs ) {
        writer.write(JSON.stringify(output) + "\n");
    }
}

// Substream completed
function done() {
    writer.end();
}

// Run Substream process
(async () => {
    await substreams.run({init, processBlock, done});
    process.exit();
})();
```

## Tests

```bash
$ npm ci
$ npm test
```