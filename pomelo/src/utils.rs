use substreams_antelope::pb::db_op;
use substreams_entity_change::pb::entity::entity_change;

pub fn from_dbop_to_entityop(op: &db_op::Operation) -> entity_change::Operation {
    match op {
        db_op::Operation::Insert => entity_change::Operation::Create,
        db_op::Operation::Update => entity_change::Operation::Update,
        db_op::Operation::Remove => entity_change::Operation::Delete,
        db_op::Operation::Unknown => entity_change::Operation::Unspecified,
    }
}
