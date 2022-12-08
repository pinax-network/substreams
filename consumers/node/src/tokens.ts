import * as fs from 'fs';
import { Token } from './generated/tokens'
import { Clock } from './generated/sf/substreams/v1/clock';
import { Response, StoreDelta } from './generated/sf/substreams/v1/substreams';
import { formatDate, getSeconds, parseBlockData, printBlock } from './utils';

// Initialize Process (write to CSV file)
let writer: fs.WriteStream;
export function init(startBlock: number|string, stopBlock: number|string) {
    writer = fs.createWriteStream(`tokens_${startBlock}-${stopBlock}.csv`);
    writer.write([
        'created_at',
        'timestamp',
        'block_num',
        'symcode',
        'precision',
        'contract',
        'maximum_amount',
        'issuer',
    ].join(",") + "\n");
}

function write(clock?: Clock, delta?: StoreDelta) {
    if (!delta) return;
    const timestamp = getSeconds(clock);
    const created_at = formatDate(timestamp);
    const value = Token.fromBinary(delta.newValue);

    writer.write([
        created_at,
        timestamp,
        value.blockNum,
        value.symcode,
        value.precision,
        value.contract,
        value.amount,
        value.issuer,
    ].join(",") + "\n");
}

// only store_tokens module accepted
export function processBlock(response: Response) {
    const block = parseBlockData(response);
    if (!block) return;
    printBlock(block.clock);

    for ( const output of block.outputs ) {
        if ( output.data.oneofKind !== 'storeDeltas' ) continue;
        for (const delta of output.data.storeDeltas.deltas) {
            write(block.clock, delta);
        }
    }
}

export function done() {
    writer.end();
    process.exit();
}
