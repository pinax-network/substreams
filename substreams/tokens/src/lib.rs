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
            if let Ok(ext_sym) = util::extract_extsym_from_create(trace.action.as_ref().unwrap()) {

                log::debug!("Found {}::create action in block #{}", ext_sym.contract, blk.number);
                tokens.push(tokens::Token {
                    block_num: blk.number,
                    timestamp: Some(blk.header.as_ref().unwrap().timestamp.as_ref().unwrap().clone()),
                    contract: ext_sym.contract,
                    symcode: ext_sym.symbol.symcode,
                    precision: ext_sym.symbol.precision as u32,
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