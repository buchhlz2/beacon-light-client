pub use types::{Attestation, BeaconBlockHeader, Epoch, EthSpec, ForkVersion, Hash256, SignatureBytes, Slot};
pub use serde::{Deserialize, Serialize};

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
    // ForkVersion is a [u8; 4]
    pub fork_version: ForkVersion
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitteeData {
    #[serde(with = "eth2_serde_utils::quoted_u64")]
    pub index: u64,
    pub slot: Slot,
    #[serde(with = "eth2_serde_utils::quoted_u64_vec")]
    pub validators: Vec<u64>,
}