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

pub fn is_transfer(json_data: &str) -> bool {
    let json: Result<Transfer> = serde_json::from_str(json_data);
    match json {
        Ok(data) => {
            let from = data.from.parse::<AccountName>();
            let to = data.to.parse::<AccountName>();
            let quantity = data.quantity.parse::<Asset>();

            from.is_ok()
            && to.is_ok()
            && quantity.is_ok()
            && from.unwrap().as_u64() != 0
            && to.unwrap().as_u64() != 0
            
        },
        Err(_) => false,
    }
}

pub fn parse_transfer(json_data: &str) -> Transfer {
    serde_json::from_str(json_data).unwrap()
}

pub fn is_issue(json_data: &str) -> bool {
    let json: Result<Issue> = serde_json::from_str(json_data);
    match json {
        Ok(data) => {
            data.to.parse::<AccountName>().is_ok()
            && data.quantity.parse::<Asset>().is_ok()
        },
        Err(_) => false,
    }
}

pub fn is_create(json_data: &str) -> bool {
    let json: Result<Create> = serde_json::from_str(json_data);
    match json {
        Ok(data) => {
            data.issuer.parse::<AccountName>().is_ok()
            && data.maximum_supply.parse::<Asset>().is_ok()
        },
        Err(_) => false,
    }
}

pub fn is_close(json_data: &str) -> bool {
    let json: Result<Close> = serde_json::from_str(json_data);
    match json {
        Ok(data) => {
            data.owner.parse::<AccountName>().is_ok()
            // && data.symbol.parse::<Symbol>().is_ok()
        },
        Err(_) => false,
    }
}

pub fn is_open(json_data: &str) -> bool {
    let json: Result<Open> = serde_json::from_str(json_data);
    match json {
        Ok(data) => {
            data.owner.parse::<AccountName>().is_ok()
            && data.ram_payer.parse::<AccountName>().is_ok()
            // && data.symbol.parse::<Symbol>().is_ok()
        },
        Err(_) => false,
    }
}


pub fn is_retire(json_data: &str) -> bool {
    let json: Result<Retire> = serde_json::from_str(json_data);
    match json {
        Ok(data) => {
            data.quantity.parse::<Asset>().is_ok()
        },
        Err(_) => false,
    }
}

#[test]
fn test_transfer() {
    // valid
    assert_eq!(is_transfer(&r#"
        {
            "from": "myaccount",
            "to": "toaccount",
            "quantity": "1.0000 EOS",
            "memo": "memo"
        }
    "#.to_string()), true);

    // valid (empty memo)
    assert_eq!(is_transfer(&r#"
        {
            "from": "myaccount",
            "to": "toaccount",
            "quantity": "1.0000 EOS",
            "memo": ""
        }
    "#.to_string()), true);

    // missing to
    assert_eq!(is_transfer(&r#"
        {
            "from": "myaccount",
            "to": "",
            "quantity": "1.0000 EOS",
            "memo": "memo"
        }
    "#.to_string()), false);

    // invalid quantity symbol
    assert_eq!(is_transfer(&r#"
        {
            "from": "myaccount",
            "to": "toaccount",
            "quantity": "1.0000",
            "memo": "memo"
        }
    "#.to_string()), false);
}

#[test]
fn test_issue() {
    let json_data = r#"
        {
            "to": "toaccount",
            "quantity": "1.0000 EOS",
            "memo": "memo"
        }
        "#;
    let result = is_issue(&json_data.to_string());
    assert_eq!(result, true);
}