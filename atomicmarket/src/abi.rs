use substreams_antelope::ActionTrace;
use antelope::Asset;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct LogNewSale {
    pub sale_id: u64,
    pub seller: String,
    pub asset_ids: Vec<String>,
    pub listing_price: String,
    pub settlement_symbol: String,
    pub maker_marketplace: String,
    pub collection_name: String,
    pub collection_fee: String,
}

pub struct NewSale {
    pub collection_name: String,
    pub listen_price: i64,
}

pub fn parse_lognewsale(action_trace: &ActionTrace) -> Option<NewSale> {
    let action = action_trace.action.as_ref()?;
    if action.name != "lognewsale" { return None; }
    let data: LogNewSale = serde_json::from_str(&action.json_data).unwrap();
    Some(NewSale {
        collection_name: data.collection_name,
        listen_price: Asset::from(data.listing_price.as_str()).amount,
    })
}
