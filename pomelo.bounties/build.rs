
fn main() {
    substreams_antelope::Abigen::new("Contract", "abi/pomelo.bounties.abi.json")
        .expect("failed to load abi")
        .generate()
        .expect("failed to generate contract")
        .write_to_file("src/abi/contract.rs")
        .expect("failed to write contract");
}