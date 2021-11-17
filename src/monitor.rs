pub use crate::api_client::{ApiResult, BeaconApiLightClient};
pub use crate::timer::Timer;
pub use crate::settings::Settings;
use std::time::Instant;

pub struct Monitor {
    timer: Timer,
    config: Settings
}

impl Monitor {
    pub fn from_config(config: &Settings) -> Self {
        let timer: Timer = Timer::new(config.beacon_chain.genesis_time, config.beacon_chain.seconds_per_slot, config.beacon_chain.slots_per_epoch);
        let settings = config.clone();
        Self {
            timer,
            config: settings
        }
    }

    pub async fn run(&self) -> ApiResult<()> {
        let beacon_api_light_client: BeaconApiLightClient = BeaconApiLightClient::new(&self.config.node.url);

        loop {
            let start = Instant::now();
            let (slot, epoch) = self.timer.tick_slot().await;
            let elapsed = start.elapsed();
    
            let header = beacon_api_light_client.get_latest_header().await;
            println!("Header: {:#?}", header);
    
            log::info!("Time elapsed: {:?} seconds", elapsed);
            log::info!("epoch: {}, slot: {}", epoch, slot);
        }
        Ok(())
    }
}