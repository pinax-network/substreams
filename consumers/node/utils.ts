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

export const isIPFS = ( str: string ) => /^Qm[1-9A-Za-z]{44}$/.test(str);

export async function download(url: string) {
    const response = await fetch(url);
    if (!response.ok) throw new Error(`unexpected response ${response.statusText}`)

    const blob = await response.blob();
    const buffer = await blob.arrayBuffer();
    return new Uint8Array(buffer);
}