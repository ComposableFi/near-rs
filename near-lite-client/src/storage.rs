pub trait StateStorage {}

#[cfg(test)]
pub struct DummyStateStorage {}

#[cfg(test)]
impl StateStorage for DummyStateStorage {}
