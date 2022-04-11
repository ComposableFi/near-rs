//! # Trusted checkpoint
//!
//! A lite client keeps track of a subset of the total blockchain data.
//! To avoid having to play all the state transitions in order to startup a chain
//! there's also the possiblity to feed the client with a checkpoint. A checkpoint
//! is a state of the chain that's considered **valid**.

use crate::types::{Signature, SignatureType};

pub struct TrustedCheckpoint {
    pub epoch_id: Vec<u8>,
    pub height: u64,
    pub next_bps: Vec<(SignatureType, Signature)>,
    // TODO: add more fields
}
