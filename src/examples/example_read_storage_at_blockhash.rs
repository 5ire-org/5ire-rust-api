///! Very simple example that shows how to get some simple storage values.
use clap::{load_yaml, App};

use fire_api_client::{rpc::WsRpcClient, AccountId, AccountInfo, Api, Header};
use sp_application_crypto::Ss58Codec;
use sp_core::sr25519;
use sp_runtime::{generic, traits::BlakeTwo256};

fn main() {
    env_logger::init();
    let (url, address) = get_args_from_cli();

    let client = WsRpcClient::new(&url);
    let api: Api<sr25519::Pair, _> = Api::new(client).unwrap();

    let last_header: generic::Header<u64, BlakeTwo256> = api.get_header(None).unwrap().unwrap();

    println!("Last header hash: {}", last_header.hash());
    println!("Last header parent hash: {}", last_header.parent_hash());

    let account_id: AccountId = sp_core::sr25519::Public::from_ss58check(&address)
        .unwrap()
        .into();

    let account_info: AccountInfo = api
        .get_storage_map(
            "System",
            "Account",
            &account_id,
            Some(last_header.parent_hash),
        )
        .unwrap()
        .unwrap();

    println!(
        "Account's balance at {} was {}",
        last_header.parent_hash(),
        account_info.data.free
    );

    let account_data = api.get_account_data(&account_id).unwrap().unwrap();
    println!("Account's current balance is {}", account_data.free);
}

pub fn get_args_from_cli() -> (String, String) {
    let yml = load_yaml!("../../src/examples/cli.yml");
    let matches = App::from_yaml(yml).get_matches();

    let node_ip = matches.value_of("node-server").unwrap_or("ws://127.0.0.1");
    let node_port = matches.value_of("node-port").unwrap_or("9944");
    let account = matches
        .value_of("account")
        .unwrap_or("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");
    let url = format!("{}:{}", node_ip, node_port);
    println!("Interacting with node on {}\n", url);
    (url, account.to_string())
}
