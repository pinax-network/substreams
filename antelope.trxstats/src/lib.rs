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
            .filter_map(|trx| match trx.index {
                0 => None, // skip eosio::onblock trxs
                _ => Some(Transaction {
                    trx_id: trx.id.clone(),
                    block_num: trx.block_num,
                    block_time: trx.block_time.clone(),
                    action_count: trx.action_traces.len() as u32,
                    elapsed: trx.elapsed as i32,
                    net_usage: trx.net_usage as u32,
                    cpu_usage_micro_seconds: trx.receipt.clone().unwrap_or_default().cpu_usage_micro_seconds,
                    net_usage_words: trx.receipt.clone().unwrap_or_default().net_usage_words,
                    status: trx.receipt.clone().unwrap_or_default().status,
                }),
            })
            .collect(),
    })
}

#[substreams::handlers::map]
fn db_out(trxs: Transactions) -> Result<DatabaseChanges, substreams::errors::Error> {
    // Initialize changes container
    let mut tables = DatabaseChangeTables::new();
    trxs.transactions.into_iter().enumerate().for_each(|(i, trx)| {
        tables
            .create_row("transactions", format!("{}-{}", trx.block_num, i))
            .set("trx_id", trx.trx_id)
            .set("block_time", trx.block_time.unwrap())
            .set("block_num", trx.block_num)
            .set("action_count", trx.action_count)
            .set("elapsed", trx.elapsed)
            .set("net_usage", trx.net_usage)
            .set("cpu_usage_micro_seconds", trx.cpu_usage_micro_seconds)
            .set("net_usage_words", trx.net_usage_words)
            .set("status", trx.status);
    });

    Ok(tables.to_database_changes())
}
