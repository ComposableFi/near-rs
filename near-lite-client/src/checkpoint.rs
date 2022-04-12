//! # Trusted checkpoint
//!
//! A lite client keeps track of a subset of the total blockchain data.
//! To avoid having to play all the state transitions in order to startup a chain
//! there's also the possiblity to feed the client with a checkpoint. A checkpoint
//! is a state of the chain that's considered **valid**.

use crate::types::LightClientBlockView;

pub struct TrustedCheckpoint(LightClientBlockView);

#[cfg(test)]
impl TrustedCheckpoint {
    pub fn new() -> TrustedCheckpoint {
        Self(LightClientBlockView {
            prev_block_hash: todo!(),
            next_block_inner_hash: todo!(),
            inner_lite: todo!(),
            inner_rest_hash: todo!(),
            next_bps: todo!(),
            approvals_after_next: todo!(),
        })
    }
}

impl From<TrustedCheckpoint> for LightClientBlockView {
    fn from(s: TrustedCheckpoint) -> Self {
        s.0
    }
}
