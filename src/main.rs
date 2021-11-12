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
use beacon_light_client::settings;

#[macro_use]
extern crate lazy_static;
lazy_static! {
    static ref CONFIG: settings::Settings =
        settings::Settings::new().expect("config can be loaded");
}

use pretty_env_logger;
#[macro_use] extern crate log;

#[tokio::main]
async fn main() -> Result<(), HttpError> {
    pretty_env_logger::init();
    println!("Initializing light client");
    println!("Running in ENV `{}` at URL `{}`", CONFIG.env, CONFIG.server.url);

    let beacon_chain_config = &CONFIG.beacon_chain;

    let timer: &Timer = &Timer::new(beacon_chain_config.seconds_per_slot, beacon_chain_config.slots_per_epoch, beacon_chain_config.genesis_time);
    
    let beacon_api_light_client: BeaconLightClient = BeaconLightClient::new(&CONFIG.server.url).await;

    let header = beacon_api_light_client.get_block_header().await;
    println!("{:?}", header);

    Ok(())
}