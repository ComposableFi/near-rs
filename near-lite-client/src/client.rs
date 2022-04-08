use std::marker::PhantomData;

use crate::{signature::SignatureVerificatiion, storage::StateStorage};

pub struct LightClient<S: StateStorage, V: SignatureVerificatiion> {
    _s: PhantomData<S>,
    _v: PhantomData<V>,
}

pub trait StateTransitionTracker {
    type V: SignatureVerificatiion;
    type S: StateStorage;

    fn validate_and_update_head(&mut self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{signature::DummyVerificator, storage::DummyStateStorage};

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

    impl<S: StateStorage, V: SignatureVerificatiion> StateTransitionTracker for MockLightClient<S, V> {
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
