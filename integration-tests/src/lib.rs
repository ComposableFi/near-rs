//! Some common utilites
extern crate alloc;

use alloc::collections::BTreeMap;

use near_lite_client::{
	CryptoHash, LightClientBlockView, NearLiteClientTrait, TrustedCheckpoint, ValidatorStakeView,
};
use near_primitives_wasm_friendly::HostFunctions;

pub struct NearHostFunctions;

impl HostFunctions for NearHostFunctions {
	fn sha256(data: &[u8]) -> [u8; 32] {
		use sha2::Digest;
		sha2::Sha256::digest(data).try_into().unwrap()
	}
}

pub struct LightClient {
	pub head: LightClientBlockView,
	pub epoch_block_producers: BTreeMap<CryptoHash, Vec<ValidatorStakeView>>,
}

impl LightClient {
	pub fn update_head(&mut self, block_view: LightClientBlockView) {
		self.epoch_block_producers.insert(
			block_view.inner_lite.next_epoch_id,
			block_view.next_bps.as_ref().unwrap().clone(),
		);
		self.head = block_view;
	}
}
impl NearLiteClientTrait for LightClient {
	fn new_from_checkpoint(checkpoint: TrustedCheckpoint, _heights_to_track: usize) -> Self {
		let block_view = checkpoint.0;
		Self {
			epoch_block_producers: [(
				block_view.inner_lite.next_epoch_id,
				block_view.next_bps.clone().unwrap(),
			)]
			.into_iter()
			.collect::<BTreeMap<_, _>>(),
			head: block_view,
		}
	}

	fn current_block_height(&self) -> u64 {
		self.head.inner_lite.height
	}
}
