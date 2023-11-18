use substreams::errors::Error;
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};
use substreams_sink_kv::pb::sf::substreams::sink::kv::v1::KvOperations;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams::pb::substreams::Clock;

use crate::antelope_oracles::Quotes;

// Work In Progress: Make a generic db_out for oracle information
#[substreams::handlers::map]
pub fn db_out(map_quotes: Quotes) -> Result<DatabaseChanges, Error> {
    let mut db_out = DatabaseChanges::default();

    for quote in map_quotes.quotes {
        db_out
            .push_change("id", &quote.value.as_ref().unwrap().id.to_string(), 0, table_change::Operation::Create)
            .change("pair", ("", quote.pair.as_str()))
            .change("median", ("", quote.value.as_ref().unwrap().median.to_string().as_ref()))
            .change("owner", ("", quote.value.as_ref().unwrap().owner.as_str()))
            .change("timestamp", ("", quote.value.as_ref().unwrap().timestamp.as_str()))
            .change("value", ("", quote.value.as_ref().unwrap().value.to_string().as_ref()));
    }

    Ok(db_out)
}

#[substreams::handlers::map]
pub fn kv_out(map_quotes: Quotes, clock: Clock) -> Result<KvOperations, Error> {
    let mut kv_out = KvOperations::default();
    let seconds = clock.timestamp.unwrap().seconds;
    let epoch = (seconds / 86400) * 86400;

    let _day = epoch / 86400;

    for quote in map_quotes.quotes {
        let key = format!("delphioracle:{}:{}", quote.value.as_ref().ok_or(0).unwrap().timestamp, quote.pair);
        kv_out.push_new(key, &quote.value.ok_or(0).unwrap().value.to_ne_bytes(), 1);
    }

    Ok(kv_out)
}

#[substreams::handlers::map]
pub fn graph_out(map_quotes: Quotes) -> Result<EntityChanges, Error> {
    let mut table = Tables::new();

    for quote in map_quotes.quotes {
        let datapoint = quote.value.unwrap();
        let row = table.create_row("quotes", &quote.pair).set("q_pair", &quote.pair);
        row.set("q_id", datapoint.id);
        row.set("q_median", datapoint.median);
        row.set("q_owner", datapoint.owner);
        row.set("q_timestamp", datapoint.timestamp);
        row.set("q_value", datapoint.value);
    }

    Ok(table.to_entity_changes())
}
