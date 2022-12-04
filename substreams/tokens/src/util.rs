use crate::abi;
use substreams_antelope_core::asset::Asset;
use substreams_antelope_core::extended_symbol::ExtendedSymbol;
use substreams_antelope_core::pb::antelope::Action;

type Err = String;
pub(crate) type Result<T> = std::result::Result<T, Err>;

// extract extended symbol from create action
pub(crate) fn extract_extsym_from_create(action: &Action) -> Result<ExtendedSymbol> {
    if action.name != "create".to_string() {
        return Err("Wrong action".to_string());
    }
    let params = action.json_data.parse::<abi::Create>()?;
    let asset = params.maximum_supply.parse::<Asset>()?;

    Ok(ExtendedSymbol{
        contract: action.account.clone(),
        symbol: asset.symbol
    })
}