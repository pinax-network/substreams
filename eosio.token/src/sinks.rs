use substreams::errors::Error;
use substreams_sink_winston::{Logger, Meta, LoggerOperations};
use substreams_database_change::pb::database::{DatabaseChanges, table_change::Operation};

use crate::TransferEvents;

#[substreams::handlers::map]
pub fn db_out(map_transfers: TransferEvents) -> Result<DatabaseChanges, Error> {

    let mut db_out = DatabaseChanges::default();

    for transfer in map_transfers.items {
        let pk = format!("{}-{}", transfer.trx_id, transfer.action_ordinal);
        db_out.push_change("transfer", pk.as_str(), 0, Operation::Create)
            // transaction
            .change("trx_id", ("", transfer.trx_id.as_str()))
            .change("action_ordinal", ("", transfer.action_ordinal.to_string().as_str()))

            // contract & scope
            .change("account", ("", transfer.account.as_str()))
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
pub fn log_out(map_transfers: TransferEvents) -> Result<LoggerOperations, Error> {

    let mut log_out = LoggerOperations::default();

    let logger = Logger::new("eosio.token");
    for transfer in map_transfers.items {
        let mut meta = Meta::new();

        // transaction
        meta.insert("trx_id", transfer.trx_id.as_str());
        meta.insert("action_ordinal", transfer.action_ordinal.to_string().as_str());

        // contract & scope
        meta.insert("account", transfer.account.as_str());
        meta.insert("symcode", transfer.symcode.as_str());

        // data payload
        meta.insert("from", transfer.from.as_str());
        meta.insert("to", transfer.to.as_str());
        meta.insert("memo", transfer.memo.as_str());
        meta.insert("quantity", transfer.quantity.as_str());

        // extras
        meta.insert("amount", transfer.amount.to_string().as_str());
        meta.insert("precision", transfer.precision.to_string().as_str());

        log_out.push(logger.info("transfer").with(meta));
    }

    Ok(log_out)
}
