// Generated by antelope-abi2rs 0.3.0 - eosio::abi/1.1

use serde::{Deserialize, Serialize};

type Asset = String;
type Name = String;
type Symbol = String;


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
    pub balance: Asset,
}
impl_try_from_str!(Account);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Close {
    pub owner: Name,
    pub symbol: Symbol,
}
impl_try_from_str!(Close);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Create {
    pub issuer: Name,
    pub maximum_supply: Asset,
}
impl_try_from_str!(Create);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CurrencyStats {
    pub supply: Asset,
    pub max_supply: Asset,
    pub issuer: Name,
}
impl_try_from_str!(CurrencyStats);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Issue {
    pub to: Name,
    pub quantity: Asset,
    pub memo: String,
}
impl_try_from_str!(Issue);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Open {
    pub owner: Name,
    pub symbol: Symbol,
    pub ram_payer: Name,
}
impl_try_from_str!(Open);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Retire {
    pub quantity: Asset,
    pub memo: String,
}
impl_try_from_str!(Retire);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Transfer {
    pub from: Name,
    pub to: Name,
    pub quantity: Asset,
    pub memo: String,
}
impl_try_from_str!(Transfer);
