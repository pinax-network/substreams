use serde::{Deserialize, Serialize};
use serde_json::{Result};
use eosio::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Transfer {
    pub from: String,
    pub to: String,
    pub quantity: String,
    pub memo: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Issue {
    pub to: String,
    pub quantity: String,
    pub memo: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Create {
    pub issuer: String,
    pub maximum_supply: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Close {
    pub owner: String,
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Open {
    pub owner: String,
    pub symbol: String,
    pub ram_payer: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Retire {
    pub quantity: String,
    pub memo: String,
}

pub fn parse_quantity(quantity: &String) -> Asset {
    quantity.parse::<Asset>().unwrap()
}

pub fn is_transfer(json_data: &str) -> Result<()> {
    let _data: Transfer = serde_json::from_str(json_data)?;
    Ok(())
}

pub fn parse_transfer(json_data: &str) -> Transfer {
    serde_json::from_str(json_data).unwrap()
}

// pub fn is_issue(json_data: &str) -> Result<()> {
//     let _data: Issue = serde_json::from_str(json_data)?;
//     Ok(())
// }

// pub fn is_create(json_data: &str) -> Result<()> {
//     let _data: Create = serde_json::from_str(json_data)?;
//     Ok(())
// }

// pub fn is_close(json_data: &str) -> Result<()> {
//     let _data: Close = serde_json::from_str(json_data)?;
//     Ok(())
// }

// pub fn is_open(json_data: &str) -> Result<()> {
//     let _data: Open = serde_json::from_str(json_data)?;
//     Ok(())
// }

// pub fn is_retire(json_data: &str) -> Result<()> {
//     let _data: Retire = serde_json::from_str(json_data)?;
//     Ok(())
// }