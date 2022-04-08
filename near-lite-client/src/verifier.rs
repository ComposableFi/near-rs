use crate::{signature::SignatureVerificatiion, storage::StateStorage};

pub trait StateTransitionVerificator {
    type V: SignatureVerificatiion;
    type S: StateStorage;

    fn validate_and_update_head(&mut self) -> bool;
}
