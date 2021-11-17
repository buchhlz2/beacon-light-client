#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]
#![allow(unused_must_use)]

use beacon_light_client::settings::Settings;
use beacon_light_client::monitor::Monitor;
use beacon_light_client::server::BeaconApiServer;
use pretty_env_logger;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let config: Settings = Settings::new().expect("config file can be loaded");
    println!("Initializing light client");
    println!("Running in ENV `{}` at Server `localhost:{}` and external Node URL `{}`\n", config.env, config.server.port, config.node.url);

    let server = BeaconApiServer::from_config(&config);
    server.run().await;

    // let monitor = Monitor::from_config(&config);
    // monitor.run().await;

    Ok(())
}