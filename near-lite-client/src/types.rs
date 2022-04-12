pub(crate) type Signature = [u8; 32];

#[derive(Debug, Clone)]
pub enum SignatureType {
    Ed25519,
}

pub type EpochId = Vec<u8>;
pub type AccountId = String;
pub type Balance = u64;

pub type BlockProducer = (SignatureType, Signature);

type PublicKey = Vec<u8>;

#[derive(Debug, Clone)]
pub struct ValidatorStakeView {
    pub account_id: AccountId,
    pub public_key: PublicKey,
    pub stake: Balance,
}

pub struct CryptoHash([u8; 32]);

pub struct BlockView {
    // block height
    pub height: u64,

    // block epoch
    pub epoch_id: EpochId,
    // next epoch hash
    pub next_epoch_id: EpochId,

    // next epoch's block producers
    pub next_bps: Option<Vec<ValidatorStakeView>>,

    // signatures that can validate a block in the current epoch
    pub approvals_after_next: Vec<Option<BlockProducer>>,
}
