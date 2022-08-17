use crate::LiteClientResult;
use borsh::BorshSerialize;
use near_primitives_wasm_friendly::{Direction, HostFunctions, MerkleHash, MerklePathItem};
use sp_std::vec::Vec;

pub fn compute_root_from_path<H: HostFunctions>(
	path: &Vec<MerklePathItem>,
	item_hash: MerkleHash,
) -> LiteClientResult<MerkleHash> {
	let mut res = item_hash;
	for item in path {
		match item.direction {
			Direction::Left => {
				res = combine_hash::<H>(&item.hash, &res)?;
			},
			Direction::Right => {
				res = combine_hash::<H>(&res, &item.hash)?;
			},
		}
	}
	Ok(res)
}

pub fn combine_hash<H: HostFunctions>(
	hash1: &MerkleHash,
	hash2: &MerkleHash,
) -> LiteClientResult<MerkleHash> {
	Ok(MerkleHash::try_from(H::sha256(&(hash1, hash2).try_to_vec()?).as_slice())?)
}
