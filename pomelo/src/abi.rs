use serde::{Deserialize, Serialize};

type Name = String;
type Double = String;
type Timestamp = String;
type Checksum = String;
type SymbolCode = String;
type Asset = String;
type Uint64 = u64;
type Uint16 = u16;


#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ExtendedQuantity {
    pub quantity: Name,
    pub contract: Name,
}


macro_rules! impl_try_from_str {
    ($type:ty) => {
        impl TryFrom<&str> for $type {
            type Error = serde_json::Error;
            #[inline]
            fn try_from(str: &str) -> Result<Self, Self::Error> {
                serde_json::from_str(str)
            }
        }
    };
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GrantsRow {
    pub id: Name,
    pub r#type: Name,
    pub author_user_id: Name,
    pub funding_account: Name,
    pub accepted_tokens: Vec<SymbolCode>,
    pub status: Name,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}
impl_try_from_str!(GrantsRow);

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct TransfersRow {
    pub transfer_id: Uint64,
    pub from: Name,
    pub to: Name,
    pub ext_quantity: ExtendedQuantity,
    pub fee: Asset,
    pub memo: String,
    pub user_id: Name,
    pub season_id: Uint16,
    pub round_id: Uint16,
    pub project_type: Name,
    pub project_id: Name,
    pub value: Double,
    pub trx_id: Checksum,
    pub created_at: Timestamp,
}
impl_try_from_str!(TransfersRow);

