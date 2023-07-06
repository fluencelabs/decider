use crate::web3::curl_transport::CurlTransport;
use ethcontract::common::abiext::EventExt;
use ethcontract::prelude::*;

// ethcontract::contract!(
//     "/Users/folex/Development/fluencelabs/deal/artifacts/contracts/global/Matcher.sol/Matcher.json"
// );

include!(concat!(env!("OUT_DIR"), "/Matcher.rs"));

/*
Account #19: 0x6f10E8209296Ea9e556f80b0Ff545D8175F271d0 (10000 ETH)
Private Key: 0xfbd9e512cc1b62db1ca689737c110afa9a3799e1bc04bf12c1c34ac39e0e2dd5
*/

#[marine]
pub fn test() {
    let raw = Matcher::raw_contract();
    println!("raw: {:?}", raw);
    let matched = raw.abi.events.get("Matched").unwrap();

    let address = "0x42e59295F72a5B31884d8532396C0D89732c8e84"
        .parse()
        .unwrap();
    let provider = "0x6f10E8209296Ea9e556f80b0Ff545D8175F271d0"
        .parse()
        .unwrap();
    let transport =
        CurlTransport::new("http://127.0.0.1:8545").expect("error in CurlTransport::new");
    let web3 = Web3::new(transport);
    let instance = Matcher::at(&web3, address);
    let mut events = instance
        .events()
        .matched()
        .from_block("0x0".parse().unwrap())
        .compute_provider(Topic::This(provider))
        .stream();
}
