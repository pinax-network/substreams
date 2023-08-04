mod abi;
mod pb;

use pb::antelope::gems::blend::v1::{Actions, BlendLog, NftExtra};

#[substreams::handlers::map]
fn map_actions(block: substreams_antelope::pb::Block) -> Result<Actions, substreams::errors::Error> {
    Ok(Actions {
        blendlogs: block.actions::<abi::contract::actions::Blendlog>(&[abi::contract::ACCOUNT.expect("specify contract account")])
            .map(|(action, trx)| BlendLog {
                trx_id: trx.transaction_id.clone(),
                trx_index: trx.action_ordinal,

                owner: action.owner,
                description: action.description,
                in_asset_ids: action.in_asset_ids,
                out_asset_id: action.out_asset_id,
                in_templates: action
                    .in_templates
                    .iter()
                    .map(|template| NftExtra {
                        collection_name: template.collection_name.clone(),
                        template_id: template.template_id,
                        schema_name: template.schema_name.clone(),
                    })
                    .collect::<Vec<_>>(),
                out_template: Some(NftExtra {
                    collection_name: action.out_template.collection_name,
                    template_id: action.out_template.template_id,
                    schema_name: action.out_template.schema_name,
                }),
                total_mint: action.total_mint,
                total_burn: action.total_burn,
            })
            .collect(),
    })
}