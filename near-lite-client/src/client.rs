use std::{collections::HashMap, marker::PhantomData};

use crate::{
    block_validation::BlockValidation,
    checkpoint::TrustedCheckpoint,
    storage::StateStorage,
    types::{CryptoHash, LightClientBlockView, Signature},
};

/// LightClient keeps track of at least one block per epoch, the set of validators
/// in each relevant epoch (depends on how much state wants to be stored -- configurable).
/// It is also able to verify a new state transition, and update its head.
#[allow(dead_code)]
pub struct LightClient<S: StateStorage, V: BlockValidation> {
    // current block view tracked by the client which has been validated
    head: LightClientBlockView,

    /// how many epochs the light client will track
    relevant_epochs: usize,

    /// set of validators that can sign a mined block
    block_producers_per_epoch: HashMap<CryptoHash, Vec<Option<Signature>>>,

    _s: PhantomData<S>, // TODO remote this one
    _v: PhantomData<V>,
}

impl<S: StateStorage, V: BlockValidation> LightClient<S, V> {
    pub fn with_checkpoint(checkpoint: TrustedCheckpoint, relevant_epochs: usize) -> Self {
        let head = LightClientBlockView::from(checkpoint);
        Self {
            relevant_epochs,
            block_producers_per_epoch: [(head.inner_lite.next_epoch_id, head.approvals_after_next)]
                .into_iter()
                .collect::<HashMap<_, _>>(),
            _s: PhantomData::default(),
            _v: PhantomData::default(),
            head,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        block_validation::Sha256Digest, storage::DummyStateStorage,
        verifier::StateTransitionVerificator,
    };

    struct MockLightClient<V: BlockValidation> {
        /// set of validators that can sign a mined block
        block_producers_per_epoch: HashMap<CryptoHash, Vec<Option<Signature>>>,
        storage: DummyStateStorage,
        _v: PhantomData<V>,
    }

    impl<V: BlockValidation> MockLightClient<V> {
        fn with_checkpoint(checkpoint: TrustedCheckpoint) -> Self {
            let head = LightClientBlockView::from(checkpoint);
            Self {
                storage: DummyStateStorage::new(head),
                block_producers_per_epoch: [(
                    head.inner_lite.next_epoch_id,
                    head.approvals_after_next,
                )]
                .into_iter()
                .collect::<HashMap<_, _>>(),
                _v: PhantomData::default(),
            }
        }
    }

    // dummy implementation (at least for now)
    impl<V: BlockValidation> StateStorage for MockLightClient<V> {
        fn get_head(&mut self) -> &mut LightClientBlockView {
            self.storage.get_head()
        }

        fn get_epoch_block_producers(
            &self,
        ) -> &HashMap<CryptoHash, Vec<crate::types::ValidatorStakeView>> {
            todo!()
        }

        fn get_epoch_block_producers_mut(
            &mut self,
        ) -> &mut HashMap<CryptoHash, Vec<crate::types::ValidatorStakeView>> {
            todo!()
        }
    }

    impl<V: BlockValidation> StateTransitionVerificator for MockLightClient<V> {
        type V = V;
        type D = Sha256Digest;

        fn validate_and_update_head(&mut self, _block_view: &LightClientBlockView) -> bool {
            true
        }
    }

    #[test]
    fn test_mock_light_with_checkpoint() {
        let mut mock_light_client =
            MockLightClient::<Sha256Digest>::with_checkpoint(TrustedCheckpoint::new());

        let block_view = LightClientBlockView {
            prev_block_hash: todo!(),
            next_block_inner_hash: todo!(),
            inner_lite: todo!(),
            inner_rest_hash: todo!(),
            next_bps: todo!(),
            approvals_after_next: todo!(),
        };
        assert!(mock_light_client.validate_and_update_head(&block_view));
    }
}
