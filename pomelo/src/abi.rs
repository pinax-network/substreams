// use antelope::Asset;
use serde::{Deserialize, Serialize};

macro_rules! impl_from_str {
    ($type:ty) => {
        impl From<&str> for $type {
            #[inline]
            #[must_use]
            fn from(str: &str) -> Self {
                serde_json::from_str(str).unwrap()
            }
        }
    };
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GrantsRow {
    pub id: String,
    pub r#type: String,
    pub author_user_id: String,
    pub funding_account: String,
    pub accepted_tokens: Vec<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}
impl_from_str!(GrantsRow);



#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ExtendedQuantity {
    pub quantity: String,
    pub contract: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct TransfersRow {
    pub transfer_id: u64,
    pub from: String,
    pub to: String,
    pub ext_quantity: ExtendedQuantity,
    pub fee: String,
    pub memo: String,
    pub user_id: String,
    pub season_id: u16,
    pub round_id: u16,
    pub project_type: String,
    pub project_id: String,
    pub value: String,
    pub trx_id: String,
    pub created_at: String,
}
impl_from_str!(TransfersRow);

