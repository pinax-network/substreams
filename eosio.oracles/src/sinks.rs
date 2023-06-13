use antelope::Asset;
use substreams::errors::Error;
use substreams_entity_change::pb::entity::{entity_change, EntityChanges};
use substreams_sink_kv::pb::sf::substreams::sink::kv::v1::KvOperations;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

use crate::eosio_oracles::{Pairs};

// Work In Progress: Make a generic db_out for oracle information
#[substreams::handlers::map]
pub fn db_out(map_pairs: Pairs) -> Result<DatabaseChanges, Error> {
    let mut db_out = DatabaseChanges::default();

    for pair in map_pairs.pairs {
        db_out
            .push_change("pair", pair.name.as_str(), 0, table_change::Operation::Create)
            .change("active", ("", pair.active.to_string().as_str())) 
            .change("bounty_awarded", ("", pair.bounty_awarded.to_string().as_str())) 
            .change("bounty_edited_by_custodians", ("", pair.bounty_edited_by_custodians.to_string().as_str())) 
            .change("proposer", ("", pair.proposer.as_str())) 
            .change("name", ("", pair.name.as_str()))
            .change("bounty_amount", ("", pair.bounty_amount.as_str())) 
            //.change("approving_custodians", ("", pair.approving_custodians.as_str())) 
            //.change("approving_oracles", ("", pair.approving_oracles.as_str())) 
            .change("base_symbol", ("", pair.base_symbol.as_str())) 
            .change("base_type", ("", pair.base_type.to_string().as_str())) 
            .change("base_contract", ("", pair.base_contract.as_str())) 
            .change("quote_symbol", ("", pair.quote_symbol.as_str())) 
            .change("quote_type", ("", pair.quote_type.to_string().as_str())) 
            .change("quote_contract", ("", pair.quote_contract.as_str())) 
            .change("quoted_precision", ("", pair.quoted_precision.to_string().as_str()));
    }

    Ok(db_out)
}