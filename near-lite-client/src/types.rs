pub(crate) type Signature = [u8; 32];

#[derive(Debug, Clone)]
pub enum SignatureType {
    Ed25519,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct CryptoHash([u8; 32]);

pub struct EpochId(pub CryptoHash);
pub type BlockHeight = u64;
pub type AccountId = String;
pub type Balance = u64;

type PublicKey = Vec<u8>;

#[derive(Debug, Clone)]
pub struct LightClientBlockLiteView {
    pub prev_block_hash: CryptoHash,
    pub inner_rest_hash: CryptoHash,
    pub inner_lite: BlockHeaderInnerLiteView,
}

#[derive(Debug, Clone)]
pub struct LightClientBlockView {
    pub prev_block_hash: CryptoHash,
    pub next_block_inner_hash: CryptoHash,
    pub inner_lite: BlockHeaderInnerLiteView,
    pub inner_rest_hash: CryptoHash,
    pub next_bps: Option<Vec<ValidatorStakeView>>,
    pub approvals_after_next: Vec<Option<Signature>>,
}

#[derive(Debug, Clone)]
pub struct BlockHeaderInnerLiteView {
    pub height: BlockHeight,
    pub epoch_id: CryptoHash,
    pub next_epoch_id: CryptoHash,
    pub prev_state_root: CryptoHash,
    pub outcome_root: CryptoHash,
    pub timestamp: u64,
    pub next_bp_hash: CryptoHash,
    pub block_merkle_root: CryptoHash,
}

pub enum ApprovalInner {
    Endorsement(CryptoHash),
    Skip(BlockHeight),
}

#[derive(Debug, Clone)]
pub struct ValidatorStakeView {
    pub account_id: AccountId,
    pub public_key: PublicKey,
    pub stake: Balance,
}
