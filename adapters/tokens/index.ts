import fs from "fs";
import * as substreams from "../../consumers/node";
import { BlockScopedData } from "../../consumers/node";

// Initialize Process (write to JSONL file)
let writer: fs.WriteStream;
function init(startBlock: string, stopBlock: string) {
    writer = fs.createWriteStream(`tokens_${startBlock}-${stopBlock}.jsonl`);
}

// Process Block Data
function processBlock(block: BlockScopedData) {
    substreams.printBlock(block);
    for ( const output of block.outputs ) {
        writer.write(JSON.stringify(output) + "\n");
    }
}

// Substream completed
function done() {
    writer.end();
    console.log("CTRL+C to exit");
}

// Run Substream process
(async () => {
    await substreams.run({init, processBlock, done});
})();