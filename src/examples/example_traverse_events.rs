///! Traverse events.
use clap::{load_yaml, App};

use fire_api_client::rpc::WsRpcClient;
use fire_api_client::Api;
use fire_api_client::Metadata;
use sp_core::sr25519;

fn main() {
    env_logger::init();
    let url = get_node_url_from_cli();

    let client = WsRpcClient::new(&url);
    let api = Api::<sr25519::Pair, _>::new(client).unwrap();

    let meta = Metadata::try_from(api.get_metadata().unwrap()).unwrap();

    for pallet in meta.pallets() {
        for event in meta.events(pallet.index) {
            event.print();
        }
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
