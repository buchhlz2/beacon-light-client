pub use types::{Attestation, BeaconBlockHeader, BeaconState, ChainSpec, Epoch, EthSpec, Eth1Data, MainnetEthSpec, ForkVersion, Hash256, SignatureBytes, Slot, SyncCommittee};
pub use serde::{Deserialize, Serialize};
pub use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockHeaderAndSignature {
    pub message: BeaconBlockHeader,
    pub signature: SignatureBytes,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockHeaderData {
    pub root: Hash256,
    pub canonical: bool,
    pub header: BlockHeaderAndSignature,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightClientStore {
    pub valid_updates: Arc<Vec<LightClientUpdate<MainnetEthSpec>>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightClientUpdate<T: EthSpec> {
    pub header: BeaconBlockHeader,
    // TO DO: change to `SyncCommittee`
    pub next_sync_committee: Arc<SyncCommittee<T>>,
    // // TO DO: change to FixedVector<Hash256, T::SIZE_OF_VECTOR>,
    // pub next_sync_committee_branch: Vec<Hash256>,
    // pub finality_header: Option<BeaconBlockHeader>,
    // // TO DO: change to FixedVector<Hash256, T::SIZE_OF_VECTOR>,
    // pub finality_branch: Option<Vec<Hash256>>,
    // // TO DO: change to BitVector<T::SyncCommitteeSize>
    // pub sync_committee_bits: Vec<u8>,
    // // TO DO: change to `AggregateSignature`
    // pub sync_committee_signature: String,
    // // ForkVersion is a [u8; 4]
    // pub fork_version: ForkVersion
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitteeData {
    #[serde(with = "eth2_serde_utils::quoted_u64")]
    pub index: u64,
    pub slot: Slot,
    #[serde(with = "eth2_serde_utils::quoted_u64_vec")]
    pub validators: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SyncCommitteeByValidatorIndices {
    #[serde(with = "eth2_serde_utils::quoted_u64_vec")]
    pub validators: Vec<u64>,
    pub validator_aggregates: Vec<SyncSubcommittee>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SyncSubcommittee {
    #[serde(with = "eth2_serde_utils::quoted_u64_vec")]
    pub indices: Vec<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RootData {
    pub root: Hash256,
}