//! # Batch Merkle Proofs
//!
//! ## Introduction
//! The purpose of this create is to allow light clients to verify proofs in batches.
//!

#![cfg_attr(not(feature = "std"), no_std)]

extern crate no_std_compat as std;

use core::marker::PhantomData;
use std::{collections::HashMap, vec::Vec};
mod host_functions;
use borsh::BorshSerialize;
use host_functions::HostFunctions;

use near_primitives::{
    hash::CryptoHash,
    merkle::{Direction, MerklePath},
};

type Level = usize;
type Index = usize;
type LeafIndex = usize;

/// ProofBatchVerifier verifies merkle proofs and maintains a cache
/// of intermediate computations to avoid having to spend too many
/// CPU cycles in vain.
///
/// ## Note: it's important that all the proofs belong to the same shard.
#[derive(Debug, PartialEq, Eq)]
pub struct ProofBatchVerifier<HF: HostFunctions> {
    cached_nodes: CachedNodes,
    _hf: PhantomData<HF>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NodeCoordinates {
    index: usize,
    level: usize,
    hash: Option<CryptoHash>,
}

#[derive(Debug, PartialEq, Eq)]
struct CachedNodes {
    inner: HashMap<(Level, Index), CryptoHash>,
    path_item_cache_mapping: HashMap<LeafIndex, Vec<(Level, Index)>>,
}

impl CachedNodes {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
            path_item_cache_mapping: HashMap::new(),
        }
    }

    fn extend_from_given(&mut self, given_nodes: &[NodeCoordinates], leaf_index: LeafIndex) {
        if given_nodes.len() == 0 {
            return;
        }

        given_nodes.iter().for_each(|node| {
            let NodeCoordinates { index, level, hash } = node;
            if self.inner.get(&(*level, *index)).is_some() {
                return;
            }
            self.inner.insert((*level, *index), hash.unwrap());
            let e = self
                .path_item_cache_mapping
                .entry(leaf_index)
                .or_insert_with(Vec::new);
            e.push((*level, *index));
        });
    }
}

impl<HF: HostFunctions> ProofBatchVerifier<HF> {
    pub fn new() -> Self {
        Self {
            cached_nodes: CachedNodes::new(),
            _hf: PhantomData::default(),
        }
    }

    /// Computes the root hash of a given merkle proof and item hash
    /// It will update the cache of intermediate nodes so that they do not have
    /// to be recomputed
    pub fn calculate_root_hash(&mut self, proof: &MerklePath, item_hash: CryptoHash) -> CryptoHash {
        // trivial example, where proof is empty
        if proof.len() == 0 {
            return CryptoHash::default();
        }

        // the first element is somewhat different, since the caller is passing the item's hash
        let (_, node_coordinates_to_calculate) = self.get_node_coordinates(proof);
        let nodes_to_calculate = node_coordinates_to_calculate.len();

        let sibling_item = &proof[0];

        // calculate the hash for the leaf level by hashing the item_hash given and its sibling (provided in the proof)
        let hash = match sibling_item.direction {
            Direction::Left => hash_borsh::<_, HF>(&(sibling_item.hash, item_hash)),
            Direction::Right => hash_borsh::<_, HF>(&(item_hash, sibling_item.hash)),
        };

        let NodeCoordinates { index, level, .. } =
            &node_coordinates_to_calculate[nodes_to_calculate - 0 - 1];
        let cached_value = self.cached_nodes.inner.get(&(*level, *index));

        match cached_value {
            None => {
                self.cached_nodes.inner.insert((*level, *index), hash);
            }
            Some(parent_hash) => {
                // ensure that, if the value was cached it matches the calculation made above
                // this is important, otherwise when most of the intermediates nodes are cached, if this check
                // is not made, a wrong proof could be passed and stil "yield" the right root hash
                assert_eq!(parent_hash, &hash);
            }
        }

        proof
            .iter()
            .enumerate()
            .skip(1) // skip the parent
            .fold(hash, |mut hash, (item_idx, merkle_path_item)| {
                let NodeCoordinates { index, level, .. } =
                    &node_coordinates_to_calculate[nodes_to_calculate - item_idx - 1];

                let cached_value = self.cached_nodes.inner.get(&(*level, *index));
                match cached_value {
                    None => {
                        match merkle_path_item.direction {
                            Direction::Left => {
                                hash = CryptoHash::hash_borsh(&(merkle_path_item.hash, hash))
                            }
                            Direction::Right => {
                                hash = CryptoHash::hash_borsh(&(hash, merkle_path_item.hash))
                            }
                        };
                        // update the cache
                        self.cached_nodes.inner.insert((*level, *index), hash);
                    }
                    Some(cached_value) => {
                        hash = *cached_value;
                    }
                }

                hash
            })
    }

    /// Updates the cache with all the values that are given on a merkle proof
    pub fn update_cache<'a>(&mut self, proofs: impl Iterator<Item = &'a MerklePath>) {
        proofs.for_each(|proof| {
            let (given_nodes, _) = self.get_node_coordinates(proof);
            let leaf_index = given_nodes.last().unwrap().index;
            self.cached_nodes
                .extend_from_given(&given_nodes[0..(given_nodes.len() - 1)], leaf_index);
        });
    }

    pub fn get_node_coordinates(
        &self,
        proof: &MerklePath,
    ) -> (Vec<NodeCoordinates>, Vec<NodeCoordinates>) {
        let tree_depth = proof.len();
        proof
            .iter()
            .rev()
            .fold(
                ((Vec::new(), Vec::new()), 0, 0, 0),
                |(
                    (mut node_coordinates_given, mut node_coordinates_to_calculate),
                    mut depth,
                    mut idx_given,
                    mut idx_to_calculate,
                ),
                 el| {
                    depth += 1;
                    match depth {
                        1 => {
                            node_coordinates_to_calculate.push(NodeCoordinates {
                                index: 0,
                                level: 0,
                                hash: None,
                            });

                            match el.direction {
                                Direction::Left => {
                                    idx_to_calculate = 1;
                                }
                                Direction::Right => {
                                    idx_given = 1;
                                    idx_to_calculate = 0;
                                }
                            }
                            // edge case depth == 1
                            node_coordinates_given.push(NodeCoordinates {
                                index: idx_given,
                                level: depth,
                                hash: Some(el.hash),
                            });
                            if depth == tree_depth {
                                node_coordinates_given.push(NodeCoordinates {
                                    index: idx_given ^ 1,
                                    level: depth,
                                    hash: Some(el.hash),
                                });
                            } else {
                                node_coordinates_to_calculate.push(NodeCoordinates {
                                    index: idx_to_calculate,
                                    level: depth,
                                    hash: None,
                                });
                            }
                        }
                        depth if depth == tree_depth => {
                            idx_to_calculate *= 2;
                            idx_given = idx_to_calculate;
                            // both nodes are given on the leaf level
                            node_coordinates_given.push(NodeCoordinates {
                                index: idx_given,
                                level: depth,
                                hash: Some(el.hash),
                            });
                            node_coordinates_given.push(NodeCoordinates {
                                index: idx_given ^ 1,
                                level: depth,
                                hash: Some(el.hash),
                            })
                        }
                        depth => {
                            // move to the children
                            idx_to_calculate *= 2;
                            idx_given = idx_to_calculate;
                            match el.direction {
                                Direction::Left => {
                                    idx_to_calculate ^= 1;
                                }
                                Direction::Right => {
                                    idx_given ^= 1;
                                }
                            }
                            node_coordinates_given.push(NodeCoordinates {
                                index: idx_given,
                                level: depth,
                                hash: Some(el.hash),
                            });
                            node_coordinates_to_calculate.push(NodeCoordinates {
                                index: idx_to_calculate,
                                level: depth,
                                hash: None,
                            });
                        }
                    };
                    (
                        (node_coordinates_given, node_coordinates_to_calculate),
                        depth,
                        idx_given,
                        idx_to_calculate,
                    )
                },
            )
            .0
    }
}

fn hash_borsh<T: BorshSerialize, HF: HostFunctions>(items: &(T, T)) -> CryptoHash {
    let serialized = items.try_to_vec().unwrap();
    CryptoHash(HF::sha256(&serialized))
}

#[cfg(test)]
mod tests {
    use near_primitives::merkle::{compute_root_from_path_and_item, merklize, MerklePathItem};

    use super::*;

    struct ExpectedResult {
        node_coordinates_given: Vec<NodeCoordinates>,
        node_coordinates_to_calculate: Vec<NodeCoordinates>,
    }

    impl From<ExpectedResult> for (Vec<NodeCoordinates>, Vec<NodeCoordinates>) {
        fn from(e: ExpectedResult) -> Self {
            (e.node_coordinates_given, e.node_coordinates_to_calculate)
        }
    }

    struct MockedHostFunctions;
    impl HostFunctions for MockedHostFunctions {
        fn sha256(data: &[u8]) -> [u8; 32] {
            use sha2::Digest;
            sha2::Sha256::digest(data).try_into().unwrap()
        }
    }

    #[test]
    fn test_get_nodes_to_be_calculated() {
        let cases = [
            (
                [MerklePathItem {
                    direction: Direction::Left,
                    hash: CryptoHash::default(),
                }]
                .into_iter()
                .collect(),
                (ExpectedResult {
                    node_coordinates_given: [
                        NodeCoordinates {
                            index: 0,
                            level: 1,
                            hash: Some(CryptoHash::default()),
                        },
                        NodeCoordinates {
                            index: 1,
                            level: 1,
                            hash: Some(CryptoHash::default()),
                        },
                    ]
                    .into_iter()
                    .collect(),
                    node_coordinates_to_calculate: [NodeCoordinates {
                        index: 0,
                        level: 0,
                        hash: None,
                    }]
                    .into_iter()
                    .collect(),
                }),
            ),
            (
                [
                    MerklePathItem {
                        direction: Direction::Right,
                        hash: CryptoHash::default(),
                    },
                    MerklePathItem {
                        direction: Direction::Left,
                        hash: CryptoHash::default(),
                    },
                ]
                .into_iter()
                .rev()
                .collect(),
                (ExpectedResult {
                    node_coordinates_given: [
                        NodeCoordinates {
                            index: 1,
                            level: 1,
                            hash: Some(CryptoHash::default()),
                        },
                        NodeCoordinates {
                            index: 0,
                            level: 2,
                            hash: Some(CryptoHash::default()),
                        },
                        NodeCoordinates {
                            index: 1,
                            level: 2,
                            hash: Some(CryptoHash::default()),
                        },
                    ]
                    .into_iter()
                    .collect(),
                    node_coordinates_to_calculate: [
                        NodeCoordinates {
                            index: 0,
                            level: 0,
                            hash: None,
                        },
                        NodeCoordinates {
                            index: 0,
                            level: 1,
                            hash: None,
                        },
                    ]
                    .into_iter()
                    .collect(),
                }),
            ),
            (
                [
                    MerklePathItem {
                        direction: Direction::Right,
                        hash: CryptoHash::default(),
                    },
                    MerklePathItem {
                        direction: Direction::Right,
                        hash: CryptoHash::default(),
                    },
                ]
                .into_iter()
                .rev()
                .collect::<Vec<_>>(),
                (ExpectedResult {
                    node_coordinates_given: [
                        NodeCoordinates {
                            index: 1,
                            level: 1,
                            hash: Some(CryptoHash::default()),
                        },
                        NodeCoordinates {
                            index: 0,
                            level: 2,
                            hash: Some(CryptoHash::default()),
                        },
                        NodeCoordinates {
                            index: 1,
                            level: 2,
                            hash: Some(CryptoHash::default()),
                        },
                    ]
                    .into_iter()
                    .collect(),
                    node_coordinates_to_calculate: [
                        NodeCoordinates {
                            index: 0,
                            level: 0,
                            hash: None,
                        },
                        NodeCoordinates {
                            index: 0,
                            level: 1,
                            hash: None,
                        },
                    ]
                    .into_iter()
                    .collect(),
                }),
            ),
            (
                [
                    MerklePathItem {
                        direction: Direction::Left,
                        hash: CryptoHash::default(),
                    },
                    MerklePathItem {
                        direction: Direction::Right,
                        hash: CryptoHash::default(),
                    },
                ]
                .into_iter()
                .rev()
                .collect(),
                (ExpectedResult {
                    node_coordinates_given: [
                        NodeCoordinates {
                            index: 0,
                            level: 1,
                            hash: Some(CryptoHash::default()),
                        },
                        NodeCoordinates {
                            index: 2,
                            level: 2,
                            hash: Some(CryptoHash::default()),
                        },
                        NodeCoordinates {
                            index: 3,
                            level: 2,
                            hash: Some(CryptoHash::default()),
                        },
                    ]
                    .into_iter()
                    .collect(),
                    node_coordinates_to_calculate: [
                        NodeCoordinates {
                            index: 0,
                            level: 0,
                            hash: None,
                        },
                        NodeCoordinates {
                            index: 1,
                            level: 1,
                            hash: None,
                        },
                    ]
                    .into_iter()
                    .collect(),
                }),
            ),
            (
                [
                    MerklePathItem {
                        direction: Direction::Left,
                        hash: CryptoHash::default(),
                    },
                    MerklePathItem {
                        direction: Direction::Right,
                        hash: CryptoHash::default(),
                    },
                    MerklePathItem {
                        direction: Direction::Right,
                        hash: CryptoHash::default(),
                    },
                ]
                .into_iter()
                .rev()
                .collect::<Vec<_>>(),
                (ExpectedResult {
                    node_coordinates_given: [
                        NodeCoordinates {
                            index: 0,
                            level: 1,
                            hash: Some(CryptoHash::default()),
                        },
                        NodeCoordinates {
                            index: 3,
                            level: 2,
                            hash: Some(CryptoHash::default()),
                        },
                        NodeCoordinates {
                            index: 4,
                            level: 3,
                            hash: Some(CryptoHash::default()),
                        },
                        NodeCoordinates {
                            index: 5,
                            level: 3,
                            hash: Some(CryptoHash::default()),
                        },
                    ]
                    .into_iter()
                    .collect(),
                    node_coordinates_to_calculate: [
                        NodeCoordinates {
                            index: 0,
                            level: 0,
                            hash: None,
                        },
                        NodeCoordinates {
                            index: 1,
                            level: 1,
                            hash: None,
                        },
                        NodeCoordinates {
                            index: 2,
                            level: 2,
                            hash: None,
                        },
                    ]
                    .into_iter()
                    .collect(),
                }),
            ),
            (
                [
                    MerklePathItem {
                        direction: Direction::Right,
                        hash: CryptoHash::default(),
                    },
                    MerklePathItem {
                        direction: Direction::Right,
                        hash: CryptoHash::default(),
                    },
                    MerklePathItem {
                        direction: Direction::Right,
                        hash: CryptoHash::default(),
                    },
                ]
                .into_iter()
                .rev()
                .collect::<Vec<_>>(),
                (ExpectedResult {
                    node_coordinates_given: [
                        NodeCoordinates {
                            index: 1,
                            level: 1,
                            hash: Some(CryptoHash::default()),
                        },
                        NodeCoordinates {
                            index: 1,
                            level: 2,
                            hash: Some(CryptoHash::default()),
                        },
                        NodeCoordinates {
                            index: 0,
                            level: 3,
                            hash: Some(CryptoHash::default()),
                        },
                        NodeCoordinates {
                            index: 1,
                            level: 3,
                            hash: Some(CryptoHash::default()),
                        },
                    ]
                    .into_iter()
                    .collect(),
                    node_coordinates_to_calculate: [
                        NodeCoordinates {
                            index: 0,
                            level: 0,
                            hash: None,
                        },
                        NodeCoordinates {
                            index: 0,
                            level: 1,
                            hash: None,
                        },
                        NodeCoordinates {
                            index: 0,
                            level: 2,
                            hash: None,
                        },
                    ]
                    .into_iter()
                    .collect(),
                }),
            ),
            (
                [
                    MerklePathItem {
                        direction: Direction::Right,
                        hash: CryptoHash::default(),
                    },
                    MerklePathItem {
                        direction: Direction::Right,
                        hash: CryptoHash::default(),
                    },
                    MerklePathItem {
                        direction: Direction::Left,
                        hash: CryptoHash::default(),
                    },
                ]
                .into_iter()
                .rev()
                .collect::<Vec<_>>(),
                (ExpectedResult {
                    node_coordinates_given: [
                        NodeCoordinates {
                            index: 1,
                            level: 1,
                            hash: Some(CryptoHash::default()),
                        },
                        NodeCoordinates {
                            index: 1,
                            level: 2,
                            hash: Some(CryptoHash::default()),
                        },
                        NodeCoordinates {
                            index: 0,
                            level: 3,
                            hash: Some(CryptoHash::default()),
                        },
                        NodeCoordinates {
                            index: 1,
                            level: 3,
                            hash: Some(CryptoHash::default()),
                        },
                    ]
                    .into_iter()
                    .collect(),
                    node_coordinates_to_calculate: [
                        NodeCoordinates {
                            index: 0,
                            level: 0,
                            hash: None,
                        },
                        NodeCoordinates {
                            index: 0,
                            level: 1,
                            hash: None,
                        },
                        NodeCoordinates {
                            index: 0,
                            level: 2,
                            hash: None,
                        },
                    ]
                    .into_iter()
                    .collect(),
                }),
            ),
        ]
        .into_iter()
        .collect::<Vec<_>>();

        let verifier = ProofBatchVerifier::<MockedHostFunctions>::new();
        for (ref mp, expected_result) in cases {
            assert_eq!(verifier.get_node_coordinates(mp), expected_result.into());
        }
    }

    #[test]
    fn test_calculate_root_hash() {
        let elements = &[1, 2, 3, 4, 5];
        let (root_hash, merkle_proofs) = merklize(elements);
        let mp = &merkle_proofs[0];
        let mp2 = &merkle_proofs[1];
        assert_eq!(compute_root_from_path_and_item(mp, &1), root_hash);
        assert_eq!(compute_root_from_path_and_item(mp2, &2), root_hash);

        let mut verifier = ProofBatchVerifier::<MockedHostFunctions>::new();

        for (idx, element) in elements.iter().enumerate() {
            let merkle_proof = &merkle_proofs[idx];
            assert_eq!(
                verifier.calculate_root_hash(merkle_proof, CryptoHash::hash_borsh(element)),
                root_hash
            );
        }

        // try with cache updated
        verifier.update_cache(merkle_proofs.iter());
        for (idx, element) in elements.iter().enumerate() {
            let merkle_proof = &merkle_proofs[idx];
            assert_eq!(
                verifier.calculate_root_hash(merkle_proof, CryptoHash::hash_borsh(element)),
                root_hash
            );
        }
    }

    #[test]
    #[should_panic]
    fn test_calculate_root_hash_wrong_items() {
        let elements = &[1, 2, 3, 4, 5];
        let (root_hash, merkle_proofs) = merklize(elements);
        let mp = &merkle_proofs[0];
        let mp2 = &merkle_proofs[1];
        assert_eq!(compute_root_from_path_and_item(mp, &1), root_hash);
        assert_eq!(compute_root_from_path_and_item(mp2, &2), root_hash);

        let mut verifier = ProofBatchVerifier::<MockedHostFunctions>::new();
        let merkle_proof = &merkle_proofs[0];
        assert_eq!(
            verifier.calculate_root_hash(merkle_proof, CryptoHash::hash_borsh(&2)),
            root_hash
        );
    }

    #[test]
    #[should_panic]
    fn test_calculate_root_hash_wrong_items_with_loaded_cache() {
        let elements = &[1, 2, 3, 4, 5];
        let (root_hash, merkle_proofs) = merklize(elements);
        let mp = &merkle_proofs[0];
        let mp2 = &merkle_proofs[1];
        assert_eq!(compute_root_from_path_and_item(mp, &1), root_hash);
        assert_eq!(compute_root_from_path_and_item(mp2, &2), root_hash);

        let mut verifier = ProofBatchVerifier::<MockedHostFunctions>::new();
        // try with cache updated
        verifier.update_cache(merkle_proofs.iter());

        let merkle_proof = &merkle_proofs[0];
        assert_eq!(
            verifier.calculate_root_hash(merkle_proof, CryptoHash::hash_borsh(&2)),
            root_hash
        );
    }
}
