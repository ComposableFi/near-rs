use std::{collections::HashMap, marker::PhantomData};

use crate::{
    signature::{Signature, SignatureVerificatiion},
    storage::StateStorage,
};

#[allow(dead_code)]
type EpochId = usize;

#[allow(dead_code)]
type BlockProducer = Signature;

/// LightClient keeps track of at least one block per epoch, the set of validators
/// in each relevant epoch (depends on how much state wants to be stored -- configurable).
/// It is also able to verify a new state transition, and update its head.
#[allow(dead_code)]
pub struct LightClient<S: StateStorage, V: SignatureVerificatiion> {
    /// how many epochs the light client will track
    relevant_epochs: usize,

    /// set of validators that can sign a mined block
    block_producers_per_epoch: HashMap<EpochId, Vec<BlockProducer>>,

    _s: PhantomData<S>,
    _v: PhantomData<V>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        signature::DummyVerificator, storage::DummyStateStorage,
        verifier::StateTransitionVerificator,
    };

    struct MockLightClient<S: StateStorage, V: SignatureVerificatiion> {
        _s: PhantomData<S>,
        _v: PhantomData<V>,
    }

    impl<S: StateStorage, V: SignatureVerificatiion> MockLightClient<S, V> {
        fn new() -> Self {
            Self {
                _s: PhantomData::default(),
                _v: PhantomData::default(),
            }
        }
    }

    impl<S: StateStorage, V: SignatureVerificatiion> StateTransitionVerificator
        for MockLightClient<S, V>
    {
        type V = V;
        type S = S;

        fn validate_and_update_head(&mut self) -> bool {
            true
        }
    }

    #[test]
    fn test_mock_light_client() {
        let mut mock_light_client = MockLightClient::<DummyStateStorage, DummyVerificator>::new();
        assert!(mock_light_client.validate_and_update_head());
    }
}
