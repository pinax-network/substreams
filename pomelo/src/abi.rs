use substreams_antelope::ActionTrace;
// use antelope::Asset;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SetGrant {
    pub author_id: String,
    pub project_id: String,
    pub funding_account: String,
    pub accepted_tokens: Vec<String>,
}


pub fn parse_setgrant(action_trace: &ActionTrace) -> Option<SetGrant> {
    let action = action_trace.action.as_ref()?;
    if action.name != "setgrant" { return None; }
    let data: SetGrant = serde_json::from_str(&action.json_data).unwrap();
    Some(data)
}
