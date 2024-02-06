use sinkfiles::Lines;
use substreams::errors::Error;
use substreams_antelope::pb::{d_trx_op, Block};

#[path = "pb/antelope.eosio.deferred.v1.rs"]
#[allow(dead_code)]
pub mod eosio_deferred;
#[path = "pb/substreams.sink.files.v1.rs"]
#[allow(dead_code)]
pub mod sinkfiles;
pub use self::eosio_deferred::*;

fn get_op_str(op: d_trx_op::Operation) -> String {
    match op {
        d_trx_op::Operation::Create => "CREATE".to_string(),
        d_trx_op::Operation::Cancel => "CANCEL".to_string(),
        d_trx_op::Operation::Unknown => "UNKNOWN".to_string(),
        d_trx_op::Operation::PushCreate => "PUSHCREATE".to_string(),
        d_trx_op::Operation::Failed => "FAILED".to_string(),
        d_trx_op::Operation::ModifyCancel => "MODIFYCANCEL".to_string(),
        d_trx_op::Operation::ModifyCreate => "MODIFYCREATE".to_string(),
    }
}

#[substreams::handlers::map]
fn map_deferred(block: Block) -> Result<Transactions, Error> {
    let mut res: Transactions = Default::default();
    let producer = block.header.as_ref().unwrap().producer.clone();

    for trx in block.all_transaction_traces() {
        for dtrx in &trx.dtrx_ops {
            let dtrx = dtrx.clone();
            let actions = dtrx.transaction.unwrap().transaction.unwrap().actions;
            let action = if actions.len() != 0 {
                actions.get(0).unwrap().clone()
            } else {
                substreams_antelope::pb::Action { ..Default::default() }
            };
            res.transactions.push(Transaction {
                parent_trx_id: trx.id.clone(),
                trx_id: dtrx.transaction_id,
                op: get_op_str(d_trx_op::Operation::from_i32(dtrx.operation).unwrap()),
                sender: dtrx.sender,
                account: action.account,
                action: action.name,
                json_data: action.json_data,
                block_num: trx.block_num,
                timestamp: block.header.as_ref().unwrap().timestamp.clone(),
                producer: producer.to_string(),
            });
        }
    }
    Ok(res)
}

#[substreams::handlers::map]
fn csv_out(transactions: Transactions) -> Result<Lines, substreams::errors::Error> {
    Ok(Lines {
        lines: transactions
            .transactions
            .into_iter()
            .map(|t| {
                format!(
                    "{},{},{},{},{},{},\"{}\",{},{},{}",
                    t.trx_id,
                    t.parent_trx_id,
                    t.op,
                    t.sender,
                    t.account,
                    t.action,
                    t.json_data.replace("\"", "'"),
                    t.block_num,
                    t.timestamp.unwrap().to_string(),
                    t.producer
                )
            })
            .collect(),
    })
}
