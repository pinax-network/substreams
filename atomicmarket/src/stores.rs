use substreams::prelude::*;
use substreams_antelope::ActionTraces;
use crate::abi;

#[substreams::handlers::store]
fn store_new_sales(action_traces: ActionTraces, s: StoreAddInt64) {
    for action_trace in action_traces.action_traces {
        match abi::parse_lognewsale(&action_trace) {
            Some(new_sale) => {
                s.add(1, new_sale.collection_name, new_sale.listen_price);
            },
            None => (),
        }
    }
}
