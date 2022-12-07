import * as fs from 'fs';
import { Token } from './generated/tokens'
import * as moment from 'moment';
import { BlockScopedData, Response } from './generated/sf/substreams/v1/substreams';

const csvFilename = `tokens.csv`;
let linesWritten = 0;
const tokens = new Map<string, Token>();

// only store_tokens module accepted
export function processBlock(response: Response) {
    if(response.message.oneofKind == 'progress') process.stdout.write(`.`)
    if(response.message.oneofKind !== 'data') return;
    const dataMess = (response.message as any).data as BlockScopedData;
    if(+(dataMess.clock?.number ?? "0") % 100 === 0){
        process.stdout.write(`${dataMess.clock?.number},`)
    }
    for(const output of dataMess.outputs){
        if(output.data.oneofKind !== 'storeDeltas') continue;
        for(const delta of output.data.storeDeltas.deltas){
            tokens.set(delta.key, Token.fromBinary(delta.newValue));
            console.log('\nFound:', delta.key)
        }
    }
}

function initStore() {
    fs.writeFileSync(csvFilename, `\
#,\
created_at,\
block_num,\
symcode,\
precision,\
contract,\
maximum_amount,\
issuer\n`);

    linesWritten++;
}

export function saveTokens() {
    for (const token of tokens.values()) {
        saveToken(token);
    }
}

function saveToken(token: Token) {
    if(linesWritten === 0) initStore();
    const date = moment.unix(+(token.timestamp?.seconds ?? '0'));
    fs.writeFileSync(csvFilename, `\
${linesWritten},\
${date.format('YYYY-MM-DD HH:mm:ss')},\
${token.blockNum},\
${token.symcode},\
${token.precision},\
${token.contract},\
${token.amount},\
${token.issuer}\n`, { flag:'a+' });

    linesWritten++;

}