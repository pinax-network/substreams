use std::str::FromStr;
use crate::symbol::Symbol;

#[derive(Debug)]
pub struct Asset {
    pub amount: i64,
    pub symbol: Symbol,
}

impl FromStr for Asset {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts1: Vec<&str> = s.split(" ").collect();
        if parts1.len() != 2 {
            return Err("Invalid asset string".to_string());
        }
        let symcode = parts1[1].to_string();
        let parts2: Vec<&str> = parts1[0].split(".").collect();
        let precision = if parts2.len() == 1 { 0_u8 } else { parts2[1].len() as u8 };
        let amount1 = match parts2[0].parse::<i64>() {
            Ok(n) => n,
            Err(_) => return Err("Invalid amount".to_string()),
        };
        let amount2 = if precision == 0 { 0 } else {
            match parts2[1].parse::<i64>() {
                Ok(n) => n,
                Err(_) => return Err("Invalid amount".to_string()),
            }
        };
        let amount = 10_i64.pow(precision as u32) * amount1 + amount2;

        Ok(Asset{ amount, symbol: Symbol{ symcode, precision}})
    }
}

#[test]
fn parse_success_1() {
    let asset = "1.0000 EOS".parse::<Asset>().unwrap();
    assert_eq!(10000, asset.amount);
    assert_eq!("EOS", asset.symbol.symcode);
    assert_eq!(4, asset.symbol.precision);
}

#[test]
fn parse_success_2() {
    let asset = "10000 SYS".parse::<Asset>().unwrap();
    assert_eq!(10000, asset.amount);
    assert_eq!("SYS", asset.symbol.symcode);
    assert_eq!(0, asset.symbol.precision);
}

#[test]
fn parse_invalid_asset_1() {
    assert_eq!(true, "1.0000EOS".parse::<Asset>().is_err());
}


#[test]
fn parse_invalid_asset_2() {
    assert_eq!(true, "10000EOS".parse::<Asset>().is_err());
}

