#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]
#![allow(unused_must_use)]

use std::fs;
use std::path::PathBuf;
use std::error::Error;
use reqwest::{Client, Error as HttpError};
use beacon_light_client::settings::Settings;
use beacon_light_client::monitor;

use pretty_env_logger;
#[macro_use] extern crate log;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let config: Settings = Settings::new().expect("config file can be loaded");
    println!("Initializing light client");
    println!("Running in ENV `{}` at URL `{}`\n", config.env, config.server.url);

    monitor::run(&config).await;

    Ok(())
}