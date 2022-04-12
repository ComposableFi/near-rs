use std::error::Error;

use crate::block_validation::Digest;

pub(crate) type Signature = [u8; 32];

#[derive(Debug, Clone)]
pub enum SignatureType {
    Ed25519,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

impl LightClientBlockView {
    pub fn current_block_hash(&self) -> CryptoHash {
        current_block_hash(
            Sha256::digest(self.inner_lite.try_to_vec().unwrap())
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
        .as_slice(),
    )
}
