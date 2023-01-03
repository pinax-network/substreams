// ------------------------------------------------
//      store_accounts
// ------------------------------------------------
pub fn store_accounts(owner: &String, contract: &String, symcode: &String) -> String {
    format!("accounts:{}:{}:{}", owner, contract, symcode )
}

// ------------------------------------------------
//      store_stat
// ------------------------------------------------
pub fn store_stat(contract: &String, symcode: &String) -> String {
    format!("stat:{}:{}", contract, symcode)
}
