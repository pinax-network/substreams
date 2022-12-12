use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct Transfer {
    pub from: String,
    pub to: String,
    pub quantity: String,
    pub memo: String,
}

impl std::str::FromStr for Transfer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match serde_json::from_str(&s) {
            Ok(n) => n,
            Err(_) => return Err("Failed to deserialize Transfer params".to_string()),
        })
    }
}
