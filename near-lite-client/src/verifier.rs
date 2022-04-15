use crate::{
    block_validation::{BlockValidation, Digest},
    storage::StateStorage,
    types::{ApprovalInner, CryptoHash, LightClientBlockView},
};

use borsh::BorshSerialize;
pub trait StateTransitionVerificator: StateStorage {
    type D: Digest;

    fn validate_and_update_head(&mut self, block_view: &LightClientBlockView) -> bool {
        self.get_epoch_block_producers_mut().insert(
            block_view.inner_lite.next_epoch_id,
            block_view.next_bps.as_ref().unwrap().clone(),
        );

        let head = self.get_head();
        *head = block_view.clone();
        true
    }
}

fn reconstruct_light_client_block_view_fields<D: Digest>(
    block_view: &LightClientBlockView,
) -> (CryptoHash, CryptoHash, Vec<u8>) {
    let current_block_hash = block_view.current_block_hash::<D>();
    let next_block_hash =
        next_block_hash::<D>(block_view.next_block_inner_hash, current_block_hash);
    let approval_message = [
        ApprovalInner::Endorsement(next_block_hash)
            .try_to_vec()
            .unwrap()
            .as_ref(),
        (block_view.inner_lite.height + 2).to_le_bytes().as_ref(), // TODO: double check this one
    ]
    .concat();
    (current_block_hash, next_block_hash, approval_message)
}

pub(crate) fn next_block_hash<D: Digest>(
    next_block_inner_hash: CryptoHash,
    current_block_hash: CryptoHash,
) -> CryptoHash {
    D::digest([next_block_inner_hash.as_ref(), current_block_hash.as_ref()].concat())
        .as_slice()
        .try_into()
        .unwrap()
}
