pub use crate::api_client::{ApiResult, BeaconApiClient};
pub use crate::light_client_types::{Attestation, BlockHeaderData, BlockId, ChainSpec, CommitteeData, EthSpec, Eth1Data, Hash256, LightClientStore, LightClientUpdate, Period, RootData, SyncCommitteeByValidatorIndices, BeaconState, MainnetEthSpec, Slot, Epoch};
pub use crate::timer::Timer;
pub use crate::light_client_server::LightClientServer;
pub use crate::settings::Settings;
pub use std::sync::Arc;
pub use state_processing;
use std::time::Instant;

pub struct Builder {
    pub timer: Timer,
    pub config: Settings,
    pub client: BeaconApiClient,
    pub server: LightClientServer,
    pub chain_spec: ChainSpec,
    pub store: Arc<LightClientStore>
}

impl Builder {
    pub fn from_config(config: &Settings) -> Self {
        let timer: Timer = Timer::new(config.beacon_chain.genesis_time, config.beacon_chain.seconds_per_slot, config.beacon_chain.slots_per_epoch);
        let settings = config.clone();
        let beacon_api_client: BeaconApiClient = BeaconApiClient::new(&config.node.url);
        let server: LightClientServer = LightClientServer::from_config(&config);
        let light_client_store: LightClientStore = LightClientStore {
            valid_updates: Arc::new(Vec::new())
        };
        let chain_spec = ChainSpec::mainnet();

        Self {
            timer,
            config: settings,
            client: beacon_api_client,
            server,
            chain_spec,
            store: Arc::new(light_client_store)
        }
    }

    pub async fn run<T: EthSpec>(&self) -> ApiResult<()> {
        
        // let start = Instant::now();
        let (timer_slot, timer_epoch) = self.timer.tick_slot().await;
        // let elapsed = start.elapsed();

        // let state = self.client.get_state_at_head().await?;
        Builder::build_light_client_update_at_head::<T>(&self, timer_slot, timer_epoch).await;
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

    pub async fn build_light_client_update_at_head<T: EthSpec>(&self, _timer_slot: Slot, _timer_epoch: Epoch) {
        let block_id = BlockId::Head;
        let (fork_name, signed_block) = self.client.get_signed_beacon_block::<T>(block_id).await.map(|d| (d.version, d.data)).unwrap();
        let fork_version = self.chain_spec.fork_version_for_name(fork_name.unwrap());
        let (beacon_block, block_signature) = signed_block.clone().deconstruct();
        let block_header = beacon_block.block_header();
        // let current_sync_committee = client.get_sync_committees_at_state_root(block_header_data.header.message.state_root).await.unwrap();
        let current_epoch = beacon_block.epoch();
        let current_period: Period = Period::new(current_epoch.sync_committee_period(&self.chain_spec).unwrap());
        let next_period = current_period + 1;
        let next_period_starting_epoch = next_period.start_epoch(&self.chain_spec);
        let next_sync_committee = self.client.get_sync_committees_at_epoch(beacon_block.state_root(), next_period_starting_epoch).await.unwrap();
        let current_sync_committee_aggregate = beacon_block.body().sync_aggregate().unwrap();
        let current_sync_committee_bits = current_sync_committee_aggregate.clone().sync_committee_bits;
        let current_sync_committee_signature = current_sync_committee_aggregate.clone().sync_committee_signature;

        let light_client_update: LightClientUpdate<T> = LightClientUpdate {
            header: block_header,
            sync_committee_bits: current_sync_committee_bits,
            sync_committee_signature: current_sync_committee_signature,
            fork_version: fork_version
        };
        // let current_sync_committee_signature =
    
        // let current_sync_committee = client.current_sync_committee().unwrap().clone();
        // 
        // let light_client_update: LightClientUpdate<T> = LightClientUpdate {
        //     header: block_header,
        //     next_sync_committee: next_sync_committee
        // };
        // store.valid_updates.push(light_client_update);
        
        // println!("Block header:\n{:#?}", block_header_data);
        println!("Beacon block:\n{:#?}", beacon_block);
        println!("Light client update:\n{:#?}", light_client_update);
        // println!("current_epoch:\n{:?}", current_epoch);
        // println!("current_period:\n{:?}", current_period);
        // println!("next_period:\n{:?}", next_period);
        // println!("next_period_starting_epoch:\n{:?}", next_period_starting_epoch);
        // println!("next_sync_committee:\n{:?}", next_sync_committee);
        // println!("light_client_update:\n{:#?}", light_client_update);
    }
}



// pub async fn build_light_client_update_from_state_object(store: &Arc<LightClientStore>, state: &BeaconState<MainnetEthSpec>) {
//     let block_header = state.latest_block_header().clone();
//     let current_sync_committee = state.current_sync_committee().unwrap().clone();
//     let next_sync_committee = state.next_sync_committee().unwrap().clone();
//     let light_client_update: LightClientUpdate<MainnetEthSpec> = LightClientUpdate {
//         header: block_header,
//         next_sync_committee: next_sync_committee
//     };
//     // store.valid_updates.push(light_client_update);
    
//     println!("Updates:\n{:#?}", light_client_update);
// }