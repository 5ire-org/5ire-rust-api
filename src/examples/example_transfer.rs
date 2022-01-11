///! Very simple example that shows how to use a predefined extrinsic from the extrinsic module
use clap::{load_yaml, App};
use fire_api_client::{rpc::WsRpcClient, AccountId, Api, XtStatus};
use sp_core::crypto::Pair as TraitPair;
use sp_core::sr25519::{Pair, Public};
use sp_runtime::MultiAddress;
use std::str::FromStr;

struct Config {
    url: String,
    signer_mnemonic: String,
    destination_address: String,
}

fn main() {
    env_logger::init();
    let config = get_config_from_cli();

    // initialize api and set the signer (sender) that is used to sign the extrinsics
    let (from, _) = Pair::from_phrase(&config.signer_mnemonic, None).unwrap();

    let client = WsRpcClient::new(&config.url);
    let api = Api::new(client)
        .map(|api| api.set_signer(from.clone()).set_unit(u128::pow(10, 18)))
        .unwrap();

    let to: AccountId = Public::from_str(&config.destination_address)
        .unwrap()
        .into();

    match api.get_account_data(&to).unwrap() {
        Some(to) => println!("[+] Destination address' free balance is {}\n", to.free),
        None => println!("[+] Destination address' free balance is is 0\n"),
    }
    // generate extrinsic
    let xt = api.balance_transfer_in_unit(MultiAddress::Id(to.clone()), 1);

    println!("Transferring from {}, to {}\n", from.public(), to);

    println!("[+] Composed extrinsic: {:?}\n", xt);

    // send and watch extrinsic until finalized
    let tx_hash = api
        .send_extrinsic(xt.hex_encode(), XtStatus::InBlock)
        .unwrap();
    println!("[+] Transaction got included. Hash: {:?}\n", tx_hash);

    // verify that Bob's free Balance increased
    let to = api.get_account_data(&to).unwrap().unwrap();
    println!("[+] Destination address' free balance is now {}\n", to.free);
}

fn get_config_from_cli() -> Config {
    let yml = load_yaml!("../../src/examples/transfer-cli.yml");
    let matches = App::from_yaml(yml).get_matches();

    let node_ip = matches.value_of("node-server").unwrap_or("ws://127.0.0.1");
    let node_port = matches.value_of("node-port").unwrap_or("9944");
    let url = format!("{}:{}", node_ip, node_port);

    println!("Interacting with node on {}\n", url);

    Config {
        url,
        signer_mnemonic: matches.value_of("from-mnemonic").unwrap().to_string(),
        destination_address: matches.value_of("to-address").unwrap().to_string(),
    }
}
