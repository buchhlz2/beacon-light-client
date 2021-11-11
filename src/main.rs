#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]
#![allow(unused_must_use)]

use std::fs;
use std::path::PathBuf;
use reqwest::{Client, Error as HttpError};
use beacon_light_client::light_client_types::*;
use beacon_light_client::timer::Timer;
use beacon_light_client::api_client::BeaconLightClient;

use pretty_env_logger;
#[macro_use] extern crate log;

#[derive(Debug)]
pub struct BeaconChainConfig {
    pub seconds_per_slot: u64,
    pub slots_per_epoch: u64,
    pub genesis_time: u64,
}

#[tokio::main]
async fn main() -> Result<(), HttpError> {
    pretty_env_logger::init();
    println!("Initializing light client");

    let beacon_chain_config: BeaconChainConfig = BeaconChainConfig {
        seconds_per_slot: 1,
        slots_per_epoch: 3,
        genesis_time: 1606824023
    };

    let timer: &Timer = &Timer::new(beacon_chain_config.seconds_per_slot, beacon_chain_config.slots_per_epoch, beacon_chain_config.genesis_time);
    
    let beacon_api_light_client: BeaconLightClient = BeaconLightClient::new("https://jsonplaceholder.typicode.com/todos/1").await;

    let header = beacon_api_light_client.get_block_header().await;
    println!("{:?}", header);

    Ok(())
}