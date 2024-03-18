use substreams::errors::Error;

use substreams_antelope::pb::Block;

use crate::abi;
use crate::eosio_cpu::*;
use crate::utils;
use antelope::Asset;

#[substreams::handlers::map]
fn map_transfers(params: String, block: Block) -> Result<TransferEvents, Error> {
    let mut response = vec![];

    // query-params
    let filter_from = utils::create_filters(params.as_str(), "from");
    let filter_to = utils::create_filters(params.as_str(), "to");
    let filter_symcode = utils::create_filters(params.as_str(), "symcode");
    let filter_contract = utils::create_filters(params.as_str(), "contract");
    let filter_to_or_from = utils::create_filters(params.as_str(), "to_or_from");

    let _filter_quantity_lt = utils::create_i64_filter(params.as_str(), "quantity_lt");
    let _filter_quantity_gt = utils::create_i64_filter(params.as_str(), "quantity_gt");
    let _filter_quantity_lte = utils::create_i64_filter(params.as_str(), "quantity_lte");
    let _filter_quantity_gte = utils::create_i64_filter(params.as_str(), "quantity_gte");
    let mut transaction_count: u32 = 0;

    for trx in block.executed_transaction_traces() {
        let cpu_usage = trx.receipt.as_ref().unwrap().cpu_usage_micro_seconds as u32;
        let net_usage = trx.net_usage as u32;
        let producer = block.header.as_ref().unwrap().producer.clone();

        // action traces
        for trace in &trx.action_traces {
            let action_trace = trace.action.as_ref().unwrap();

            if action_trace.account != trace.receiver {
                continue;
            }
            if action_trace.name != "transfer" {
                continue;
            }

            match abi::Transfer::try_from(action_trace.json_data.as_str()) {
                Ok(data) => {
                    let quantity = Asset::from(data.quantity.as_str());
                    let symcode = quantity.symbol.code().to_string();
                    //let precision = quantity.symbol.precision().into();
                    //let amount = quantity.amount;
                    let contract = action_trace.account.clone();

                    // filter by params
                    if !filter_from.is_empty() && !filter_from.contains(&data.from) {
                        continue;
                    }
                    if !filter_to.is_empty() && !filter_to.contains(&data.to) {
                        continue;
                    }
                    if !filter_symcode.is_empty() && !filter_symcode.contains(&symcode) {
                        continue;
                    }
                    if !filter_contract.is_empty() && !filter_contract.contains(&contract) {
                        continue;
                    }
                    if !filter_to_or_from.is_empty() && !(filter_to_or_from.contains(&data.to) || filter_to_or_from.contains(&data.from)) {
                        continue;
                    }
                    //if filter_quantity_lt.is_some() && !(quantity.amount < filter_quantity_lt.unwrap()) { continue; }
                    //if filter_quantity_gt.is_some() && !(quantity.amount > filter_quantity_gt.unwrap()) { continue; }
                    //if filter_quantity_lte.is_some() && !(quantity.amount <= filter_quantity_lte.unwrap()) { continue; }
                    //if filter_quantity_gte.is_some() && !(quantity.amount >= filter_quantity_gte.unwrap()) { continue; }

                    transaction_count += 1;

                    response.push(TransferEvent {
                        // trace information
                        trx_id: trx.id.clone(),
                        //action_ordinal: trace.action_ordinal,

                        // contract & scope
                        contract,
                        action: action_trace.name.clone(),
                        symcode,

                        // payload
                        from: data.from,
                        to: data.to,
                        quantity: data.quantity,
                        memo: data.memo,

                        // extras
                        //precision,
                        //amount,
                        //value: utils::to_value(quantity),

                        // cpu
                        producer: producer.to_string(),
                        cpu_usage: cpu_usage,
                        net_usage: net_usage,
                        tx_count: transaction_count,
                    });
                }
                Err(_) => continue,
            }
        }
    }
    Ok(TransferEvents { items: response })
}
