///! Shows how to connect and system informations

#[macro_use]
extern crate clap;

use clap::App;

use sp_core::sr25519;

use fire_api_client::rpc::WsRpcClient;
use fire_api_client::Api;

fn main() {
    env_logger::init();
    let url = get_node_url_from_cli();

    let client = WsRpcClient::new(&url);
    let api = Api::<sr25519::Pair, _>::new(client).unwrap();

    println!(
        "You are connected to chain {} using {} v{}.",
        api.system_chain().unwrap(),
        api.system_name().unwrap(),
        api.system_version().unwrap()
    );
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
