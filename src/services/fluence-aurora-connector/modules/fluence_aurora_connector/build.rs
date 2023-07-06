use ethcontract_generate::loaders::TruffleLoader;
use ethcontract_generate::ContractBuilder;
use std::path::Path;

fn main() {
    // Prepare filesystem paths.
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest = std::path::Path::new(&out_dir).join("Matcher.rs");

    let path = Path::new("/Users/folex/Development/fluencelabs/deal/artifacts/contracts/global/Matcher.sol/Matcher.json");

    assert!(path.exists(), "WTF");

    // Load a contract.
    let contract = TruffleLoader::new().load_contract_from_file(path).unwrap();

    // Generate bindings for it.
    ContractBuilder::new()
        .generate(&contract)
        .unwrap()
        .write_to_file(dest)
        .unwrap();
}
