use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize};
use std::fmt;

const CONFIG_FILE_PATH: &str = "./config/Default.toml";
const CONFIG_FILE_PREFIX: &str = "./config/";

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub node: Node,
    pub server: Server,
    pub log: Log,
    pub env: ENV,
    pub beacon_chain: BeaconChain
}

#[derive(Debug, Deserialize, Clone)]
pub struct BeaconChain {
    pub seconds_per_slot: u64,
    pub slots_per_epoch: u64,
    pub genesis_time: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Node {
    // pub port: u16,
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub port: u16,
    // pub url: String,
}

#[derive(Clone, Debug, Deserialize)]
pub enum ENV {
    Default,
    Development,
}

impl fmt::Display for ENV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ENV::Default => write!(f, "Default"),
            ENV::Development => write!(f, "Development"),
        }
    }
}

impl From<&str> for ENV {
    fn from(env: &str) -> Self {
        match env {
            "Development" => ENV::Development,
            _ => ENV::Default,
        }
    }
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let env = std::env::var("RUN_ENV").unwrap_or_else(|_| "Default".into());
        let mut s = Config::new();
        s.set("env", env.clone())?;

        s.merge(File::with_name(CONFIG_FILE_PATH))?;
        s.merge(File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX, env)))?;

        // This makes it so "EA_SERVER__PORT overrides server.port
        s.merge(Environment::with_prefix("ea").separator("__"))?;

        s.try_into()
    }
}
