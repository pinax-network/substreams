use std::str::FromStr;

use crate::abi;
use substreams_antelope_core::pb::antelope::Action;
use eosio::*;

type Err = String;
pub(crate) type Result<T> = std::result::Result<T, Err>;

// extract extended symbol from create action
pub(crate) fn extract_extsym_from_create(action: &Action) -> Result<ExtendedSymbol> {
    if action.name != "create".to_string() {
        return Err("Wrong action".to_string());
    }
    let params = action.json_data.parse::<abi::Create>().map_err(|_| "Invalid Create params".to_string())?;
    let asset = params.maximum_supply.parse::<Asset>().map_err(|_| "Invalid maximum_supply".to_string())?;

    Ok(ExtendedSymbol{
        contract: AccountName::from_str(action.account.as_str()).map_err(|_| "Invalid token account".to_string())?,
        symbol: asset.symbol
    })
}