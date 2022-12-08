import fs from "fs";
import * as substreams from "../../consumers/node/index.js";

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