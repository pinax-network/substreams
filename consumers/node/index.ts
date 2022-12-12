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
import { BlockScopedData, ForkStep, Request, Response } from './src/generated/sf/substreams/v1/substreams';

// Export utils & Typescript interfaces
export * from "./src/generated/sf/substreams/v1/clock"
export * from "./src/generated/sf/substreams/v1/modules"
export * from "./src/generated/sf/substreams/v1/package"
export * from "./src/generated/sf/substreams/v1/substreams"
export * from "./utils";

// Envionrment Variables
import * as dotenv from "dotenv";
import { download, getIpfsHash, isIpfs, parseBlockData } from './utils';
dotenv.config();
const PACKAGE = process.env.PACKAGE;
const PROTO = process.env.PROTO;
const MODULES = (process.env.MODULES || "").split(",");
const START_BLOCK_NUM = process.env.START_BLOCK_NUM;
const STOP_BLOCK_NUM = process.env.STOP_BLOCK_NUM;
const API_TOKEN = process.env.API_TOKEN;
const FIREHOSE_HOST = process.env.FIREHOSE_HOST || "eos.firehose.eosnation.io:9001";

if ( !START_BLOCK_NUM) throw new Error("Missing START_BLOCK_NUM environment variable");
if ( !MODULES) throw new Error("Missing MODULES environment variable");

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

export async function downloadPackage() {
    if ( !PACKAGE) throw new Error("Missing PACKAGE environment variable");

    // Download IPFS Substream package
    if ( isIpfs(PACKAGE) ) {
        const url = `https://eos.mypinata.cloud/ipfs/${PACKAGE}`
        console.log(`Downloading Substream from IPFS: ${url}`);
        return download(url);
    }
    // Read Substream from local filesystem
    console.log(`Reading Substream from file system: ${PACKAGE}`);
    const filepath = path.isAbsolute(PACKAGE) ? PACKAGE : path.resolve(process.cwd(), PACKAGE);
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

export interface Adapter {
    init(startBlockNum?: string, stopBlockNum?: string): any;
    processBlock(block: BlockScopedData): any;
    processMapOutput(value: Uint8Array): any;
    done(): any;
}

// Parse Substream Block Data
export async function run(adapter: Adapter) {

    // Setup Substream
    const binary = await downloadPackage();
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
