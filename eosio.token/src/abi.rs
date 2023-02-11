use serde::{Deserialize, Serialize};
use serde_json::{Result};
use eosio::{Name, Asset};

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

pub fn is_transfer(json_data: &str) -> bool {
    let json: Result<Transfer> = serde_json::from_str(json_data);
    match json {
        Ok(data) => {
            let from = data.from.parse::<Name>();
            let to = data.to.parse::<Name>();
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Balance {
    pub balance: String,
}

pub fn parse_balance(data_json: &str) -> Option<Balance> {
    match serde_json::from_str(data_json) {
        Ok(data) => Some(data),
        Err(_) => None,
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CurrencyStats {
    pub issuer: String,
    pub max_supply: String,
    pub supply: String,
}

pub fn parse_currency_stats(data_json: &str) -> Option<CurrencyStats> {
    match serde_json::from_str(data_json) {
        Ok(data) => Some(data),
        Err(_) => None,
    }
}


// pub fn is_issue(json_data: &str) -> bool {
//     let json: Result<Issue> = serde_json::from_str(json_data);
//     match json {
//         Ok(data) => {
//             data.to.parse::<Name>().is_ok()
//             && data.quantity.parse::<Asset>().is_ok()
//         },
//         Err(_) => false,
//     }
// }

// pub fn is_create(json_data: &str) -> bool {
//     let json: Result<Create> = serde_json::from_str(json_data);
//     match json {
//         Ok(data) => {
//             data.issuer.parse::<Name>().is_ok()
//             && data.maximum_supply.parse::<Asset>().is_ok()
//         },
//         Err(_) => false,
//     }
// }

// pub fn is_close(json_data: &str) -> bool {
//     let json: Result<Close> = serde_json::from_str(json_data);
//     match json {
//         Ok(data) => {
//             data.owner.parse::<Name>().is_ok()
//             && is_symbol(&data.symbol)
//         },
//         Err(_) => false,
//     }
// }

// pub fn is_symbol(symbol: &str) -> bool {
//     let parts: Vec<&str> = symbol.split(",").collect();
//     let precision = parts[1].parse::<u8>();
//     let symcode = parts[0].parse::<SymbolCode>();

//     if parts.len() != 2 { return false; }
//     if precision.is_err() { return false; }
//     if symcode.is_err() { return false; }
//     if symcode.unwrap().as_u64() == 0 { return false; }
//     return true;
// }

// pub fn is_open(json_data: &str) -> bool {
//     let json: Result<Open> = serde_json::from_str(json_data);
//     match json {
//         Ok(data) => {
//             data.owner.parse::<Name>().is_ok()
//             && is_symbol(&data.symbol)
//             && data.ram_payer.parse::<Name>().is_ok()
//         },
//         Err(_) => false,
//     }
// }

// pub fn is_retire(json_data: &str) -> bool {
//     let json: Result<Retire> = serde_json::from_str(json_data);
//     match json {
//         Ok(data) => {
//             data.quantity.parse::<Asset>().is_ok()
//         },
//         Err(_) => false,
//     }
// }

// pub fn is_accounts(_new_data: &Vec<u8>) -> bool {
//     // TO-DO
//     return true;
// }

// pub fn is_stat(_new_data: &Vec<u8>) -> bool {
//     // TO-DO
//     return true;
// }

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

// #[test]
// fn test_issue() {
//     let json_data = r#"
//         {
//             "to": "toaccount",
//             "quantity": "1.0000 EOS",
//             "memo": "memo"
//         }
//         "#;
//     let result = is_issue(&json_data.to_string());
//     assert_eq!(result, true);
// }

// pub fn string_to_symcode(str: &str) -> SymbolCode {
//     let name = str.parse::<Name>();
//     if name.is_err() { return SymbolCode::new(0); }
//     return SymbolCode::new(name.unwrap().as_u64());
// }

// #[test]
// fn test_symbol_code() {
//     assert_eq!(SymbolCode::new(5459781).to_string(), "EOS");
//     assert_eq!(Name::from_str("........ehbo5").unwrap().as_u64(), 5459781);
// }