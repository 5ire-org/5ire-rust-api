use std::sync::mpsc::channel;

use clap::{load_yaml, App};
use fire_api_client::{rpc::WsRpcClient, Api};
use sp_core::sr25519;
use sp_runtime::{generic::Header, traits::BlakeTwo256};

fn main() {
    env_logger::init();
    let url = get_node_url_from_cli();

    let client = WsRpcClient::new(&url);
    let api = Api::<sr25519::Pair, _>::new(client).unwrap();

    println!("Subscribe to events");
    let (events_in, events_out) = channel();

    api.subscribe_new_heads(events_in).unwrap();

    loop {
        let header: Header<u64, BlakeTwo256> =
            serde_json::from_str(&events_out.recv().unwrap()).unwrap();
        println!("\n=========================");
        println!("NEW BLOCK");
        println!("=========================");
        println!("{:?}", header);
    }
}

pub fn get_node_url_from_cli() -> String {
    let yml = load_yaml!("../../src/examples/cli.yml");
    let matches = App::from_yaml(yml).get_matches();

    let node_ip = matches.value_of("node-server").unwrap_or("ws://127.0.0.1");
    let node_port = matches.value_of("node-port").unwrap_or("9944");
    let url = format!("{}:{}", node_ip, node_port);
    println!("Interacting with node on {}", url);
    url
}
