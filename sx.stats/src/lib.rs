use pb::antelope::sx::stats::v1::{Asset, Logs, TradeLog};
use std::fmt;
use std::str::FromStr;
use substreams::scalar::BigDecimal;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables as DatabaseChangeTables;

mod abi;
mod pb;

const ACCOUNT: &'static str = "stats.sx";

#[substreams::handlers::map]
fn map_logs(block: substreams_antelope::pb::Block) -> Result<Logs, substreams::errors::Error> {
    Ok(Logs {
        tradelogs: block
            .actions::<abi::contract::actions::Tradelog>(&[ACCOUNT])
            .map(|(action, trx)| TradeLog {
                trx_id: trx.transaction_id.clone(),
                trx_index: trx.action_ordinal,
                block_num: trx.block_num,
                block_time: trx.block_time.clone(),
                producer: block.header.as_ref().unwrap().producer.clone(),
                cpu_usage: block
                    .executed_transaction_traces()
                    .find(|tx| tx.id == trx.transaction_id)
                    .and_then(|tx| Some(tx.receipt.as_ref().unwrap().cpu_usage_micro_seconds))
                    .unwrap(),
                contract: action.contract,
                executor: action.executor,
                borrow: action.borrow.parse::<Asset>().ok(),
                profit: action.profit.parse::<Asset>().ok(),
                quantities: action
                    .quantities
                    .iter()
                    .map(|quantity| quantity.parse::<Asset>().expect("invalid asset"))
                    .collect(),
                codes: action.codes,
            })
            .collect(),
    })
}

#[substreams::handlers::map]
fn db_out(logs: Logs) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut tables = DatabaseChangeTables::new();
    logs.tradelogs.into_iter().enumerate().for_each(|(i, trade)| {
        tables
            .create_row("trades", format!("{}-{}", trade.block_num, i))
            .set("trx_id", trade.trx_id)
            .set("block_time", trade.block_time.unwrap())
            .set("block_num", trade.block_num)
            .set("producer", trade.producer)
            .set("cpu_usage", trade.cpu_usage)
            .set("contract", trade.contract)
            .set("executor", trade.executor)
            .set("borrow", trade.borrow.unwrap().to_string())
            .set("profit", trade.profit.as_ref().unwrap().to_string())
            .set("profit_symbol", trade.profit.as_ref().unwrap().symbol.clone())
            .set("profit_value", BigDecimal::try_from(trade.profit.as_ref().unwrap().value).unwrap())
            .set(
                "path",
                trade
                    .quantities
                    .iter()
                    .cycle()
                    .skip(trade.quantities.len() - 1)
                    .take(trade.quantities.len())
                    .map(|asset| asset.symbol.clone())
                    .collect::<Vec<_>>()
                    .join("/"),
            )
            .set("hops", trade.codes.len() as u32)
            .set("codes", trade.codes.join("/"));
    });

    Ok(tables.to_database_changes())
}

impl FromStr for Asset {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let quantity = parts.next().ok_or("missing quantity")?;
        let value = quantity.parse::<f64>().map_err(|_| format!("invalid value"))?;
        let symbol = parts.next().ok_or("missing symbol")?.to_string();
        let precision = quantity.split('.').nth(1).unwrap_or("").len() as u32;
        let amount = (value * 10f64.powi(precision as i32)) as u64;
        Ok(Self {
            value,
            amount,
            symbol,
            precision,
        })
    }
}

impl fmt::Display for Asset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.precision$} {}", self.value, self.symbol, precision = self.precision as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_from_str() {
        let test_cases = [
            ("123.0000 SYM", 123.0, 1230000, "SYM", 4),
            ("123 SYM", 123.0, 123, "SYM", 0),
            ("123456.123456789 SYM", 123456.123456789, 123456123456789, "SYM", 9),
            ("0.0 SYM", 0.0, 0, "SYM", 1),
            ("0 SYM", 0.0, 0, "SYM", 0),
        ];

        for (input, value, amount, symbol, precision) in test_cases.iter() {
            let asset = input.parse::<Asset>().unwrap();
            assert_eq!(asset.value, *value);
            assert_eq!(asset.amount, *amount);
            assert_eq!(asset.symbol, *symbol);
            assert_eq!(asset.precision, *precision);
        }
    }
}
