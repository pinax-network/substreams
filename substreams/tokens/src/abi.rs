use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTokenAction {
    pub issuer: String,
    #[serde(rename = "maximum_supply")]
    pub maximum_supply: String,
}
