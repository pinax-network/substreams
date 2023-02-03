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
