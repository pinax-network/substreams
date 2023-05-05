use std::collections::{HashSet};

use substreams::log;
use substreams_database_change::change::AsString;

// Build query-params
//
// "to=swap.defi,swap.rome&symcode=EOS"
// all EOS symcode transfers to these two accounts
pub fn create_filters(params: &str, key: &str) -> HashSet<String> {
    let mut filter = HashSet::new();
    for part in params.split("&").collect::<Vec<&str>>() {
        let kv = part.split("=").collect::<Vec<&str>>();
        if ( kv.len() != 2 ) || ( kv[0] != key ) {
            continue;
        }
        for item in kv[1].split("+").collect::<Vec<&str>>() {
            filter.insert(item.as_string());
        }
    }
    log::debug!("filter: {:?} {:?}", key, filter);
    filter
}
