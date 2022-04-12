use std::{collections::HashMap, marker::PhantomData};

use crate::{
    block_validation::BlockValidation,
    checkpoint::TrustedCheckpoint,
    storage::StateStorage,
    types::{BlockProducer, BlockView, EpochId},
};

/// LightClient keeps track of at least one block per epoch, the set of validators
/// in each relevant epoch (depends on how much state wants to be stored -- configurable).
/// It is also able to verify a new state transition, and update its head.
#[allow(dead_code)]
pub struct LightClient<S: StateStorage, V: BlockValidation> {
    // current block view tracked by the client which has been validated
    head: BlockView,

    /// how many epochs the light client will track
    relevant_epochs: usize,

    /// set of validators that can sign a mined block
    block_producers_per_epoch: HashMap<EpochId, Vec<Option<BlockProducer>>>,

    _s: PhantomData<S>,
    _v: PhantomData<V>,
}

impl<S: StateStorage, V: BlockValidation> LightClient<S, V> {
    pub fn with_checkpoint(checkpoint: TrustedCheckpoint, relevant_epochs: usize) -> Self {
        Self {
            head: BlockView::from(&checkpoint),
            relevant_epochs,
            block_producers_per_epoch: [(
                checkpoint.next_epoch_id,
                checkpoint.approvals_after_next,
            )]
            .into_iter()
            .collect::<HashMap<_, _>>(),
            _s: PhantomData::default(),
            _v: PhantomData::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        block_validation::Sha256Digest, signature::DummyVerificator, storage::DummyStateStorage,
        verifier::StateTransitionVerificator,
    };

    struct MockLightClient<S: StateStorage, V: BlockValidation> {
        head: BlockView,
        /// set of validators that can sign a mined block
        block_producers_per_epoch: HashMap<EpochId, Vec<Option<BlockProducer>>>,
        _s: PhantomData<S>,
        _v: PhantomData<V>,
    }

    impl<S: StateStorage, V: BlockValidation> MockLightClient<S, V> {
        fn with_checkpoint(checkpoint: TrustedCheckpoint) -> Self {
            Self {
                head: BlockView::from(&checkpoint),
                block_producers_per_epoch: [(
                    checkpoint.next_epoch_id,
                    checkpoint.approvals_after_next,
                )]
                .into_iter()
                .collect::<HashMap<_, _>>(),

                _s: PhantomData::default(),
                _v: PhantomData::default(),
            }
        }
    }

    impl<S: StateStorage, V: BlockValidation> StateTransitionVerificator for MockLightClient<S, V> {
        type V = V;
        type S = S;

        fn validate_and_update_head(&mut self, _block_view: &BlockView) -> bool {
            true
        }
    }

    #[test]
    fn test_mock_light_with_checkpoint() {
        let mut mock_light_client =
            MockLightClient::<DummyStateStorage, Sha256Digest>::with_checkpoint(
                TrustedCheckpoint {
                    epoch_id: vec![],
                    height: 0,
                    next_bps: None,
                    next_epoch_id: vec![],
                    approvals_after_next: vec![],
                },
            );

        let block_view = BlockView {
            height: 1,
            epoch_id: vec![],
            next_epoch_id: vec![],
            next_bps: None,
            approvals_after_next: vec![],
        };
        assert!(mock_light_client.validate_and_update_head(&block_view));
    }
}
