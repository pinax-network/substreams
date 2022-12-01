use crate::asset::Symbol;


// TODO: -> Result<Symbol, Err>
pub fn extract_symbol(value_str: String) -> Symbol {
    let parts1: Vec<&str> = value_str.split(" ").collect();
    let symcode = String::from(parts1[1]);
    let parts2: Vec<&str> = parts1[0].split(".").collect();
    let precision = parts2[1].len() as u8;

    Symbol{ symcode, precision}
}