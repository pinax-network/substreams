import fs from "node:fs";
import protobuf from "protobufjs";
import * as substreams from "../../consumers/node";

// Protobuf schema
const root = protobuf.loadSync("../../substreams/eosio.token/proto/actions.proto");
const Actions = root.lookupType("Actions");

// Initialize Process (write to JSONL file)
let writer: fs.WriteStream;
function init(startBlock: string, stopBlock: string) {
    writer = fs.createWriteStream(`eosio.token_${startBlock}-${stopBlock}.jsonl`);
}

// Process Block Data
function processBlock( block: substreams.BlockScopedData ) {
    substreams.printBlock(block);
}

// Process MapOutput
function processMapOutput( value: Uint8Array ) {
    const { actions } = Actions.decode(value).toJSON();
    for ( const action of actions ) {
        if ( action.jsonData ) action["jsonData"] = JSON.parse(action.jsonData);
        writer.write(JSON.stringify(action) + "\n");
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