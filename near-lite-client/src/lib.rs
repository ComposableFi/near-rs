//! # Near lite client
//!
//! The main purpose of the lite client is to keep track of a small subset
//! of the chain's state while still being able to:
//! 1. verify the chain's state transitions and keep a subset of the state
//! 2. verify that a transaction belongs to a vald block
//!
//! ## Usage
//!
//! ```ignore
//! use near_lite_client::prelude::*;
//! // call the Light Client constructuro with a `TrustedCheckpoint`
//! let mut lite_client = LightClient::with_checkpoint(trusted_checkpoint);
//!
//! // there are two operations that can be performed:
//! // `validate_head` & `validate_transaction`
//!
//! lite_client.validate_head(block_view);
//! lite_client.validate_transaction(outcome_proof, outcome_root_proof, expected_block_outcome_root);
//! ```
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod block_validation;
mod checkpoint;
mod error;
mod merkle_tree;
#[cfg(test)]
pub mod test_utils;
mod verifier;

pub use checkpoint::TrustedCheckpoint;
pub use near_primitives_wasm::{
	CryptoHash, LightClientBlockView, MerklePath, NearSignature, OutcomeProof, ValidatorStakeView,
};
pub use verifier::{validate_head, validate_transaction, validate_transactions};

use crate::error::NearLiteClientError;

pub type LiteClientResult<T> = Result<T, NearLiteClientError>;

pub mod prelude {
	pub use super::{
		validate_head, validate_transaction, validate_transactions, CryptoHash,
		LightClientBlockView, MerklePath, NearLiteClientTrait, NearSignature, OutcomeProof,
		TrustedCheckpoint, ValidatorStakeView,
	};
}

pub trait NearLiteClientTrait {
	fn new_from_checkpoint(checkpoint: TrustedCheckpoint, heights_to_track: usize) -> Self;
	fn current_block_height(&self) -> u64;
}
