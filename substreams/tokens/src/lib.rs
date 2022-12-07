use substreams::{prelude::*, store, log};
use substreams::store::{StoreSetProto};
use crate::pb::tokens;

mod pb;
mod util;
mod abi;



/// Extracts blockmeta from each block
#[substreams::handlers::map]
fn map_tokens(blk: substreams_antelope_core::pb::antelope::Block) -> Result<tokens::Tokens, substreams::errors::Error> {
    let mut tokens = vec![];

    for trx in blk.clone().all_transaction_traces() {
        for trace in &trx.action_traces {
            if let Ok(data) = util::parse_create_action(trace.action.as_ref().unwrap()) {

                log::debug!("Found {}::create action in block #{}", data.maximum_supply.contract.to_string(), blk.number);
                tokens.push(tokens::Token {
                    block_num: blk.number,
                    timestamp: Some(blk.header.as_ref().unwrap().timestamp.as_ref().unwrap().clone()),
                    contract: data.maximum_supply.contract.to_string(),
                    symcode: data.maximum_supply.quantity.symbol.code().to_string(),
                    precision: data.maximum_supply.quantity.symbol.precision() as u32,
                    amount: data.maximum_supply.quantity.amount,
                    issuer: data.issuer.to_string(),
                })
            }
        }
    }
    Ok(tokens::Tokens { tokens })
}


#[substreams::handlers::store]
fn store_tokens(tokens: tokens::Tokens, s: store::StoreSetProto<tokens::Token>) {
    for token in tokens.tokens {
        log::debug!("storing new token {},{}@{} from block {} at {}", token.symcode, token.precision, token.contract, token.block_num, token.clone().timestamp.unwrap().to_string());
        let token_key = format!("{},{}@{}", token.symcode, token.precision, token.contract);
        s.set(1, token_key, &token);
    }
}