use sp_std::{collections::btree_map::BTreeMap, vec::Vec};

use crate::{
    block_validation::SubstrateDigest,
    checkpoint::TrustedCheckpoint,
    storage::{DummyStateStorage, StateStorage},
    types::{LightClientBlockView, ValidatorStakeView},
    verifier::StateTransitionVerificator,
};

use near_primitives::hash::CryptoHash;

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
                (
                    head.inner_lite.next_epoch_id,
                    head.next_bps.as_ref().unwrap().clone(),
                ),
            ),
            relevant_epochs,
        }
    }

    pub fn current_block_height(&self) -> u64 {
        self.state_storage.get_head().inner_lite.height
    }
}

impl StateStorage for LightClient {
    fn get_head(&self) -> &LightClientBlockView {
        self.state_storage.get_head()
    }

    fn set_new_head(&mut self, new_head: LightClientBlockView) {
        self.state_storage.set_new_head(new_head)
    }

    fn get_epoch_block_producers(
        &self,
    ) -> &BTreeMap<CryptoHash, Vec<crate::types::ValidatorStakeView>> {
        self.state_storage.get_epoch_block_producers()
    }

    fn insert_epoch_block_producers(&mut self, epoch: CryptoHash, bps: Vec<ValidatorStakeView>) {
        self.state_storage.insert_epoch_block_producers(epoch, bps)
    }
}

impl StateTransitionVerificator for LightClient {
    type D = SubstrateDigest;
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
        fn new_from_checkpoint(checkpoint: TrustedCheckpoint) -> Self {
            let head = LightClientBlockView::from(checkpoint);
            Self {
                storage: DummyStateStorage::new(
                    head.clone(),
                    (
                        head.inner_lite.next_epoch_id,
                        head.next_bps.as_ref().unwrap().clone(),
                    ),
                ),
            }
        }
    }

    // dummy implementation (at least for now)
    impl StateStorage for MockLightClient {
        fn get_head(&self) -> &LightClientBlockView {
            self.storage.get_head()
        }

        fn set_new_head(&mut self, new_head: LightClientBlockView) {
            self.storage.set_new_head(new_head)
        }

        fn get_epoch_block_producers(
            &self,
        ) -> &BTreeMap<CryptoHash, Vec<crate::types::ValidatorStakeView>> {
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
    fn test_mock_light_new_from_checkpoint() {
        let mut mock_light_client =
            MockLightClient::new_from_checkpoint(TrustedCheckpoint::new_for_test());

        let block_view = LightClientBlockView::new_for_test();
        assert!(mock_light_client
            .validate_and_update_head(&block_view)
            .unwrap());
    }
}
