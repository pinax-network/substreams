use substreams::{errors::Error, log};
use substreams_antelope::{Block, d_trx_op};

#[path = "pb/antelope.eosio.deferred.v1.rs"]
#[allow(dead_code)]
pub mod eosio_deferred;
pub use self::eosio_deferred::*;

#[substreams::handlers::map]
fn map_deferred(block: Block) -> Result<Transactions, Error> {
    let mut res: Transactions = Default::default();
    for trx in block.all_transaction_traces() {
        for dtrx in &trx.dtrx_ops {
            if dtrx.operation != d_trx_op::Operation::Create.into() {
                continue;
            }
            let dtrx = dtrx.clone();
            let actions = dtrx.transaction.unwrap().transaction.unwrap().actions.clone();
            let action = if actions.len() != 0 { actions.get(0).unwrap().clone() } else { substreams_antelope::Action { ..Default::default() } };
            res.transactions.push(Transaction {
                trx_id: dtrx.transaction_id,
                sender: dtrx.sender,
                account: action.account,
                action: action.name,
                json_data: action.json_data,
                block_num: trx.block_num,
                timestamp: block.header.as_ref().unwrap().timestamp.clone(),
            });
        }
    }
    Ok(res)
}
