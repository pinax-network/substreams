use antelope::Asset;
use substreams::errors::Error;
use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};
use substreams_sink_kv::pb::sf::substreams::sink::kv::v1::KvOperations;

use crate::eosio_token::{Accounts, Stats, TransferEvents};

#[substreams::handlers::map]
pub fn db_out(map_transfers: TransferEvents) -> Result<DatabaseChanges, Error> {
    let mut db_out = DatabaseChanges::default();

    for transfer in map_transfers.items {
        let pk = format!("{}-{}", transfer.trx_id, transfer.action_ordinal);
        db_out
            .push_change("transfer", pk.as_str(), 0, Operation::Create)
            // transaction
            .change("trx_id", ("", transfer.trx_id.as_str()))
            .change(
                "action_ordinal",
                ("", transfer.action_ordinal.to_string().as_str()),
            )
            // contract & scope
            .change("contract", ("", transfer.contract.as_str()))
            .change("symcode", ("", transfer.symcode.as_str()))
            // data payload
            .change("from", ("", transfer.from.as_str()))
            .change("to", ("", transfer.to.as_str()))
            .change("memo", ("", transfer.memo.as_str()))
            .change("quantity", ("", transfer.quantity.as_str()))
            // extras
            .change("amount", ("", transfer.amount.to_string().as_str()))
            .change("precision", ("", transfer.precision.to_string().as_str()));
    }

    Ok(db_out)
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
