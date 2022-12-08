import fs from "fs";
import substream, { printBlock, parseBlockData } from "substream-consumer";

// Initialize Process (write to JSONL file)
let writer;
function init(startBlock, stopBlock) {
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
    writer.end();
}

// Run Substream process
(async () => {
    await substream({init, processBlock, done});
    process.exit();
})();