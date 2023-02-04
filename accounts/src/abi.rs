use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]

pub struct PermissionLevel {
    pub account: String,
    pub permission: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PermissionLevelWeight {
    pub permission: PermissionLevel,
    pub weight: u16,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyWeight {
    pub key: String,
    pub weight: u16,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WaitWeight {
    pub wait_sec: u32,
    pub weight: u16,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Authority {
    pub threshold: u32,
    pub keys: Vec<KeyWeight>,
    pub accounts: Vec<PermissionLevelWeight>,
    pub waits: Vec<WaitWeight>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct NewAccount {
    pub creator: String,
    pub name: String,
    pub owner: Authority,
    pub active: Authority,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct BuyRamBytes {
    pub payer: String,
    pub receiver: String,
    pub bytes: u32,
}

impl std::str::FromStr for NewAccount {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match serde_json::from_str(&s) {
            Ok(n) => n,
            Err(_) => return Err("Failed to deserialize NewAccount params".to_string()),
        })
    }
}

impl std::str::FromStr for BuyRamBytes {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match serde_json::from_str(&s) {
            Ok(n) => n,
            Err(_) => return Err("Failed to deserialize BuyRamBytes params".to_string()),
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_for_buy_ram_bytes_success() {
        let input_string = r#"{"payer": "payer", "receiver": "receiver", "bytes": 123456}"#;
        let expected = BuyRamBytes {
            payer: "payer".to_string(),
            receiver: "receiver".to_string(),
            bytes: 123456,
        };

        let result = input_string.parse::<BuyRamBytes>();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_from_str_for_buy_ram_bytes_failure() {
        let input_string = "not a valid json string";

        let result = input_string.parse::<BuyRamBytes>();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Failed to deserialize BuyRamBytes params");
    }

    #[test]
    fn test_from_str_for_new_account() {
        let valid_json_string = r#"{
            "creator": "creator",
            "name": "newaccount",
            "owner": {
                "threshold": 1,
                "keys": [
                    {
                        "key": "EOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV",
                        "weight": 1
                    }
                ],
                "accounts": [],
                "waits": []
            },
            "active": {
                "threshold": 1,
                "keys": [
                    {
                        "key": "EOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV",
                        "weight": 1
                    }
                ],
                "accounts": [],
                "waits": []
            }
        }"#;
        let expected_result = NewAccount {
            creator: "creator".to_string(),
            name: "newaccount".to_string(),
            owner: Authority {
                threshold: 1,
                keys: vec![KeyWeight {
                    key: "EOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV".to_string(),
                    weight: 1,
                }],
                accounts: vec![],
                waits: vec![],
            },
            active: Authority {
                threshold: 1,
                keys: vec![KeyWeight {
                    key: "EOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV".to_string(),
                    weight: 1,
                }],
                accounts: vec![],
                waits: vec![],
            },
        };
        let result = valid_json_string.parse::<NewAccount>().unwrap();
        assert_eq!(expected_result, result);

    }

    #[test]
    fn test_from_str_for_new_account_failure() {

        let invalid_json_string = "{}";
        let result = invalid_json_string.parse::<NewAccount>();
        assert!(result.is_err());

        let invalid_json_string = r#"{"creator": "creator"}"#;
        let result = invalid_json_string.parse::<NewAccount>();
        assert!(result.is_err());

        let invalid_json_string = "just some text";
        let result = invalid_json_string.parse::<NewAccount>();
        assert!(result.is_err());
    }
}
