mod abi;
mod pb;

use pb::antelope::pomelo::bounties::v1::{Actions, StateLog, Apply, CreateLog, ExtendedSymbol, ClaimLog, ExtendedAsset};

#[substreams::handlers::map]
fn map_actions(param_account: String, block: substreams_antelope::pb::Block) -> Result<Actions, substreams::errors::Error> {
    Ok(Actions {
        statelogs: block.actions::<abi::contract::actions::Statelog>(&[param_account.as_str()])
            .map(|(action, trx)| StateLog {
                trx_id: trx.transaction_id.clone(),
                trx_index: trx.action_ordinal,

                action: action.action,
                state: action.status,
                bounty_id: action.bounty_id,
            })
            .collect(),
        applys: block.actions::<abi::contract::actions::Apply>(&[param_account.as_str()])
            .map(|(action, trx)| Apply {
                trx_id: trx.transaction_id.clone(),
                trx_index: trx.action_ordinal,

                bounty_id: action.bounty_id,
                user_id: action.user_id,
            })
            .collect(),
        createlogs: block.actions::<abi::contract::actions::Createlog>(&[param_account.as_str()])
            .map(|(action, trx)| CreateLog {
                trx_id: trx.transaction_id.clone(),
                trx_index: trx.action_ordinal,

                bounty_id: action.bounty_id,
                author_user_id: action.author_user_id,
                r#type: action.type_,
                permissions: action.permissions,
                ext_sym: Some(ExtendedSymbol {
                    sym: action.ext_sym.sym,
                    contract: action.ext_sym.contract,
                }),

            })
            .collect(),
        claimlogs: block.actions::<abi::contract::actions::Claimlog>(&[param_account.as_str()])
            .map(|(action, trx)| ClaimLog {
                trx_id: trx.transaction_id.clone(),
                trx_index: trx.action_ordinal,

                bounty_id: action.bounty_id,
                receiver: action.receiver,
                fee: action.fee,
                ext_quantity: Some(ExtendedAsset{
                    quantity: action.ext_quantity.quantity,
                    contract: action.ext_quantity.contract,
                }),
                status: action.status,
                worker_user_id: action.worker_user_id,
                days_since_created: action.days_since_created,
            })
            .collect(),
    })
}