import fs from 'fs';
import path from 'node:path';
import { credentials, Metadata } from '@grpc/grpc-js';
import { GrpcTransport } from '@protobuf-ts/grpc-transport';

// Substream generated code
// $ buf generate buf.build/fubhy/substreams
import { StreamClient } from './client/sf/substreams/v1/substreams.client';
import { Package } from './client/sf/substreams/v1/package';
import { ForkStep, Request, Response } from './client/sf/substreams/v1/substreams';

// Export utils & Typescript interfaces
export * from "./client/sf/substreams/v1/clock"
export * from "./client/sf/substreams/v1/modules"
export * from "./client/sf/substreams/v1/package"
export * from "./client/sf/substreams/v1/substreams"
export * from "./utils";

// Envionrment Variables
const PACKAGE = "../../substreams/tokens/tokens-v0.0.1.spkg"
const MODULES = "store_tokens".split(",");
const START_BLOCK_NUM = "2";
const STOP_BLOCK_NUM = "1000";
const API_TOKEN = "";
const FIREHOSE_HOST = "eos.firehose.eosnation.io:9001";

// Credentials
const metadata = new Metadata();
metadata.add('authorization', API_TOKEN);
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

interface Adapter {
    init(startBlockNum: string, stopBlockNum: string): Promise<void> | void;
    processBlock(response: Response): Promise<void> | void;
    done(): Promise<void> | void;
}

// Parse Substream Block Data
export default async (adapter: Adapter) => {
    await adapter.init(START_BLOCK_NUM, STOP_BLOCK_NUM);
    for await (const response of stream.responses) {
        adapter.processBlock(response);
    }
    await adapter.done();
}
