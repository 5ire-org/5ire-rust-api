use fire_api_client::{rpc::WsRpcClient, Api, AccountId, Metadata, XtStatus};
use sp_runtime::MultiAddress;
use std::str::FromStr;

use sp_core::{
    Pair as TraitPair,
    sr25519::{Pair, Public}
};
use std::io::{self, Write};


/**
 * View accounts
 * View block details
 * Transfer
 * Settings
 *  - Authentication
 *  - Server settings
 */

struct App {
    api: Option<Api<Pair, WsRpcClient>>,
    authenticated: bool
}

fn read_num_input() -> Result<u32, &'static str> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Read input failed.");
    buffer.trim().parse::<u32>().map_err(|_| "Please enter a valid numeric input.")
}

fn read_str_input() -> Result<String, &'static str> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Read input failed.");
    Ok(buffer.trim().to_string())
}

impl App {
    pub fn new() -> Self {
        App { api: None, authenticated: false }
    }

    pub fn start(mut self) {
        loop {
            if let Err(reason) = self.prompt_menu() {
                println!("{}", reason);
            }
        }
    }
    
    fn prompt_menu(&mut self) -> Result<(), &'static str> {
        println!("\n1. View account");
        println!("2. Transfer");
        println!("3. Settings");
        print!("=> ");
        io::stdout().flush().unwrap();

        match read_num_input()? {
            1 => self.view_account(),
            2 => self.transfer(),
            3 => self.settings(),
            _ => Err("Invalid input"),
        }
    }

    fn view_account(&self) -> Result<(), &'static str> {
        Ok(())
    }

    fn transfer(&mut self) -> Result<(), &'static str> {
        if !self.authenticated {
            return Err("Please authenticate first.");
        }

        if self.api.is_none() {
            return Err("Please configure the node first.");
        }

        let api = self.api.as_mut().unwrap();

        let to: AccountId = Public::from_str(&read_str_input()?)
            .unwrap()
            .into();

        // generate extrinsic
        let xt = api.balance_transfer_in_unit(MultiAddress::Id(to.clone()), 1);

        // send and watch extrinsic until finalized
        let tx_hash = api
            .send_extrinsic(xt.hex_encode(), XtStatus::InBlock)
            .unwrap();

        println!("Transaction is included in: {}", tx_hash.unwrap());

        Ok(())
    }

    fn settings(&mut self) -> Result<(), &'static str> {
        println!("1. Configure server");
        println!("2. Authenticate");

        match read_num_input()? {
            1 => self.configure_server(),
            2 => self.authenticate(),
            _ => Err("Invalid input"),
        }
    }

    fn configure_server(&mut self) -> Result<(), &'static str> {
        print!("Server address (ip:port): ");
        io::stdout().flush().unwrap();
        
        let client = WsRpcClient::new(read_str_input()?.as_str());
        self.api = Some(Api::new(client).unwrap());

        Ok(())
    }

    fn authenticate(&mut self) -> Result<(), &'static str> {
        print!("Mnemonic: ");
        io::stdout().flush().unwrap();

        if let Some(api) = self.api.take() {
            let (from, _) = Pair::from_phrase(&read_str_input()?, None).unwrap();
            self.api = Some(api.set_signer(from.clone()));
            self.authenticated = true;
        }

        Ok(())
    }
}

fn main() {

    let app = App::new();
    app.start();
}
