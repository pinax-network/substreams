// Generated by antelope-abi2rs 0.3.0 - eosio::abi/1.2

use serde::{Deserialize, Serialize};

type Asset = String;
type Name = String;
type Bool = bool;
type Bytes = String;
type TimePointSec = String;
type Uint32 = u32;
type Varuint32 = u32;
type Uint64 = u64;


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


#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Account {
    pub id: Uint64,
    pub eth_address: Bytes,
    pub nonce: Uint64,
    pub balance: Bytes,
    pub code_id: Option<Uint64>,
}
impl_try_from_str!(Account);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AccountCode {
    pub id: Uint64,
    pub ref_count: Uint32,
    pub code: Bytes,
    pub code_hash: Bytes,
}
impl_try_from_str!(AccountCode);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Addegress {
    pub accounts: Vec<Name>,
}
impl_try_from_str!(Addegress);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AllowedEgressAccount {
    pub account: Name,
}
impl_try_from_str!(AllowedEgressAccount);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Balance {
    pub owner: Name,
    pub balance: BalanceWithDust,
}
impl_try_from_str!(Balance);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct BalanceWithDust {
    pub balance: Asset,
    pub dust: Uint64,
}
impl_try_from_str!(BalanceWithDust);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Close {
    pub owner: Name,
}
impl_try_from_str!(Close);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub version: Varuint32,
    pub chainid: Uint64,
    pub genesis_time: TimePointSec,
    pub ingress_bridge_fee: Asset,
    pub gas_price: Uint64,
    pub miner_cut: Uint32,
    pub status: Uint32,
}
impl_try_from_str!(Config);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct FeeParameters {
    pub gas_price: Option<Uint64>,
    pub miner_cut: Option<Uint32>,
    pub ingress_bridge_fee: Option<Asset>,
}
impl_try_from_str!(FeeParameters);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Freeze {
    pub value: Bool,
}
impl_try_from_str!(Freeze);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Gc {
    pub max: Uint32,
}
impl_try_from_str!(Gc);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Gcstore {
    pub id: Uint64,
    pub storage_id: Uint64,
}
impl_try_from_str!(Gcstore);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Init {
    pub chainid: Uint64,
    pub fee_params: FeeParameters,
}
impl_try_from_str!(Init);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Nextnonce {
    pub owner: Name,
    pub next_nonce: Uint64,
}
impl_try_from_str!(Nextnonce);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Open {
    pub owner: Name,
}
impl_try_from_str!(Open);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Pushtx {
    pub miner: Name,
    pub rlptx: Bytes,
}
impl_try_from_str!(Pushtx);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Removeegress {
    pub accounts: Vec<Name>,
}
impl_try_from_str!(Removeegress);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Setfeeparams {
    pub fee_params: FeeParameters,
}
impl_try_from_str!(Setfeeparams);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Storage {
    pub id: Uint64,
    pub key: Bytes,
    pub value: Bytes,
}
impl_try_from_str!(Storage);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Withdraw {
    pub owner: Name,
    pub quantity: Asset,
}
impl_try_from_str!(Withdraw);
