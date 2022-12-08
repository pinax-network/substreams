import { Clock } from "./src/generated/sf/substreams/v1/clock";
import { BlockScopedData, Response } from "./src/generated/sf/substreams/v1/substreams";

export function printBlock( clock?: Clock, interval = 100 ) {
    const seconds = getSeconds(clock);
    const date = formatDate(seconds);
    const block_num = Number(clock?.number);
    if ( block_num % interval !== 0) return;
    console.log(`----------- NEW BLOCK #${block_num} (${date}) ---------------`);
}

export function formatDate( seconds: number ) {
    return new Date(seconds * 1000).toISOString().replace(".000Z", "")
}

export function parseBlockData( response: Response) {
    if (response.message.oneofKind !== 'data') return;
    return (response.message as any).data as BlockScopedData;
}

export function getSeconds( clock?: Clock ) {
    return Number(clock?.timestamp?.seconds);
}