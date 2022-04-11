use crate::{block_validation::BlockValidation, storage::StateStorage};

pub trait StateTransitionVerificator {
    type V: BlockValidation;
    type S: StateStorage;

    fn validate_and_update_head(&mut self) -> bool;
}
