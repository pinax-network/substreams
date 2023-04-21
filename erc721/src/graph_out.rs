use substreams_entity_change::pb::entity::{ entity_change::Operation, EntityChanges };
use crate::pb::eth::erc721::v1 as Proto;

#[allow(non_snake_case)]
pub fn transfers_to_entities_changes(changes: &mut EntityChanges, transfers: Proto::Transfers) {
    for transfer in transfers.transfers {
        changes
            .push_change("transfer", &transfer.trx_hash, transfer.ordinal, Operation::Create)
            .change("from", transfer.from)
            .change("to", transfer.to)
            .change("trx_hash", transfer.trx_hash)
            .change("tokenid", transfer.token_id)
            .change("contract_address", transfer.contract_address)
            .change("ordinal", transfer.ordinal.to_string());
    }
}

#[allow(non_snake_case)]
pub fn approvals_to_entities_changes(changes: &mut EntityChanges, approvals: Proto::Approvals) {
    for approval in approvals.approval {
        changes
            .push_change("approval", &approval.trx_hash, approval.ordinal, Operation::Create)
            .change("owner", approval.owner)
            .change("approuved", approval.approved)
            .change("trx_hash", approval.trx_hash)
            .change("tokenid", approval.token_id)
            .change("contract_address", approval.contract_address)
            .change("ordinal", approval.ordinal.to_string());
    }
}

#[allow(non_snake_case)]
pub fn approvals_for_all_to_entities_changes(
    changes: &mut EntityChanges,
    approvals: Proto::ApprovalForAlls
) {
    for approval in approvals.approvalall {
        changes
            .push_change("approvalforall", &approval.trx_hash, approval.ordinal, Operation::Create)
            .change("owner", approval.owner)
            .change("operator", approval.operator)
            .change("approuved", approval.approved)
            .change("trx_hash", approval.trx_hash)
            .change("contract_address", approval.contract_address)
            .change("ordinal", approval.ordinal.to_string());
    }
}