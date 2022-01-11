use fire_api_client::{rpc::WsRpcClient, Api, Metadata};

use sp_core::sr25519;

fn main() {
    // instantiate an Api that connects to the given address
    let url = "ws://110.238.108.183:9947";
    //let url = "ws://127.0.0.1:9944";
    let client = WsRpcClient::new(url);
    // if no signer is set in the whole program, we need to give to Api a specific type instead of an associated type
    // as during compilation the type needs to be defined.
    let api = Api::<sr25519::Pair, WsRpcClient>::new(client).unwrap();

    let meta = api.get_metadata().unwrap();
    println!("Metadata:\n {}", Metadata::pretty_format(&meta).unwrap());
}
