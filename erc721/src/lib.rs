mod abi;
mod pb;
#[path = "graph_out.rs"]
mod graph;
use substreams_ethereum::Event;
use substreams_ethereum::{ pb::eth::v2 as eth };
use abi::erc721::events::Transfer as ERC721TransferEvent;
use abi::erc721::events::Approval as ERC721ApprovalEvent;
use abi::erc721::events::ApprovalForAll as ERC721ApprovalForAllEvent;
use pb::eth::erc721::v1 as Proto;
use substreams::Hex;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams::errors::Error;
substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_approvals(blk: eth::Block) -> Result<Proto::Approvals, substreams::errors::Error> {
    Ok(Proto::Approvals {
        approval: get_approvals(&blk).collect(),
    })
}

fn get_approvals<'a>(blk: &'a eth::Block) -> impl Iterator<Item = Proto::Approval> + 'a {
    blk.receipts().flat_map(|receipt| {
        let hash = &receipt.transaction.hash;
        receipt.receipt.logs.iter().flat_map(|log| {
            if let Some(event) = ERC721ApprovalEvent::match_and_decode(log) {
                return vec![new_erc721_approval(log, hash, event)];
            }

            vec![]
        })
    })
}

fn new_erc721_approval(
    log: &substreams_ethereum::pb::eth::v2::Log,
    hash: &[u8],
    event: ERC721ApprovalEvent
) -> Proto::Approval {
    Proto::Approval {
        owner: Hex(&event.owner).to_string(),
        approved: Hex(&event.approved).to_string(),
        token_id: event.token_id.to_string(),
        contract_address: Hex(&log.address).to_string(),
        trx_hash: Hex(hash).to_string(),
        ordinal: log.block_index as u64,
    }
}

#[substreams::handlers::map]
fn map_transfers(blk: eth::Block) -> Result<Proto::Transfers, substreams::errors::Error> {
    Ok(Proto::Transfers {
        transfers: get_transfers(&blk).collect(),
    })
}

fn get_transfers<'a>(blk: &'a eth::Block) -> impl Iterator<Item = Proto::Transfer> + 'a {
    blk.receipts().flat_map(|receipt| {
        let hash = &receipt.transaction.hash;
        receipt.receipt.logs.iter().flat_map(|log| {
            if let Some(event) = ERC721TransferEvent::match_and_decode(log) {
                return vec![new_erc721_transfer(log, hash, event)];
            }

            vec![]
        })
    })
}

fn new_erc721_transfer(
    log: &substreams_ethereum::pb::eth::v2::Log,
    hash: &[u8],
    event: ERC721TransferEvent
) -> Proto::Transfer {
    Proto::Transfer {
        from: Hex(&event.from).to_string(),
        to: Hex(&event.to).to_string(),
        token_id: event.token_id.to_string(),
        contract_address: Hex(&log.address).to_string(),
        trx_hash: Hex(hash).to_string(),
        ordinal: log.block_index as u64,
    }
}

#[substreams::handlers::map]
fn map_approval_for_all(
    blk: eth::Block
) -> Result<Proto::ApprovalForAlls, substreams::errors::Error> {
    Ok(Proto::ApprovalForAlls {
        approvalall: get_approval_for_alls(&blk).collect(),
    })
}

fn get_approval_for_alls<'a>(
    blk: &'a eth::Block
) -> impl Iterator<Item = Proto::ApprovalForAll> + 'a {
    blk.receipts().flat_map(|receipt| {
        let hash = &receipt.transaction.hash;
        receipt.receipt.logs.iter().flat_map(|log| {
            if let Some(event) = ERC721ApprovalForAllEvent::match_and_decode(log) {
                return vec![new_erc721_approval_for_all(log, hash, event)];
            }

            vec![]
        })
    })
}

fn new_erc721_approval_for_all(
    log: &substreams_ethereum::pb::eth::v2::Log,
    hash: &[u8],
    event: ERC721ApprovalForAllEvent
) -> Proto::ApprovalForAll {
    Proto::ApprovalForAll {
        owner: Hex(&event.owner).to_string(),
        operator: Hex(&event.operator).to_string(),
        approved: event.approved,
        contract_address: Hex(&log.address).to_string(),
        trx_hash: Hex(hash).to_string(),
        ordinal: log.block_index as u64,
    }
}

#[substreams::handlers::map]
pub fn graph_out(
    transfers: Proto::Transfers,
    approvals: Proto::Approvals,
    approvalforall: Proto::ApprovalForAlls
) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();
    graph::transfers_to_entities_changes(&mut entity_changes, transfers);
    graph::approvals_to_entities_changes(&mut entity_changes, approvals);
    graph::approvals_for_all_to_entities_changes(&mut entity_changes, approvalforall);
    Ok(entity_changes)
}