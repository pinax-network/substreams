import path from "path";
import fs from "fs";
import os from "os";
import { importer } from 'ipfs-unixfs-importer';
import { MemoryBlockstore } from 'blockstore-core/memory';

// Substream auto-generated
import { Clock } from "./src/generated/sf/substreams/v1/clock";
import { BlockScopedData, Response } from "./src/generated/sf/substreams/v1/substreams";

export function printBlock( block: BlockScopedData, interval = 100 ) {
    const seconds = getSeconds(block.clock);
    const date = formatDate(seconds);
    const block_num = Number(block.clock?.number);
    if ( block_num % interval !== 0) return;
    console.log(`----------- NEW BLOCK #${block_num} (${date}) ---------------`);
}

export function formatDate( seconds: number ) {
    return new Date(seconds * 1000).toISOString().replace(".000Z", "")
}

export function parseBlockData( response: Response ) {
    if (response.message.oneofKind !== 'data') return;
    return (response.message as any).data as BlockScopedData;
}

export function getSeconds( clock?: Clock ) {
    return Number(clock?.timestamp?.seconds);
}

export const isIpfs = ( str: string ) => /^Qm[1-9A-Za-z]{44}$/.test(str);

export async function downloadToFile(ipfs: string) {
    const filepath = path.join(os.tmpdir(), ipfs);
    if (fs.existsSync(filepath)) {
        console.log(`Already exists: ${ipfs}`);
        const buffer = fs.readFileSync(filepath);
        return new Uint8Array(buffer);
    }
    const data = await download(ipfs);
    fs.writeFileSync(filepath, data);
    return data;
}

export async function download(ipfs: string) {
    console.log(`Downloading: ${ipfs}`);
    const url = `https://eos.mypinata.cloud/ipfs/${ipfs}`
    const response = await fetch(url);
    if (!response.ok) throw new Error(`unexpected response ${response.statusText}`)
    console.log(`Download completed: ${ipfs}`);

    const blob = await response.blob();
    const buffer = await blob.arrayBuffer();
    return new Uint8Array(buffer);
}

const blockstore = new MemoryBlockstore();

export async function getIpfsHash(binary: Uint8Array) {
    const content = Buffer.from(binary);
    for await (const { cid } of importer([{content}], blockstore, {onlyHash: true})) {
        return cid.toString();
    }
    throw new Error("Failed to read IPFS hash from ReadStream content");
}