#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub use types::{Slot, Epoch, Hash256};
pub use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct BeaconBlockHeader {
    pub slot: Slot,
    pub proposer_index: u64,
    pub parent_root: Hash256,
    pub state_root: Hash256,
    pub body_root: Hash256,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockHeaderAndSignature {
    pub message: BeaconBlockHeader,
    pub signature: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockHeaderData {
    pub root: Hash256,
    pub canonical: bool,
    // pub header: String,
    // pub header: BlockHeaderAndSignature,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct LightClientUpdate {
    pub header: BeaconBlockHeader,
    // TO DO: change to `SyncCommittee`
    pub next_sync_committee: String,
    // TO DO: change to FixedVector<Hash256, T::SIZE_OF_VECTOR>,
    pub next_sync_committee_branch: Vec<Hash256>,
    pub finality_header: Option<BeaconBlockHeader>,
    // TO DO: change to FixedVector<Hash256, T::SIZE_OF_VECTOR>,
    pub finality_branch: Option<Vec<Hash256>>,
    // TO DO: change to BitVector<T::SyncCommitteeSize>
    pub sync_committee_bits: Vec<u8>,
    // TO DO: change to `AggregateSignature`
    pub sync_committee_signature: String,
    pub fork_version: [u8; 4]
}

// pub struct LightClientUpdate {
//     pub header: BeaconBlockHeader,
//     // TO DO: change to `SyncCommittee`
//     pub next_sync_committee: String,
//     // TO DO: change to FixedVector<Hash256, T::SIZE_OF_VECTOR>,
//     pub next_sync_committee_branch: Vec<Hash256>,
//     pub finality_header: Option<BeaconBlockHeader>,
//     // TO DO: change to FixedVector<Hash256, T::SIZE_OF_VECTOR>,
//     pub finality_branch: Option<Vec<Hash256>>,
//     // TO DO: change to BitVector<T::SyncCommitteeSize>
//     pub sync_committee_bits: Vec<bool>,
//     // TO DO: change to `AggregateSignature`
//     pub sync_committee_signature: String,
//     pub fork_version: ForkVersion
// }