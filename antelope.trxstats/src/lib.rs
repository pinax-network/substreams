use substreams::errors::Error as SubstreamsError;
use substreams_antelope::pb::Block;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables as DatabaseChangeTables;

#[path = "pb/antelope.trxstats.v1.rs"]
pub mod antelope_trxstats;
pub use self::antelope_trxstats::*;

#[derive(Debug, Default)]
struct Params {
    cpu_elapsed: u32,
    net_elapsed: u32,
    cpu_usage: u32,
    net_usage: u32,
    action_count: u32,
}

fn parse_params(input: String) -> Result<Params, String> {
    let mut result = Params::default();
    if input.is_empty() {
        return Ok(result);
    }

    for param in input.split('&') {
        let (key, value) = param
            .split_once('=')
            .ok_or_else(|| format!("Invalid parameter format: {}", param))?;
        let parsed_value = value.parse::<u32>().map_err(|_| format!("Invalid param value for key '{}'", key))?;

        match key {
            "cpu_elapsed" => result.cpu_elapsed = parsed_value,
            "net_elapsed" => result.net_elapsed = parsed_value,
            "cpu_usage" => result.cpu_usage = parsed_value,
            "net_usage" => result.net_usage = parsed_value,
            "action_count" => result.action_count = parsed_value,
            _ => return Err(format!("Unknown parameter: '{}'", key)),
        }
    }

    Ok(result)
}

#[substreams::handlers::map]
fn map_trxs(params: String, block: Block) -> Result<Transactions, SubstreamsError> {
    let params = match parse_params(params) {
        Ok(params) => params,
        Err(err) => panic!("Bad params: {}", err),
    };
    Ok(Transactions {
        transactions: block
            .executed_transaction_traces()
            .filter_map(|trx| match trx.index {
                0 => None, // skip eosio::onblock trxs
                _ => Some(Transaction {
                    trx_id: trx.id.clone(),
                    block_num: trx.block_num,
                    block_time: trx.block_time.clone(),
                    action_count: trx.action_traces.len() as u32,
                    cpu_elapsed_us: trx.elapsed as u32,
                    net_elapsed_bytes: trx.net_usage as u32,
                    cpu_usage_us: trx.receipt.clone().unwrap_or_default().cpu_usage_micro_seconds,
                    net_usage_bytes: trx.receipt.clone().unwrap_or_default().net_usage_words * 8,
                    status: trx.receipt.clone().unwrap_or_default().status,
                }),
            })
            .filter(|trx| {
                trx.cpu_elapsed_us >= params.cpu_elapsed
                    && trx.net_usage_bytes >= params.net_elapsed
                    && trx.cpu_usage_us >= params.cpu_usage
                    && trx.net_usage_bytes >= params.net_usage
                    && trx.action_count >= params.action_count
            })
            .collect(),
    })
}

#[substreams::handlers::map]
fn db_out(trxs: Transactions) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut tables = DatabaseChangeTables::new();
    trxs.transactions.into_iter().enumerate().for_each(|(i, trx)| {
        tables
            .create_row("transactions", format!("{}-{}", trx.block_num, i))
            .set("trx_id", trx.trx_id)
            .set("block_time", trx.block_time.unwrap())
            .set("block_num", trx.block_num)
            .set("action_count", trx.action_count)
            .set("cpu_elapsed_us", trx.cpu_elapsed_us)
            .set("net_elapsed_bytes", trx.net_elapsed_bytes)
            .set("cpu_usage_us", trx.cpu_usage_us)
            .set("net_usage_bytes", trx.net_usage_bytes)
            .set("status", trx.status);
    });

    Ok(tables.to_database_changes())
}
