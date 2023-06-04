use std::collections::{HashSet};

use substreams::log;
use substreams_database_change::change::AsString;

// Build query-params
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
    log::debug!("filter: {:?} {:?}", key, filter);
    filter
}

pub fn is_amount_gt(amount: i64, filter_amountgt: &HashSet<String>) -> bool {
    if let Some(filter_amount) = filter_amountgt.iter().next() {
        if let Ok(filter_amount_number) = filter_amount.parse::<i64>() {
            return amount > filter_amount_number;
        }
    }
    
    false
}

pub fn is_amount_lt(amount: i64, filter_amountlt: &HashSet<String>) -> bool {
    if let Some(filter_amount) = filter_amountlt.iter().next() {
        if let Ok(filter_amount_number) = filter_amount.parse::<i64>() {
            return amount < filter_amount_number;
        }
    }
    
    false
}

pub fn is_balance_gt(amount: &str, filter_amountgt: &HashSet<String>) -> bool {
    if let Some(filter_amount) = filter_amountgt.iter().next() {
        if let Ok(filter_amount_number) = filter_amount.parse::<f64>() {
            // Extract the numeric part of the amount string
            let amount_numeric_part: String = amount.chars().filter(|c| c.is_numeric() || *c == '.').collect();

            if let Ok(amount_number) = amount_numeric_part.parse::<f64>() {
                return amount_number > filter_amount_number;
            } else {
                log::debug!("Could not convert amount: {}", amount);
            }
        } else {
            log::debug!("balance_lt is not a number: {}", filter_amount);
        }
    }
    
    false
}

pub fn is_balance_lt(amount: &str, filter_amountlt: &HashSet<String>) -> bool {
    if let Some(filter_amount) = filter_amountlt.iter().next() {
        if let Ok(filter_amount_number) = filter_amount.parse::<f64>() {
            // Extract the numeric part of the amount string
            let amount_numeric_part: String = amount.chars().filter(|c| c.is_numeric() || *c == '.').collect();

            if let Ok(amount_number) = amount_numeric_part.parse::<f64>() {
                return amount_number < filter_amount_number;
            } else {
                log::debug!("Could not convert amount: {}", amount);
            }
        } else {
            log::debug!("balance_lt is not a number: {}", filter_amount);
        }
    }

    false
}