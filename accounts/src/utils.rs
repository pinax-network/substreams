
use crate::accounts;
use crate::abi;

impl From<abi::Authority> for accounts::Authority {
    fn from(authority: abi::Authority) -> Self {
        accounts::Authority {
            threshold: authority.threshold,
            keys: authority.keys.iter().map(|kw| accounts::KeyWeight {
                public_key: kw.key.clone(),
                weight: kw.weight as u32,
            }).collect(),
            accounts: authority.accounts.iter().map(|plw| accounts::PermissionLevelWeight {
                permission: Some(accounts::PermissionLevel {
                    actor: plw.permission.account.clone(),
                    permission: plw.permission.permission.clone(),
                }),
                weight: plw.weight as u32,
            }).collect(),
            waits: authority.waits.iter().map(|ww| accounts::WaitWeight {
                wait_sec: ww.wait_sec,
                weight: ww.weight as u32,
            }).collect(),
        }
    }
}
