import { Clock } from "./generated/sf/substreams/v1/clock";
import { BlockScopedData, Response } from "./generated/sf/substreams/v1/substreams";

export function printBlock( clock?: Clock ) {
    const {number: blockNum, timestamp} = clock!;
    const seconds = getSeconds(clock);
    const date = formatDate(seconds);
    console.log(`----------- NEW BLOCK #${blockNum} (${date}) ---------------`);
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