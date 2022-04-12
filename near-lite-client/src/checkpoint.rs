//! # Trusted checkpoint
//!
//! A lite client keeps track of a subset of the total blockchain data.
//! To avoid having to play all the state transitions in order to startup a chain
//! there's also the possiblity to feed the client with a checkpoint. A checkpoint
//! is a state of the chain that's considered **valid**.

use crate::types::{BlockProducer, BlockView, EpochId, ValidatorStakeView};

pub struct TrustedCheckpoint {
    pub epoch_id: EpochId,
    pub next_epoch_id: EpochId,
    pub height: u64,
    pub next_bps: Option<Vec<ValidatorStakeView>>,
    pub approvals_after_next: Vec<Option<BlockProducer>>,
}

impl From<&TrustedCheckpoint> for BlockView {
    fn from(v: &TrustedCheckpoint) -> Self {
        Self {
            epoch_id: v.epoch_id.clone(),
            next_epoch_id: v.next_epoch_id.clone(),
            height: v.height,
            next_bps: v.next_bps.clone(),
            approvals_after_next: v.approvals_after_next.clone(),
        }
    }
}
