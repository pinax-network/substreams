Block stats
===========

Substream to gather hourly accumulated stats from blockchains.

### Models

```protobuf
message HourlyStats {
  uint32 block_num = 1;                     // start block number of the accumulated interval
  google.protobuf.Timestamp timestamp = 2;  // block creation timestamp (UTC) of the start block
  int64 trx_count = 3;                      // number of successfully executed transactions in the interval
  int64 act_count = 4;                      // number of successfully executed actions in the interval
  string chain = 5;
}
```

### Quickstart

1. Go into `dev/blocktivity` and run `docker-compose up`
2. Check out the [postgres-sink](https://github.com/streamingfast/substreams-sink-postgres#setup) and then run 
`go install ./cmd/substreams-sink-postgres` from within that directory (requires a proper Go installation, see 
[here](https://github.com/EOS-Nation/substreams-antelope-core#go) for instructions)
3. Run the sink: `substreams-sink-postgres run "psql://app_user:password@127.0.0.1:5432/app_db?sslmode=disable" "eos.firehose.eosnation.io:9001" "substreams.yaml" db_out`
4. Open the Hasura console on `localhost:8080/console` and add the database under "Data" using this url: `postgresql://app_user:password@db:5432/app_db?sslmode=disable` and track the `hourly_stats` table