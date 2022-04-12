use crate::{
    block_validation::{BlockValidation, Digest},
    storage::StateStorage,
    types::{CryptoHash, LightClientBlockView},
};

pub trait StateTransitionVerificator: StateStorage {
    type V: BlockValidation;
    type D: Digest;

    fn validate_and_update_head(&mut self, block_view: &LightClientBlockView) -> bool {
        self.get_epoch_block_producers_mut().insert(
            block_view.inner_lite.next_epoch_id,
            block_view.next_bps.unwrap().clone(),
        );

        let head = self.get_head();
        *head = block_view.clone();
        true
    }
}

fn reconstruct_light_client_block_view_fields(
    block_view: &LightClientBlockView,
) -> (CryptoHash, CryptoHash, Vec<u8>) {
    let current_block_hash = block_view.current_block_hash();
    let next_block_hash = next_block_hash(block_view.next_block_inner_hash, current_block_hash);
    let approval_message = [
        ApprovalInner::Endorsement(next_block_hash)
            .try_to_vec()
            .unwrap(),
        (block_view.inner_lite.height + 2)
            .to_le()
            .try_to_vec()
            .unwrap(),
    ]
    .concat();
    (current_block_hash, next_block_hash, approval_message)
}
