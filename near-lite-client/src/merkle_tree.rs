use crate::{
    block_validation::Digest,
    types::{Direction, MerkleHash, MerklePath},
};
use borsh::BorshSerialize;

pub fn compute_root_from_path<D: Digest>(path: &MerklePath, item_hash: MerkleHash) -> MerkleHash {
    let mut res = item_hash;
    for item in path {
        match item.direction {
            Direction::Left => {
                res = combine_hash::<D>(&item.hash, &res);
            }
            Direction::Right => {
                res = combine_hash::<D>(&res, &item.hash);
            }
        }
    }
    res
}

pub fn combine_hash<D: Digest>(hash1: &MerkleHash, hash2: &MerkleHash) -> MerkleHash {
    // TODO: error management
    MerkleHash::try_from(D::digest(&(hash1, hash2).try_to_vec().unwrap()).as_slice()).unwrap()
}
