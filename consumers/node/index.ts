import fs from 'fs';
import path from 'node:path';
import { credentials, Metadata } from '@grpc/grpc-js';
import { GrpcTransport } from '@protobuf-ts/grpc-transport';

// Substream generated code
// $ buf generate buf.build/fubhy/substreams
import { StreamClient } from './src/generated/sf/substreams/v1/substreams.client';
import { Package } from './src/generated/sf/substreams/v1/package';
import { ForkStep, Request, Response } from './src/generated/sf/substreams/v1/substreams';

// Export utils & Typescript interfaces
export * from "./src/generated/sf/substreams/v1/clock"
export * from "./src/generated/sf/substreams/v1/modules"
export * from "./src/generated/sf/substreams/v1/package"
export * from "./src/generated/sf/substreams/v1/substreams"
export * from "./utils";

// Envionrment Variables
import * as dotenv from "dotenv";
dotenv.config();
const PACKAGE = process.env.PACKAGE;
const MODULES = (process.env.MODULES || "").split(",");
const START_BLOCK_NUM = process.env.START_BLOCK_NUM;
const STOP_BLOCK_NUM = process.env.STOP_BLOCK_NUM;
const API_TOKEN = process.env.API_TOKEN;
const FIREHOSE_HOST = process.env.FIREHOSE_HOST || "eos.firehose.eosnation.io:9001";

if ( !START_BLOCK_NUM) throw new Error("Missing START_BLOCK_NUM environment variable");
if ( !PACKAGE) throw new Error("Missing PACKAGE environment variable");
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

// Load Substream
const file = path.isAbsolute(PACKAGE) ? PACKAGE : path.resolve(process.cwd(), PACKAGE);
const pkg = Package.fromBinary(fs.readFileSync(file));
const stream = client.blocks(Request.create({
    startBlockNum: START_BLOCK_NUM,
    stopBlockNum: STOP_BLOCK_NUM,
    forkSteps: [ForkStep.STEP_IRREVERSIBLE],
    modules: pkg.modules,
    outputModules: MODULES,
}));

export interface Adapter {
    init(startBlockNum?: string, stopBlockNum?: string): Promise<void> | void;
    processBlock(response: Response): Promise<void> | void;
    done(): Promise<void> | void;
}

// Parse Substream Block Data
export async function run(adapter: Adapter) {
    await adapter.init(START_BLOCK_NUM, STOP_BLOCK_NUM);
    for await (const response of stream.responses) {
        adapter.processBlock(response);
    }
    await adapter.done();
}
