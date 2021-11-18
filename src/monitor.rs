pub use crate::api_client::{ApiResult, BeaconApiClient};
pub use crate::light_client_types::{Attestation, BlockHeaderData, ChainSpec, CommitteeData, EthSpec, Eth1Data, Hash256, LightClientStore, LightClientUpdate, RootData, SyncCommitteeByValidatorIndices, BeaconState, MainnetEthSpec};
pub use crate::timer::Timer;
pub use crate::light_client_server::LightClientServer;
pub use crate::settings::Settings;
pub use std::sync::Arc;
pub use state_processing;
use std::time::Instant;

pub struct Monitor {
    pub timer: Timer,
    pub config: Settings,
    pub client: BeaconApiClient,
    pub server: LightClientServer,
    pub store: Arc<LightClientStore>
}

impl Monitor {
    pub fn from_config(config: &Settings) -> Self {
        let timer: Timer = Timer::new(config.beacon_chain.genesis_time, config.beacon_chain.seconds_per_slot, config.beacon_chain.slots_per_epoch);
        let settings = config.clone();
        let beacon_api_client: BeaconApiClient = BeaconApiClient::new(&config.node.url);
        let server: LightClientServer = LightClientServer::from_config(&config);
        let light_client_store: LightClientStore = LightClientStore {
            valid_updates: Arc::new(Vec::new())
        };

        Self {
            timer,
            config: settings,
            client: beacon_api_client,
            server,
            store: Arc::new(light_client_store)
        }
    }

    pub async fn run(&self) -> ApiResult<()> {
        
        // let start = Instant::now();
        let (slot, epoch) = self.timer.tick_slot().await;
        // let elapsed = start.elapsed();

        let state = self.client.get_state_at_head().await?;
        build_light_client_update(&self.store, &state).await;
        // let sync_committee = self.client.get_sync_committees_at_state(block_header).await?;
        
        // let current_sync_committee = state.current_sync_committee();
        // let next_sync_committee = state.next_sync_committee();

        // println!("Header:\n{:#?}", block_header);
        // println!("Sync committee at header: {:#?}", sync_committee);
        // println!("sync_committee: {:?}", state.current_sync_committee());

        // log::info!("Time elapsed: {:?} seconds", elapsed);
        // log::info!("epoch: {}, slot: {}", epoch, slot);
        Ok(())
    }
}

pub async fn build_light_client_update(store: &Arc<LightClientStore>, state: &BeaconState<MainnetEthSpec>) {
    let block_header = state.latest_block_header().clone();
    let current_sync_committee = state.current_sync_committee();
    let next_sync_committee = state.next_sync_committee().unwrap().clone();
    let light_client_update: LightClientUpdate<MainnetEthSpec> = LightClientUpdate {
        header: block_header,
        next_sync_committee: next_sync_committee
    };
    store.valid_updates.push(light_client_update);
    
    println!("Updates:\n{:#?}", light_client_update);
}