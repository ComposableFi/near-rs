use std::collections::HashMap;

use crate::types::{CryptoHash, LightClientBlockView, Signature, ValidatorStakeView};

pub trait StateStorage {
    fn get_head(&self) -> &LightClientBlockView;
    fn get_head_mut(&mut self) -> &mut LightClientBlockView;
    fn get_epoch_block_producers(&self) -> &HashMap<CryptoHash, Vec<ValidatorStakeView>>;
    fn insert_epoch_block_producers(&mut self, epoch: CryptoHash, bps: Vec<ValidatorStakeView>);
}
// #[cfg(test)] // TODO put back when there is another impl of StateStorage
pub struct DummyStateStorage {
    head: LightClientBlockView,
    /// set of validators that can sign a mined block
    block_producers_per_epoch: HashMap<CryptoHash, Vec<Option<Signature>>>,
}

// #[cfg(test)]
impl DummyStateStorage {
    pub fn new(
        head: LightClientBlockView,
        epoch_block_producers: (CryptoHash, Vec<Option<Signature>>),
    ) -> Self {
        Self {
            head,
            block_producers_per_epoch: [epoch_block_producers].into_iter().collect(),
        }
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

    fn insert_epoch_block_producers(&mut self, _epoch: CryptoHash, _bps: Vec<ValidatorStakeView>) {
        todo!()
    }
}
