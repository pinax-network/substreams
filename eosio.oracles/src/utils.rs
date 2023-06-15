use std::collections::{HashSet};

use antelope::{Asset};
use substreams::log;
use substreams_database_change::change::AsString;

// Filter query-params
//
// "to=swap.defi,swap.rome&symcode=EOS"
// all EOS symcode transfers to these two accounts
pub fn create_filters(params: &str, key: &str) -> HashSet<String> {
    let mut filter = HashSet::new();
    for part in params.split('&').collect::<Vec<&str>>() {
        let kv = part.split('=').collect::<Vec<&str>>();
        if ( kv.len() != 2 ) || ( kv[0] != key ) {
            continue;
        }
        for item in kv[1].split(',').collect::<Vec<&str>>() {
            filter.insert(item.as_string());
        }
    }
    if !filter.is_empty() {
        log::debug!("create_filters: {:?} {:?}", key, filter);
    }
    filter
}

// Create i64 filter (used for Asset::amount)
//
// "amount_gt=10000"
// Amount greater than "1.0000 EOS"
pub fn create_i64_filter(params: &str, key: &str) -> Option<i64> {
    for part in params.split('&').collect::<Vec<&str>>() {
        let kv = part.split('=').collect::<Vec<&str>>();
        if ( kv.len() != 2 ) || ( kv[0] != key ) {
            continue;
        }
        // TO-DO throw error if "," in param value
        if let Ok(amount) = kv[1].parse::<i64>() {
            log::debug!("create_i64_filter: {:?} {:?}", key, amount);
            return Some(amount);
        }
    }
    Option::None
}

pub fn remove_quotes_from_value(json_str: String, key: String) -> String {
    let mut json_data = json_str.clone();
    let first_char_value_offset = json_data.find(&key).unwrap() + key.len() + 2; // +2 to account for the '":' characters of the key

    if json_data.chars().nth(first_char_value_offset).unwrap() == '"' {
        json_data.remove(first_char_value_offset);
        json_data.remove(first_char_value_offset + json_data[first_char_value_offset..].find('"').unwrap());
    }

    json_data
}

pub fn to_value(quantity: Asset) -> f64 {
    quantity.amount as f64 / 10_i64.pow(quantity.symbol.precision() as u32) as f64
}
