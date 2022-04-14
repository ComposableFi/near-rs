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
    #[cfg(test)]
    pub fn new_for_test() -> Self {
        Self(LightClientBlockView::new_for_test())
    }
}

impl From<TrustedCheckpoint> for LightClientBlockView {
    fn from(s: TrustedCheckpoint) -> Self {
        s.0
    }
}
