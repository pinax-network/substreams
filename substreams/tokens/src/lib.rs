use substreams::{prelude::*, store, log};
use substreams::store::{StoreSetProto};
use crate::pb::tokens;

mod pb;
mod util;
mod asset;
mod abi;

/// Extracts blockmeta from each block
#[substreams::handlers::map]
fn map_tokens(blk: substreams_antelope_core::pb::antelope::Block) -> Result<tokens::Tokens, substreams::errors::Error> {
    let mut tokens = vec![];

    for trx in blk.clone().all_transaction_traces() {
        for action in &trx.action_traces {
            if action.action.as_ref().unwrap().name == String::from("create") {
                let contract = action.action.as_ref().unwrap().account.clone();
                let data = action.action.as_ref().unwrap().json_data.clone();
                let json: abi::CreateTokenAction = serde_json::from_str(&data).unwrap();
                let sym = util::extract_symbol(json.maximum_supply);

                log::debug!("Found {}::create action in block #{}, data: {}", contract, blk.number, data);
                tokens.push(tokens::Token {
                    block_num: blk.number,
                    timestamp: Some(blk.header.as_ref().unwrap().timestamp.as_ref().unwrap().clone()),
                    contract,
                    symcode: sym.symcode,
                    precision: sym.precision as u32,
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
        let token_id = format!("{},{}@{}", token.symcode, token.precision, token.contract);
        s.set(1, token_id, &token);
    }
}