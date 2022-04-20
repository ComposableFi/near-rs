use std::collections::HashMap;

use crate::{
    block_validation::SubstrateDigest,
    checkpoint::TrustedCheckpoint,
    storage::{DummyStateStorage, StateStorage},
    types::{CryptoHash, LightClientBlockView, LiteClientResult, ValidatorStakeView},
    verifier::StateTransitionVerificator,
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

    state_storage: DummyStateStorage,
}

impl LightClient {
    pub fn with_checkpoint(checkpoint: TrustedCheckpoint, relevant_epochs: usize) -> Self {
        let head = LightClientBlockView::from(checkpoint);
        Self {
            state_storage: DummyStateStorage::new(
                head.clone(),
                (
                    head.inner_lite.next_epoch_id,
                    head.approvals_after_next.clone(),
                ),
            ),
            relevant_epochs,
            head,
        }
    }
}

impl StateStorage for LightClient {
    fn get_head(&self) -> &LightClientBlockView {
        self.state_storage.get_head()
    }

    fn get_head_mut(&mut self) -> &mut LightClientBlockView {
        self.state_storage.get_head_mut()
    }

    fn get_epoch_block_producers(
        &self,
    ) -> &HashMap<CryptoHash, Vec<crate::types::ValidatorStakeView>> {
        todo!()
    }

    fn insert_epoch_block_producers(&mut self, _epoch: CryptoHash, _bps: Vec<ValidatorStakeView>) {
        todo!()
    }
}

impl StateTransitionVerificator for LightClient {
    type D = SubstrateDigest;

    fn validate_and_update_head(
        &mut self,
        _block_view: &LightClientBlockView,
    ) -> LiteClientResult<bool> {
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        block_validation::Sha256Digest,
        storage::{DummyStateStorage, StateStorage},
        types::LiteClientResult,
        verifier::StateTransitionVerificator,
    };

    struct MockLightClient {
        /// set of validators that can sign a mined block
        storage: DummyStateStorage,
    }

    impl MockLightClient {
        fn with_checkpoint(checkpoint: TrustedCheckpoint) -> Self {
            let head = LightClientBlockView::from(checkpoint);
            Self {
                storage: DummyStateStorage::new(
                    head.clone(),
                    (head.inner_lite.next_epoch_id, head.approvals_after_next),
                ),
            }
        }
    }

    // dummy implementation (at least for now)
    impl StateStorage for MockLightClient {
        fn get_head(&self) -> &LightClientBlockView {
            self.storage.get_head()
        }

        fn get_head_mut(&mut self) -> &mut LightClientBlockView {
            self.storage.get_head_mut()
        }

        fn get_epoch_block_producers(
            &self,
        ) -> &HashMap<CryptoHash, Vec<crate::types::ValidatorStakeView>> {
            todo!()
        }

        fn insert_epoch_block_producers(
            &mut self,
            _epoch: CryptoHash,
            _bps: Vec<ValidatorStakeView>,
        ) {
            todo!()
        }
    }

    impl StateTransitionVerificator for MockLightClient {
        type D = Sha256Digest;

        fn validate_and_update_head(
            &mut self,
            _block_view: &LightClientBlockView,
        ) -> LiteClientResult<bool> {
            Ok(true)
        }
    }

    #[test]
    fn test_mock_light_with_checkpoint() {
        let mut mock_light_client =
            MockLightClient::with_checkpoint(TrustedCheckpoint::new_for_test());

        let block_view = LightClientBlockView::new_for_test();
        assert!(mock_light_client
            .validate_and_update_head(&block_view)
            .unwrap());
    }
}
