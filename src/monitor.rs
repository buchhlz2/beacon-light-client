pub use crate::api_client::{ApiResult, BeaconApiClient};
pub use crate::timer::Timer;
pub use crate::settings::Settings;
use std::time::Instant;

pub struct Monitor {
    pub timer: Timer,
    pub config: Settings,
    pub client: BeaconApiClient
}

impl Monitor {
    pub fn from_config(config: &Settings) -> Self {
        let timer: Timer = Timer::new(config.beacon_chain.genesis_time, config.beacon_chain.seconds_per_slot, config.beacon_chain.slots_per_epoch);
        let settings = config.clone();
        let beacon_api_client: BeaconApiClient = BeaconApiClient::new(&config.node.url);

        Self {
            timer,
            config: settings,
            client: beacon_api_client
        }
    }

    pub async fn run(&self) -> ApiResult<()> {
        
        loop {
            let start = Instant::now();
            let (slot, epoch) = self.timer.tick_slot().await;
            let elapsed = start.elapsed();

            let block_header = self.client.get_latest_header().await?;

            println!("Time elapsed: {:?} seconds", elapsed);
            println!("epoch: {}, slot: {}", epoch, slot);
            println!("Header: {:#?}", block_header);
        }
        
        Ok(())
    }
}