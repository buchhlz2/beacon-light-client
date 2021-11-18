pub use crate::monitor::Monitor;
pub use crate::settings::Settings;

pub struct LightClientServer {
    config: Settings
}

impl LightClientServer {
    pub fn from_config(config: &Settings) -> Self {
        let settings = config.clone();

        Self {
            config: settings
        }
    }
}