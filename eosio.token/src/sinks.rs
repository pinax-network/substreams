use antelope::Asset;
use substreams::errors::Error;
use substreams_entity_change::pb::entity::{entity_change::Operation, EntityChanges};
use substreams_sink_kv::pb::sf::substreams::sink::kv::v1::KvOperations;

use crate::eosio_token::{Accounts, Stats, TransferEvents};

#[substreams::handlers::map]
pub fn graph_out(map_transfers: TransferEvents) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();

    for transfer in map_transfers.items {
        entity_changes
            .push_change("transfer", transfer.trx_id.as_str(), 0, Operation::Create)
            // transaction
            .change("trx_id", transfer.trx_id)
            .change("action_ordinal", transfer.action_ordinal.to_string())
            // contract & scope
            .change("contract", transfer.contract)
            .change("symcode", transfer.symcode)
            // data payload
            .change("from", transfer.from)
            .change("to", transfer.to)
            .change("memo", transfer.memo)
            .change("quantity", transfer.quantity)
            // extras
            .change("amount", transfer.amount.to_string())
            .change("precision", transfer.precision.to_string());
    }

    Ok(entity_changes)
}

#[substreams::handlers::map]
pub fn kv_out(map_accounts: Accounts, map_stat: Stats) -> Result<KvOperations, Error> {
    let mut kv_ops: KvOperations = Default::default();

    let mut ordinal = 1;
    for account in map_accounts.items {
        let asset = Asset::from(account.balance.as_str());
        let key = format!(
            "accounts:{}:{}:{}",
            account.account, account.contract, asset.symbol
        );
        kv_ops.push_new(key, asset.amount.to_string(), ordinal);
        ordinal += 1;
    }

    ordinal = 1;
    for stat in map_stat.items {
        let asset = Asset::from(stat.supply.as_str());
        let key = format!("stat:{}:{}", stat.contract, asset.symbol);
        kv_ops.push_new(key, asset.amount.to_string(), ordinal);
        ordinal += 1;
    }

    Ok(kv_ops)
}
