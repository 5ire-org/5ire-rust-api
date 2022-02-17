use std::sync::mpsc::channel;

use clap::{load_yaml, App};
use codec::{Decode, Encode};
use fire_api_client::{
    compose_extrinsic, rpc::WsRpcClient, Api, Pair, UncheckedExtrinsicV4, XtStatus,
};
use keyring::AccountKeyring;
use rand::prelude::*;
use sp_core::sr25519;
use sp_runtime::AccountId32 as AccountId;
use std::collections::{BTreeSet, HashMap};
use std::vec::Vec;

// Look at the how the transfer event looks like in in the metadata
#[derive(Decode)]
struct AddEventArgs {
    who: AccountId,
    account_ids: BTreeSet<AccountId>,
}

#[derive(Decode)]
struct VoteEventArgs {
    voter: AccountId,
    candidate: AccountId,
    vote: Vec<u8>,
}

#[derive(Decode)]
struct VoteConfiguredEventArgs {
    who: AccountId,
    start_block: u32,
    end_block: u32,
    properties: Vec<Property>,
}

#[derive(Encode, Decode, Clone)]
struct Vote {
    esg_score: Vec<u8>,
}

#[derive(Encode, Decode, Debug, Clone, Copy)]
enum Property {
    GenderEquality,
    Environmental,
    IncomeEquality,
}

fn configure_vote(api: &Api<sr25519::Pair, WsRpcClient>) {
    // get StorageMap
    let block_height: u32 = api
        .get_storage_value("System", "Number", None)
        .unwrap()
        .unwrap();

    let vote_start = block_height + 2; // 30 secs.
    let vote_end = vote_start + 5; // 30 secs.

    let active_properties = vec![Property::GenderEquality];

    let xt = compose_extrinsic!(
        api.clone(),
        "GovernanceElection",
        "configure_vote",
        vote_start,
        vote_end,
        active_properties
    );

    let tx_hash = api
        .send_extrinsic(xt.hex_encode(), XtStatus::InBlock)
        .unwrap();

    println!(
        "[+] Configure Vote transaction got included. Hash: {:?}",
        tx_hash
    );

    let (events_in, events_out) = channel();

    api.subscribe_events(events_in).unwrap();
    let args: VoteConfiguredEventArgs = api
        .wait_for_event("GovernanceElection", "VoteConfigured", None, &events_out)
        .unwrap();

    println!("VoteConfigured Event: ");
    println!("\tWho: {:?}", args.who);
    println!("\tStart Block: {}", args.start_block);
    println!("\tEnd Block: {}", args.end_block);
    println!("\tProperties:");
    args.properties.iter().for_each(|property| {
        println!("\t\t- {:?}", property);
    });
}

fn add_participants(api: &Api<sr25519::Pair, WsRpcClient>) {
    let xt = compose_extrinsic!(
        api.clone(),
        "GovernanceElection",
        "add_participants",
        vec![
            AccountKeyring::Ferdie.to_account_id(),
            AccountKeyring::Charlie.to_account_id(),
            AccountKeyring::Dave.to_account_id(),
        ]
    );

    let tx_hash = api
        .send_extrinsic(xt.hex_encode(), XtStatus::InBlock)
        .unwrap();
    println!(
        "[+] Add Participants transaction got included. Hash: {:?}",
        tx_hash
    );

    let (events_in, events_out) = channel();

    api.subscribe_events(events_in).unwrap();
    let args: AddEventArgs = api
        .wait_for_event("GovernanceElection", "AddedParticipants", None, &events_out)
        .unwrap();

    println!("AddedParticipants Event: ");
    println!("\tWho: {:?}", args.who);
    println!("\tParticipants:");
    args.account_ids.iter().for_each(|participant| {
        println!(
            "\t\t- {:?}",
            AccountKeyring::from_account_id(participant).unwrap()
        );
    })
}

fn add_candidates(api: &Api<sr25519::Pair, WsRpcClient>) {
    // call Balances::transfer
    // the names are given as strings
    let xt: UncheckedExtrinsicV4<_> = compose_extrinsic!(
        api.clone(),
        "GovernanceElection",
        "add_candidates",
        vec![
            AccountKeyring::Bob.to_account_id(),
            AccountKeyring::Eve.to_account_id()
        ]
    );

    let tx_hash = api
        .send_extrinsic(xt.hex_encode(), XtStatus::InBlock)
        .unwrap();
    println!(
        "[+] Add Candidates transaction got included. Hash: {:?}",
        tx_hash
    );

    let (events_in, events_out) = channel();

    api.subscribe_events(events_in).unwrap();
    let args: AddEventArgs = api
        .wait_for_event("GovernanceElection", "AddedCandidates", None, &events_out)
        .unwrap();

    println!("AddedCandidates Event: ");
    println!("\tWho: {:?}", args.who);
    println!("\tCandidates:");
    args.account_ids.iter().for_each(|candidate| {
        println!(
            "\t\t- {:?}",
            AccountKeyring::from_account_id(candidate).unwrap()
        );
    })
}

fn vote(api: &mut Api<sr25519::Pair, WsRpcClient>) {
    use std::time::Duration;
    std::thread::sleep(Duration::from_secs(2));
    let mut votes = HashMap::new();
    votes.insert(AccountKeyring::Bob, AccountKeyring::Eve);
    votes.insert(AccountKeyring::Eve, AccountKeyring::Bob);
    votes.insert(AccountKeyring::Ferdie, AccountKeyring::Bob);
    votes.insert(AccountKeyring::Charlie, AccountKeyring::Bob);
    votes.insert(AccountKeyring::Dave, AccountKeyring::Eve);

    let active_properties: Option<Vec<Property>> = api
        .get_storage_value("GovernanceElection", "ActiveProperties", None)
        .unwrap()
        .unwrap();

    let active_properties = active_properties.unwrap();

    votes.into_iter().for_each(|(voter, candidate)| {
        api.signer = Some(voter.clone().pair());
        let mut properties = Vec::new();
        for _ in 0..active_properties.len() {
            properties.push(random::<u8>() % 10 + 1);
        }

        let vote = Vote {
            esg_score: properties,
        };

        let xt = compose_extrinsic!(
            api.clone(),
            "GovernanceElection",
            "vote",
            candidate.to_account_id(),
            vote
        );

        let tx_hash = api
            .send_extrinsic(xt.hex_encode(), XtStatus::InBlock)
            .unwrap();
        println!("[+] Vote transaction got included. Hash: {:?}", tx_hash);
        let (events_in, events_out) = channel();

        api.subscribe_events(events_in).unwrap();
        let args: VoteEventArgs = api
            .wait_for_event("GovernanceElection", "Voted", None, &events_out)
            .unwrap();

        println!("Voted Event: ");
        println!("\tVoter: {:?}", args.voter);
        println!("\tVoted For: {:?}", args.candidate);
        println!("\tVote: {:?}", args.vote);
    });
}

fn main() {
    env_logger::init();
    let url = get_node_url_from_cli();

    let from = AccountKeyring::Alice.pair();
    let client = WsRpcClient::new(&url);
    let mut api = Api::new(client)
        .map(|api| api.set_signer(from.clone()))
        .unwrap();

    add_candidates(&api);
    add_participants(&api);
    configure_vote(&api);
    vote(&mut api);
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
