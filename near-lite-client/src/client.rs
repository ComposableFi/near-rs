use std::collections::HashMap;

use crate::{
    checkpoint::TrustedCheckpoint,
    storage::{DummyStateStorage, StateStorage},
    types::{CryptoHash, LightClientBlockView, Signature},
};

/// LightClient keeps track of at least one block per epoch, the set of validators
/// in each relevant epoch (depends on how much state wants to be stored -- configurable).
/// It is also able to verify a new state transition, and update its head.
#[allow(dead_code)]
pub struct LightClient {
    // current block view tracked by the client which has been validated
    head: LightClientBlockView,

    /// how many epochs the light client will track
    relevant_epochs: usize,

    /// set of validators that can sign a mined block
    block_producers_per_epoch: HashMap<CryptoHash, Vec<Option<Signature>>>,

    state_storage: DummyStateStorage,
}

impl LightClient {
    pub fn with_checkpoint(checkpoint: TrustedCheckpoint, relevant_epochs: usize) -> Self {
        let head = LightClientBlockView::from(checkpoint);
        Self {
            state_storage: DummyStateStorage::new(head.clone()),
            relevant_epochs,
            block_producers_per_epoch: [(
                head.inner_lite.next_epoch_id,
                head.approvals_after_next.clone(),
            )]
            .into_iter()
            .collect(),
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

    struct MockLightClient {
        /// set of validators that can sign a mined block
        #[allow(dead_code)]
        block_producers_per_epoch: HashMap<CryptoHash, Vec<Option<Signature>>>,
        storage: DummyStateStorage,
    }

    impl MockLightClient {
        fn with_checkpoint(checkpoint: TrustedCheckpoint) -> Self {
            let head = LightClientBlockView::from(checkpoint);
            Self {
                storage: DummyStateStorage::new(head.clone()),
                block_producers_per_epoch: [(
                    head.inner_lite.next_epoch_id,
                    head.approvals_after_next,
                )]
                .into_iter()
                .collect::<HashMap<_, _>>(),
            }
        }
    }

    // dummy implementation (at least for now)
    impl StateStorage for MockLightClient {
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

    impl StateTransitionVerificator for MockLightClient {
        type D = Sha256Digest;

        fn validate_and_update_head(&mut self, _block_view: &LightClientBlockView) -> bool {
            true
        }
    }

    #[test]
    fn test_mock_light_with_checkpoint() {
        let mut mock_light_client =
            MockLightClient::with_checkpoint(TrustedCheckpoint::new_for_test());

        let block_view = LightClientBlockView::new_for_test();
        assert!(mock_light_client.validate_and_update_head(&block_view));
    }
}
