pub use crate::api_client::{ApiResult, BeaconLightClient};
pub use crate::timer::Timer;
pub use crate::settings::Settings;
use std::time::Instant;

pub async fn run(config: &Settings) -> ApiResult<()> {
    let beacon_api_light_client: BeaconLightClient = BeaconLightClient::new(&config.server.url).await;
    let timer: &Timer = &Timer::new(config.beacon_chain.genesis_time, config.beacon_chain.seconds_per_slot, config.beacon_chain.slots_per_epoch);

    loop {
        let start = Instant::now();
        let (slot, epoch) = &timer.tick_slot().await;
        let elapsed = start.elapsed();

        let header = beacon_api_light_client.get_block_header().await;
        println!("Header: {:#?}", header);

        log::info!("Time elapsed: {:?} seconds", elapsed);
        log::info!("epoch: {}, slot: {}", epoch, slot);
    }
}