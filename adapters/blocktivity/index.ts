import fs from "node:fs";
import protobuf from "protobufjs";
import * as substreams from "../../consumers/node";

// Protobuf schema
const root = protobuf.loadSync("../../substreams/blocktivity/proto/block.proto");
const Stats = root.lookupType("Stats");

// Initialize Process (write to JSONL file)
let writer: fs.WriteStream;
function init(startBlock: string, stopBlock: string) {
    writer = fs.createWriteStream(`blocktivity_${startBlock}-${stopBlock}.jsonl`);
}

// Process Block Data
function processBlock( block: substreams.BlockScopedData ) {
    substreams.printBlock(block, 100);
}

// Process MapOutput
function processMapOutput( value: Uint8Array ) {
    const { stats } = Stats.decode(value).toJSON();
    for ( const row of stats ) {
        writer.write(JSON.stringify(row) + "\n");
    };
}

// Substream completed
function done() {
    writer.end();
    console.log("CTRL+C to exit");
}

// Run Substream process
(async () => {
    await substreams.run({init, processBlock, processMapOutput, done});
})();