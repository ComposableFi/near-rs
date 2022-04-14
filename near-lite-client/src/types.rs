use crate::block_validation::Digest;
use borsh::{BorshDeserialize, BorshSerialize};
use sp_core::ed25519::{Public as Ed25519Public, Signature as Ed25519Signature};

#[derive(Debug, Clone)]
pub enum SignatureType {
    Ed25519,
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct PublicKey(pub [u8; 32]);
pub type Signature = Ed25519Signature;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, BorshSerialize, BorshDeserialize)]
pub struct CryptoHash([u8; 32]);

// TODO: improve error message
impl TryFrom<&[u8]> for CryptoHash {
    type Error = String;
    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() != 32 {
            return Err("wrong size".into());
        }
        let inner: [u8; 32] = v.try_into().unwrap();
        Ok(CryptoHash(inner))
    }
}

impl AsRef<[u8]> for CryptoHash {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl From<&PublicKey> for Ed25519Public {
    fn from(pubkey: &PublicKey) -> Ed25519Public {
        Ed25519Public(pubkey.0.clone())
    }
}
pub struct EpochId(pub CryptoHash);
pub type BlockHeight = u64;
pub type AccountId = String;
pub type Balance = u64;

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

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
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

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub enum ApprovalInner {
    Endorsement(CryptoHash),
    Skip(BlockHeight),
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct ValidatorStakeView {
    pub account_id: AccountId,
    pub public_key: PublicKey,
    pub stake: Balance,
}

impl LightClientBlockView {
    pub fn current_block_hash<D: Digest>(&self) -> CryptoHash {
        current_block_hash::<D>(
            D::digest(self.inner_lite.try_to_vec().unwrap())
                .as_slice()
                .try_into()
                .unwrap(),
            self.inner_rest_hash,
            self.prev_block_hash,
        )
    }
    #[cfg(test)]
    pub fn new_for_test() -> Self {
        Self {
            prev_block_hash: CryptoHash([0; 32]),
            next_block_inner_hash: CryptoHash([0; 32]),
            inner_lite: BlockHeaderInnerLiteView::new_for_test(),
            inner_rest_hash: CryptoHash([0; 32]),
            next_bps: None,
            approvals_after_next: vec![],
        }
    }
}

impl LightClientBlockLiteView {
    pub fn current_block_hash<D: Digest>(&self) -> CryptoHash {
        current_block_hash::<D>(
            D::digest(self.inner_lite.try_to_vec().unwrap())
                .as_slice()
                .try_into()
                .unwrap(),
            self.inner_rest_hash,
            self.prev_block_hash,
        )
    }
}
/// The hash of the block is:
/// ```ignore
/// sha256(concat(
///     sha256(concat(
///         sha256(borsh(inner_lite)),
///         sha256(borsh(inner_rest)) // we can use inner_rest_hash as well
///     )
/// ),
/// prev_hash
///))
/// ```
fn current_block_hash<D: Digest>(
    inner_lite_hash: CryptoHash,
    inner_rest_hash: CryptoHash,
    prev_block_hash: CryptoHash,
) -> CryptoHash {
    CryptoHash(
        D::digest(
            [
                D::digest([inner_lite_hash.as_ref(), inner_rest_hash.as_ref()].concat()).as_ref(),
                prev_block_hash.as_ref(),
            ]
            .concat(),
        )
        .as_slice()
        .try_into()
        .unwrap(),
    )
}

impl BlockHeaderInnerLiteView {
    #[cfg(test)]
    pub fn new_for_test() -> Self {
        Self {
            height: 1,
            epoch_id: CryptoHash([0; 32]),
            next_epoch_id: CryptoHash([0; 32]),
            prev_state_root: CryptoHash([0; 32]),
            outcome_root: CryptoHash([0; 32]),
            timestamp: 1,
            next_bp_hash: CryptoHash([0; 32]),
            block_merkle_root: CryptoHash([0; 32]),
        }
    }
}
