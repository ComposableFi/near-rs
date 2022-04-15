use std::collections::HashMap;

use crate::types::{CryptoHash, LightClientBlockView, ValidatorStakeView};

pub trait StateStorage {
    fn get_head(&self) -> &LightClientBlockView;
    fn get_head_mut(&mut self) -> &mut LightClientBlockView;
    fn get_epoch_block_producers(&self) -> &HashMap<CryptoHash, Vec<ValidatorStakeView>>;
    fn get_epoch_block_producers_mut(
        &mut self,
    ) -> &mut HashMap<CryptoHash, Vec<ValidatorStakeView>>;
}
// #[cfg(test)] // TODO put back when there is another impl of StateStorage
pub struct DummyStateStorage {
    head: LightClientBlockView,
}

// #[cfg(test)]
impl DummyStateStorage {
    pub fn new(head: LightClientBlockView) -> Self {
        Self { head: head }
    }
}
// #[cfg(test)]
impl StateStorage for DummyStateStorage {
    fn get_head(&self) -> &LightClientBlockView {
        &self.head
    }

    fn get_head_mut(&mut self) -> &mut LightClientBlockView {
        &mut self.head
    }

    fn get_epoch_block_producers(&self) -> &HashMap<CryptoHash, Vec<ValidatorStakeView>> {
        todo!()
    }

    fn get_epoch_block_producers_mut(
        &mut self,
    ) -> &mut HashMap<CryptoHash, Vec<ValidatorStakeView>> {
        todo!()
    }
}
