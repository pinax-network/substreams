import type { Argv } from 'yargs';
import fs from 'fs-extra';
import path from 'node:path';
import { credentials, Metadata } from '@grpc/grpc-js';
import { GrpcTransport } from '@protobuf-ts/grpc-transport';
import { StreamClient } from '../generated/sf/substreams/v1/substreams.client';
import { ForkStep, Request } from '../generated/sf/substreams/v1/substreams';
import { Package } from '../generated/sf/substreams/v1/package';
import { CommandArgs } from '..';
import * as tokens from '../tokens'

export const command = 'run <package>';
export const description = 'Run a substream';
export const builder = (yargs: Argv) =>
  yargs
    .positional('package', {
      demandOption: true,
      type: 'string',
      normalize: true,
    })
    .option('modules', {
      demandOption: true,
      type: 'string',
      array: true,
    })
    .option('api-token', {
      demandOption: false,
      type: 'string',
      default: '',
    })
    .option('endpoint', {
      demandOption: true,
      type: 'string',
      default: 'eos.firehose.eosnation.io:9001',
    })
    .option('start-block', {
      demandOption: true,
      type: 'string',
      default: '10000'
    })
    .option('stop-block', {
      type: 'string',
      default: '+1000'
    });

export async function handler(args: CommandArgs<typeof builder>) {
  const metadata = new Metadata();
  metadata.add('authorization', args.apiToken);

  const creds = credentials.combineChannelCredentials(
    credentials.createSsl(),
    credentials.createFromMetadataGenerator((_, callback) => callback(null, metadata)),
  );

  const client = new StreamClient(
    new GrpcTransport({
      host: args.endpoint,
      channelCredentials: creds,
    }),
  );

  const file = path.isAbsolute(args.package) ? args.package : path.resolve(process.cwd(), args.package);
  const pkg = Package.fromBinary(await fs.readFile(file));
  const stream = client.blocks(
    Request.create({
      startBlockNum: args.startBlock,
      stopBlockNum: args.stopBlock ? args.stopBlock : undefined,
      forkSteps: [ForkStep.STEP_IRREVERSIBLE],
      modules: pkg.modules,
      outputModules: args.modules,
    }),
  );

  for await (const response of stream.responses) {
    tokens.processBlock(response);
  }
  tokens.saveTokens();
  console.log("Done");
}
