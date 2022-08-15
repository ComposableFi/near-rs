use crate::{
    block_validation::Digest,
    types::{ LiteClientResult,  ConversionError}, error::NearLiteClientError,
};
use borsh::BorshSerialize;
use near_primitives::{merkle::{MerklePathItem, Direction}, hash::CryptoHash};
use sp_std::vec::Vec;

pub fn compute_root_from_path<D: Digest>(
    path: &Vec<MerklePathItem>,
    item_hash: CryptoHash,
) -> LiteClientResult<CryptoHash> {
    let mut res = item_hash;
    for item in path {
        match item.direction {
            Direction::Left => {
                res = combine_hash::<D>(&item.hash, &res)?;
            }
            Direction::Right => {
                res = combine_hash::<D>(&res, &item.hash)?;
            }
        }
    }
    Ok(res)
}

pub fn combine_hash<D: Digest>(
    hash1: &CryptoHash,
    hash2: &CryptoHash,
) -> LiteClientResult<CryptoHash> {
    Ok(CryptoHash::try_from(
        D::digest(&(hash1, hash2).try_to_vec()?).as_slice(),
    ).map_err(|e| NearLiteClientError::Conversion(ConversionError(e.to_string())))?)
}
