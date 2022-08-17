use crate::checkpoint::TrustedCheckpoint;

use near_primitives_wasm_friendly::{HostFunctions, LightClientBlockView, PublicKey, Signature};

/// LightClient keeps track of at least one block per epoch, the set of validators
/// in each relevant epoch (depends on how much state wants to be stored -- configurable).
/// It is also able to verify a new state transition, and update its head.
#[allow(dead_code)]
pub struct LightClient {
	/// how many epochs the light client will track
	relevant_epochs: usize,
	state_storage: DummyStateStorage,
}

impl LightClient {
	pub fn new_from_checkpoint(checkpoint: TrustedCheckpoint, relevant_epochs: usize) -> Self {
		let head = LightClientBlockView::from(checkpoint);
		Self {
			state_storage: DummyStateStorage::new(
				head.clone(),
				(head.inner_lite.next_epoch_id, head.next_bps.as_ref().unwrap().clone()),
			),
			relevant_epochs,
		}
	}

	pub fn current_block_height(&self) -> u64 {
		self.state_storage.get_head().inner_lite.height
	}
}

pub struct NearHostFunctions;

impl HostFunctions for NearHostFunctions {
	fn sha256(data: &[u8]) -> [u8; 32] {
		use sha2::Digest;
		sha2::Sha256::digest(data).try_into().unwrap()
	}

	fn verify(signature: Signature, data: impl AsRef<[u8]>, public_key: PublicKey) -> bool {
		match signature {
			Signature::Ed25519(signature) => {
				ed25519_verify(signature, data.as_ref(), &Ed25519Public::from(&public_key))
			},
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	struct MockLightClient {
		/// set of validators that can sign a mined block
		storage: DummyStateStorage,
	}

	impl MockLightClient {
		fn new_from_checkpoint(checkpoint: TrustedCheckpoint) -> Self {
			let head = LightClientBlockView::from(checkpoint);
			Self {
				storage: DummyStateStorage::new(
					head.clone(),
					(head.inner_lite.next_epoch_id, head.next_bps.as_ref().unwrap().clone()),
				),
			}
		}
	}

	#[test]
	fn test_mock_light_new_from_checkpoint() {
		let mut mock_light_client =
			MockLightClient::new_from_checkpoint(TrustedCheckpoint::new_for_test());

		let block_view = LightClientBlockView::new_for_test();
		assert!(mock_light_client.validate_head(&block_view).unwrap());
	}
}
