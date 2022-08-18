//! Most of this code has been ported from NEAR CORE (near-store)
//! verbatim as much as possible. There has been some tweaks in order
//! to make it no_std compatible.

use near_primitives_wasm_friendly::{CryptoHash, HostFunctions};
use no_std_compat as std;
use std::{string::String, vec::Vec};

use core2::io::{Cursor, Read};

use crate::nibble::NibbleSlice;

#[derive(Debug, Eq, PartialEq)]
pub struct RawTrieNodeWithSize {
	node: RawTrieNode,
	memory_usage: u64,
}

#[derive(Debug, Eq, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum RawTrieNode {
	Leaf(Vec<u8>, u32, CryptoHash),
	Branch([Option<CryptoHash>; 16], Option<(u32, CryptoHash)>),
	Extension(Vec<u8>, CryptoHash),
}

const LEAF_NODE: u8 = 0;
const BRANCH_NODE_NO_VALUE: u8 = 1;
const BRANCH_NODE_WITH_VALUE: u8 = 2;
const EXTENSION_NODE: u8 = 3;

/// ReadBytesExtLittleEndian parses anythign that implements `Read` and converts
/// the data into primitives such as u8, u16, u32.
/// ## Assumption
/// ReadBytesExtLittleEndian assumes data coming in LittleEndian
trait ReadBytesExtLittleEndian: Read {
	fn read_u8(&mut self) -> Result<u8, String> {
		let mut buff = [0u8; 1];
		self.take(1)
			.read_exact(&mut buff)
			.map_err(|e| String::from("Could not read two bytes"))?;
		Ok(u8::from_le_bytes(buff))
	}
	fn read_u16(&mut self) -> Result<u16, String> {
		let mut buff = [0u8; 2];
		self.take(2)
			.read_exact(&mut buff)
			.map_err(|e| String::from("Could not read two bytes"))?;
		Ok(u16::from_le_bytes(buff))
	}
	fn read_u32(&mut self) -> Result<u32, String> {
		let mut buff = [0u8; 4];
		self.take(4)
			.read_exact(&mut buff)
			.map_err(|e| String::from("Could not read two bytes"))?;
		Ok(u32::from_le_bytes(buff))
	}
}

impl<T: core::convert::AsRef<[u8]>> ReadBytesExtLittleEndian for Cursor<T> {}

// ported from NEAR CORE: decodes bytes into children nods of the state trie
fn decode_children(cursor: &mut Cursor<&[u8]>) -> Result<[Option<CryptoHash>; 16], String> {
	let mut children: [Option<CryptoHash>; 16] = Default::default();
	let bitmap = cursor.read_u16().map_err(|_| String::from("decoding error"))?;
	let mut pos = 1;
	for child in &mut children {
		if bitmap & pos != 0 {
			let mut arr = [0; 32];
			cursor.read_exact(&mut arr).map_err(|_| String::from("decoding error"))?;
			*child = Some(CryptoHash::try_from(&arr[..]).unwrap());
		}
		pos <<= 1;
	}
	Ok(children)
}

impl RawTrieNodeWithSize {
	fn encode_into(&self, out: &mut Vec<u8>) {
		self.node.encode_into(out);
		out.extend(self.memory_usage.to_le_bytes());
	}

	/// decode is used to convert the proof that is sent by the RPC into `RawTrieNodeWithSize`
	pub fn decode(bytes: &[u8]) -> Result<Self, String> {
		if bytes.len() < 8 {
			return Err("Wrong type".into());
		}
		let node = RawTrieNode::decode(&bytes[0..bytes.len() - 8])?;
		let mut arr: [u8; 8] = Default::default();
		arr.copy_from_slice(&bytes[bytes.len() - 8..]);
		let memory_usage = u64::from_le_bytes(arr);
		Ok(RawTrieNodeWithSize { node, memory_usage })
	}
}

impl RawTrieNode {
	fn encode_into(&self, out: &mut Vec<u8>) {
		// size in state_parts = size + 8 for RawTrieNodeWithSize + 8 for borsh vector length
		match &self {
			// size <= 1 + 4 + 4 + 32 + key_length + value_length
			RawTrieNode::Leaf(key, value_length, value_hash) => {
				out.push(LEAF_NODE);
				out.extend((key.len() as u32).to_le_bytes());
				out.extend(key);
				out.extend((*value_length as u32).to_le_bytes());
				out.extend(value_hash.as_bytes());
			},
			// size <= 1 + 4 + 32 + value_length + 2 + 32 * num_children
			RawTrieNode::Branch(children, value) => {
				if let Some((value_length, value_hash)) = value {
					out.push(BRANCH_NODE_WITH_VALUE);
					out.extend((*value_length as u32).to_le_bytes());
					out.extend(value_hash.as_bytes());
				} else {
					out.push(BRANCH_NODE_NO_VALUE);
				}
				let mut bitmap: u16 = 0;
				let mut pos: u16 = 1;
				for child in children.iter() {
					if child.is_some() {
						bitmap |= pos
					}
					pos <<= 1;
				}
				out.extend(bitmap.to_le_bytes());
				for child in children.iter() {
					if let Some(hash) = child {
						out.extend(hash.as_bytes());
					}
				}
			},
			// size <= 1 + 4 + key_length + 32
			RawTrieNode::Extension(key, child) => {
				out.push(EXTENSION_NODE);
				out.extend((key.len() as u32).to_le_bytes());
				out.extend(key);
				out.extend(child.as_bytes());
			},
		}
	}

	fn decode(bytes: &[u8]) -> Result<Self, String> {
		let mut cursor = Cursor::new(bytes);
		match cursor.read_u8().map_err(|_| String::from("decoding error"))? {
			LEAF_NODE => {
				let key_length = cursor.read_u32().map_err(|_| String::from("decoding error"))?;
				let mut key = (0..key_length as u8).into_iter().collect::<Vec<_>>();
				cursor
					.read_exact(&mut key)
					.map(|err| err.into())
					.map_err(|_| String::from("decoding error"))?;
				let value_length = cursor.read_u32().map_err(|_| String::from("decoding error"))?;
				let mut arr = [0; 32];
				cursor.read_exact(&mut arr).map_err(|_| String::from("decoding error"))?;
				let value_hash = CryptoHash(arr);
				Ok(RawTrieNode::Leaf(key, value_length, value_hash))
			},
			BRANCH_NODE_NO_VALUE => {
				let children = decode_children(&mut cursor)?;
				Ok(RawTrieNode::Branch(children, None))
			},
			BRANCH_NODE_WITH_VALUE => {
				let value_length = cursor.read_u32().map_err(|_| String::from("decoding error"))?;
				let mut arr = [0; 32];
				cursor.read_exact(&mut arr).map_err(|_| String::from("decoding error"))?;
				let value_hash = CryptoHash(arr);
				let children = decode_children(&mut cursor)?;
				Ok(RawTrieNode::Branch(children, Some((value_length, value_hash))))
			},
			EXTENSION_NODE => {
				let key_length = cursor.read_u32().map_err(|_| String::from("decoding error"))?;
				let mut key = (0..key_length as u8).into_iter().collect::<Vec<_>>();
				cursor.read_exact(&mut key).map_err(|_| String::from("decoding error"))?;
				let mut child = [0; 32];
				cursor.read_exact(&mut child).map_err(|_| String::from("decoding error"))?;
				Ok(RawTrieNode::Extension(key, CryptoHash(child)))
			},
			_ => Err("Wrong type".into()),
		}
	}

	fn get_key(&self) -> Option<&[u8]> {
		match self {
			RawTrieNode::Leaf(key, _, _) => Some(key),
			RawTrieNode::Extension(key, _) => Some(key),
			RawTrieNode::Branch(_, _) => None,
		}
	}
}

/// Verifies proof of membership and non membership of state proofs.
pub fn verify_state_proof<H: HostFunctions>(
	key: &[u8],
	levels: &Vec<RawTrieNodeWithSize>,
	// when verifying proofs of non-membership, this value should be None
	maybe_expected_value: Option<&[u8]>,
	mut expected_hash: CryptoHash,
) -> bool {
	let mut v = Vec::new();
	let mut hash_node = |node: &RawTrieNodeWithSize| {
		v.clear();
		node.encode_into(&mut v);
		CryptoHash::from_raw(&H::sha256(&v))
	};
	let mut hash = CryptoHash::default();
	let mut key = NibbleSlice::new(key);
	if levels.is_empty() && maybe_expected_value.is_some() {
		return false;
	}
	for node in levels.iter() {
		match node {
			RawTrieNodeWithSize { node: RawTrieNode::Leaf(_, _, value_hash), .. } => {
				hash = hash_node(&node);
				if hash != expected_hash {
					return false;
				}

				let node_key = node.node.get_key().expect("we've just wrapped the value; qed");
				let nib = &NibbleSlice::from_encoded(&node_key).0;
				if &key != nib {
					return maybe_expected_value.is_none();
				}

				return if let Some(value) = maybe_expected_value {
					CryptoHash::hash_bytes(value) == *value_hash
				} else {
					false
				};
			},
			RawTrieNodeWithSize { node: RawTrieNode::Extension(_, child_hash), .. } => {
				hash = hash_node(&node);
				if hash != expected_hash {
					return false;
				}
				expected_hash = *child_hash;

				// To avoid unnecessary copy
				let node_key = node.node.get_key().expect("we've just wrapped the value; qed");
				let nib = NibbleSlice::from_encoded(&node_key).0;
				if !key.starts_with(&nib) {
					return maybe_expected_value.is_none();
				}
				key = key.mid(nib.len());
			},
			RawTrieNodeWithSize { node: RawTrieNode::Branch(children, value), .. } => {
				hash = hash_node(&node);
				if hash != expected_hash {
					return false;
				}

				if key.is_empty() {
					// TODO: validate value size?
					let maybe_value = value.map(|x| x.1);
					let maybe_expected_value = maybe_expected_value.map(CryptoHash::hash_bytes);
					return match (maybe_expected_value, maybe_value) {
						(Some(expected_value), Some(value)) => expected_value == value,
						(None, Some(_)) => false,
						(Some(_), None) => false,
						(None, None) => true,
					};
				}
				let index = key.at(0);
				match &children[index as usize] {
					Some(child_hash) => {
						key = key.mid(1);
						expected_hash = *child_hash;
					},
					None => return maybe_expected_value.is_none(),
				}
			},
		}
	}
	maybe_expected_value.is_none() && hash == expected_hash
}

#[cfg(test)]
mod tests {

	use core::str::FromStr;

	use near_primitives::hash::CryptoHash as NearCryptoHash;

	struct MockedHostFunctions;
	impl HostFunctions for MockedHostFunctions {
		fn sha256(data: &[u8]) -> [u8; 32] {
			use sha2::Digest;
			sha2::Sha256::digest(data).to_vec().try_into().unwrap()
		}

		fn verify(
			signature: near_primitives_wasm_friendly::Signature,
			data: impl AsRef<[u8]>,
			public_key: near_primitives_wasm_friendly::PublicKey,
		) -> bool {
			todo!()
		}
	}

	use super::*;
	#[test]
	fn verify_proof() {
		// these examples were taken from NEAR CORE
		// but since the implementation had to be modified to be no_std, we're validating it.
		let key = b"doge";
		let raw_proof = [
            "0301000000165a1e73ea8e3686db1c5938a8d22912c2e2936fe475d933c310c452cbfb4306ff5303000000000000",
            "011001aa33bd98698cbe05e77f027fc9bd32783d433a9a9f6213e80a523dd82e7fab108d0c9da6ee86e6957cd608f5a9bac9b9069ceaf555431744a4b09311ad62898e1f03000000000000",
            "0302000000006f7240cc9614e62849c6e37a9b74e3e46707c42e517061622d45a1c62849d744d31002000000000000",
            "02040000006806f86a17f0ae04d4ad43168fcdd0651c0fe99108b992f5ac398fd6bf235331400044a09c32d7c008aa4cd3f44adbd62bfe01a7811f21591aef4df91634d4530d1dda01000000000000",
            "01880095581dfffd1f4f734b2c69c960e63193c18fae785d0c20d9a1c52c7c1374b5efe5fe9244cc68ea29ca00d2207f0033bb007be8aed9a00e27e23f1c84456bf02f7201000000000000",
            "02050000006588ef4db6a357d6d9ca7d0c9feb69bd8e2f236ab88459da5c193b7fa95031874000e2919fa19c4fe63ae7741fc7e42ab169792efc46b715749822df86bb74c977afd300000000000000",
            "00010000003504000000b3a1984ba0b1d8ad7f9dc881dfd9c9dc78c76c647a7692fbbfd6fcdcb9d9a1216a00000000000000"
        ].into_iter().map(|p| RawTrieNodeWithSize::decode(&hex::decode(p).unwrap()).unwrap()).collect::<Vec<_>>();

		let root_hash = CryptoHash(
			NearCryptoHash::from_str("hvKZryexWm5CPgcvB3VKxKp1uRQWZnSDALLNa9raXJV")
				.unwrap()
				.try_into()
				.unwrap(),
		);

		assert!(verify_state_proof::<MockedHostFunctions>(
			key.as_ref(),
			&raw_proof,
			Some(b"coin".as_ref()),
			root_hash
		));
		assert!(!verify_state_proof::<MockedHostFunctions>(
			key.as_ref(),
			&raw_proof,
			Some(b"coin_not_present".as_ref()),
			root_hash
		));
	}

	#[test]
	fn verify_non_membership_proof() {
		// these examples were taken from NEAR CORE
		// but since the implementation had to be modified to be no_std, we're validating it.
		let key = b"white_horse";
		let raw_proof = ["0301000000165a1e73ea8e3686db1c5938a8d22912c2e2936fe475d933c310c452cbfb4306ff5303000000000000"
        ].into_iter().map(|p| RawTrieNodeWithSize::decode(&hex::decode(p).unwrap()).unwrap()).collect::<Vec<_>>();

		let root_hash = CryptoHash(
			NearCryptoHash::from_str("hvKZryexWm5CPgcvB3VKxKp1uRQWZnSDALLNa9raXJV")
				.unwrap()
				.try_into()
				.unwrap(),
		);

		assert!(verify_state_proof::<MockedHostFunctions>(
			key.as_ref(),
			&raw_proof,
			None,
			root_hash
		));
	}
}
