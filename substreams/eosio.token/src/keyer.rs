// ------------------------------------------------
//      store_accounts
// ------------------------------------------------
pub fn store_accounts(symcode: &String, contract: &String, owner: &String) -> String {
    format!("accounts:{}:{}:{}", symcode, contract, owner )
}

// ------------------------------------------------
//      store_stat
// ------------------------------------------------
pub fn store_stat(symcode: &String, contract: &String) -> String {
    format!("stat:{}:{}", symcode, contract)
}
