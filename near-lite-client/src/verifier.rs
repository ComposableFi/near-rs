use crate::{block_validation::BlockValidation, storage::StateStorage, types::BlockView};

pub trait StateTransitionVerificator {
    type V: BlockValidation;
    type S: StateStorage;

    fn validate_and_update_head(&mut self, block_view: &BlockView) -> bool;
}
