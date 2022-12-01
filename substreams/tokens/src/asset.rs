pub struct Symbol {
    pub symcode: String,
    pub precision: u8
}

#[allow(dead_code)]
pub struct ExtendedSymbol {
    symbol: Symbol,
    contract: String
}

#[allow(dead_code)]
pub struct Asset {
    amount: i64,
    symbol: Symbol
}

