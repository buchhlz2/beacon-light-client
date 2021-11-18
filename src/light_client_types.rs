pub use types::{Attestation, BeaconBlockHeader, BeaconState, ChainSpec, Epoch, EthSpec, Eth1Data, ForkName, MainnetEthSpec, ForkVersion, Hash256, SignedBeaconBlock, SignatureBytes, Slot, SyncCommittee};
pub use serde::{Deserialize, Serialize};
pub use std::sync::Arc;
pub use std::fmt;

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

#[derive(Serialize, Deserialize)]
pub struct SyncCommitteesQuery {
    pub epoch: Option<Epoch>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BlockId {
    Head,
    Genesis,
    Finalized,
    Justified,
    Slot(Slot),
    Root(Hash256),
}

impl fmt::Display for BlockId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlockId::Head => write!(f, "head"),
            BlockId::Genesis => write!(f, "genesis"),
            BlockId::Finalized => write!(f, "finalized"),
            BlockId::Justified => write!(f, "justified"),
            BlockId::Slot(slot) => write!(f, "{}", slot),
            BlockId::Root(root) => write!(f, "{:?}", root),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(bound = "T: Serialize + serde::de::DeserializeOwned")]
pub struct GenericResponse<T: Serialize + serde::de::DeserializeOwned> {
    pub data: T,
}

impl<T: Serialize + serde::de::DeserializeOwned> From<T> for GenericResponse<T> {
    fn from(data: T) -> Self {
        Self { data }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
// #[serde(bound = "T: Serialize + serde::de::DeserializeOwned")]
pub struct ForkVersionedResponse<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<ForkName>,
    pub data: T,
}

#[cfg_attr(feature = "arbitrary-fuzz", derive(arbitrary::Arbitrary))]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Period(#[serde(with = "eth2_serde_utils::quoted_u64")] u64);

impl Period {
    pub const fn new(epoch: u64) -> Period {
        Period(epoch)
    }

    /// The first epoch in the period.
    pub fn start_epoch(self, spec: &ChainSpec) -> Epoch {
        Epoch::from(self.0.saturating_mul(spec.epochs_per_sync_committee_period.as_u64()))
    }

    /// The last epoch in the period.
    pub fn end_epoch(self, spec: &ChainSpec) -> Epoch {
        Epoch::from(
            self.0
                .saturating_mul(spec.epochs_per_sync_committee_period.as_u64())
                .saturating_add(spec.epochs_per_sync_committee_period.as_u64().saturating_sub(1)),
        )
    }
}

impl std::ops::Add<u64> for Period {
    type Output = Self;

    fn add(self, other: u64) -> Self::Output {
        Self(self.0 + other)
    }
}

impl fmt::Debug for Period {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({:?})", stringify!(Period), self.0)
    }
}