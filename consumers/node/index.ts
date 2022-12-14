import fs from 'fs';
import path from 'node:path';
import protobuf from "protobufjs";
import { credentials, Metadata } from '@grpc/grpc-js';
import { GrpcTransport } from '@protobuf-ts/grpc-transport';

// Substream generated code
// $ buf generate buf.build/fubhy/substreams
import { StreamClient } from './src/generated/sf/substreams/v1/substreams.client';
import { Package } from './src/generated/sf/substreams/v1/package';
import { Modules } from './src/generated/sf/substreams/v1/modules';
import { BlockScopedData, ForkStep, Request } from './src/generated/sf/substreams/v1/substreams';

// Export utils & Typescript interfaces
export * from "./src/generated/sf/substreams/v1/clock"
export * from "./src/generated/sf/substreams/v1/modules"
export * from "./src/generated/sf/substreams/v1/package"
export * from "./src/generated/sf/substreams/v1/substreams"
export * from "./utils";

// Environment variables
import { API_TOKEN, FIREHOSE_HOST, START_BLOCK_NUM, STOP_BLOCK_NUM, MODULES, PACKAGE, PROTO } from './config';

// Utils
import { isIpfs, downloadToFile, getIpfsHash, parseBlockData } from './utils';

// Credentials
const metadata = new Metadata();
if ( API_TOKEN ) metadata.add('authorization', API_TOKEN);
const creds = credentials.combineChannelCredentials(
    credentials.createSsl(),
    credentials.createFromMetadataGenerator((_, callback) => callback(null, metadata)),
);

// Create Substream Client
const client = new StreamClient(
    new GrpcTransport({
        host: FIREHOSE_HOST,
        channelCredentials: creds,
    }),
);

export async function downloadPackage(ipfs: string) {
    // Download IPFS Substream package
    if ( isIpfs(ipfs) ) return downloadToFile(ipfs);

    // Read Substream from local filesystem
    console.log(`Reading Substream from file system: ${ipfs}`);
    const filepath = path.isAbsolute(ipfs) ? ipfs : path.resolve(process.cwd(), ipfs);
    if (!fs.existsSync(filepath)) throw new Error(`File not found: ${filepath}`);
    return new Uint8Array(fs.readFileSync(filepath));
}

// Load Substream
export function createStream(modules?: Modules) {
    return client.blocks(Request.create({
        startBlockNum: START_BLOCK_NUM,
        stopBlockNum: STOP_BLOCK_NUM,
        forkSteps: [ForkStep.STEP_IRREVERSIBLE],
        modules,
        outputModules: MODULES,
    }));
}

export type Adapter = {
    init(startBlockNum?: string, stopBlockNum?: string): any;
    processBlock(block: BlockScopedData): any;
    processMapOutput(value: Uint8Array): any;
    done(): any;
}

export type Config = {
    START_BLOCK_NUM?: string;
    STOP_BLOCK_NUM?: string;
}

// Parse Substream Block Data
export async function run(adapter: Adapter, config: Config = {}) {

    // Setup Substream
    const binary = await downloadPackage(PACKAGE);
    const ipfs = await getIpfsHash(binary);
    console.log(`Substream IPFS Hash: ${ipfs}`);
    const { modules } = Package.fromBinary(binary);
    const stream = createStream(modules);

    // Send Substream Data to Adapter
    await adapter.init(START_BLOCK_NUM, STOP_BLOCK_NUM);
    for await (const response of stream.responses) {
        const block = parseBlockData(response);
        if ( !block ) continue;
        adapter.processBlock(block);

        for ( const output of block.outputs ) {
            if ( output.data.oneofKind == "mapOutput" ) {
                const { value } = output.data.mapOutput;
                if ( !value.length ) continue;
                adapter.processMapOutput( value );
            }
        }
    }
    await adapter.done();
}
