///! Very simple example that shows how to get some simple storage values.
use clap::{load_yaml, App};

use fire_api_client::rpc::WsRpcClient;
use fire_api_client::Api;
use keyring::AccountKeyring;
use sp_runtime::AccountId32;

fn main() {
    env_logger::init();
    let url = get_node_url_from_cli();

    let client = WsRpcClient::new(&url);
    let mut api = Api::new(client).unwrap();

    // get Alice's AccountNonce with api.get_nonce()
    let signer = AccountKeyring::Alice.pair();
    api.signer = Some(signer);

    // You should specify the correct numeric type. If you specify a smaller
    // numeric type, you might get smaller and incorrect number. Otherwise,
    // you will get an error that says 'Not enough data to fill buffer'.
    let timestamp: u64 = api
        .get_storage_value("Timestamp", "Now", None)
        .unwrap()
        .unwrap();

    let validators: Vec<AccountId32> = api
        .get_storage_value("Session", "Validators", None)
        .unwrap()
        .unwrap();

    println!(
        "accountNonce({:?}) {}",
        AccountKeyring::Alice.public(),
        api.get_nonce().unwrap()
    );
    println!("last block timestamp {:?}", timestamp);

    for validator in &validators {
        let account_info = api.get_account_info(validator).unwrap().unwrap();

        println!("Validator:");
        println!("\tAccount Id: {}", validator);
        println!("\tBalance: {}", account_info.data.free);
        println!("\tNonce: {}", account_info.nonce);
    }
}

pub fn get_node_url_from_cli() -> String {
    let yml = load_yaml!("../../src/examples/cli.yml");
    let matches = App::from_yaml(yml).get_matches();

    let node_ip = matches.value_of("node-server").unwrap_or("ws://127.0.0.1");
    let node_port = matches.value_of("node-port").unwrap_or("9944");
    let url = format!("{}:{}", node_ip, node_port);
    println!("Interacting with node on {}\n", url);
    url
}
