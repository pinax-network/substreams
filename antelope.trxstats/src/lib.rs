use substreams::errors::Error;
use substreams_antelope::Block;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables as DatabaseChangeTables;

#[path = "pb/antelope.trxstats.v1.rs"]
pub mod antelope_trxstats;
pub use self::antelope_trxstats::*;

#[substreams::handlers::map]
fn map_trxs(block: Block) -> Result<Transactions, Error> {
    Ok(Transactions {
        transactions: block
            .all_transaction_traces()
            .map(|trx| Transaction {
                trx_id: trx.id.clone(),
                block_num: trx.block_num,
                block_time: trx.block_time.clone(),
                action_count: trx.action_traces.len() as u32,
                cpu: trx.elapsed as i32,
                net: trx.net_usage as u32,
            })
            .collect(),
    })
}

#[substreams::handlers::map]
fn db_out(trxs: Transactions) -> Result<DatabaseChanges, substreams::errors::Error> {
    // Initialize changes container
    let mut tables = DatabaseChangeTables::new();
    trxs.transactions.into_iter().for_each(|trx| {
        tables
            .create_row("transactions", &trx.trx_id)
            .set("block_time", trx.block_time.unwrap())
            .set("block_num", trx.block_num)
            .set("action_count", trx.action_count)
            .set("cpu", trx.cpu)
            .set("net", trx.net);
    });

    Ok(tables.to_database_changes())
}
