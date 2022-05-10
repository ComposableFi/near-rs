use sp_std::{collections::btree_map::BTreeMap, vec::Vec};

use crate::types::{CryptoHash, LightClientBlockView, ValidatorStakeView};

pub trait StateStorage {
    fn get_head() -> &LightClientBlockView;
    fn set_new_head(new_head: LightClientBlockView);
    fn get_epoch_block_producers() -> &BTreeMap<CryptoHash, Vec<ValidatorStakeView>>;
    fn insert_epoch_block_producers(epoch: CryptoHash, bps: Vec<ValidatorStakeView>);
}
// #[cfg(test)] // TODO put back when there is another impl of StateStorage
pub struct DummyStateStorage {
    head: LightClientBlockView,
    #[allow(dead_code)]
    block_producers_per_epoch: BTreeMap<CryptoHash, Vec<ValidatorStakeView>>,
}

// #[cfg(test)]
impl DummyStateStorage {
    pub fn new(
        head: LightClientBlockView,
        epoch_block_producers: (CryptoHash, Vec<ValidatorStakeView>),
    ) -> Self {
        Self {
            head,
            block_producers_per_epoch: [epoch_block_producers].into_iter().collect(),
        }
    }
}
// #[cfg(test)]
impl StateStorage for DummyStateStorage {
    fn get_head() -> &LightClientBlockView {
        &self.head
    }

    fn set_new_head(&mut self, new_head: LightClientBlockView) {
        self.head = new_head;
    }

    fn get_epoch_block_producers(&self) -> &BTreeMap<CryptoHash, Vec<ValidatorStakeView>> {
        &self.block_producers_per_epoch
    }

    fn insert_epoch_block_producers(&mut self, epoch: CryptoHash, bps: Vec<ValidatorStakeView>) {
        self.block_producers_per_epoch.insert(epoch, bps);
    }
}
