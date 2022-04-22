//! # Client proof
//!
//! Validates that the proof for a certain transaction is valid

use borsh::BorshSerialize;
use near_crypto::Signature;
use near_primitives::{
    hash::CryptoHash,
    merkle::MerklePathItem,
    views::{
        validator_stake_view::ValidatorStakeView,
        BlockHeaderInnerLiteView as NearBlockHeaderInnerLiteView, ExecutionOutcomeView,
        LightClientBlockView as NearLightClientBlockView,
    },
};

use near_sdk::BlockHeight;
use sha2::{Digest, Sha256};

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, BorshSerialize)]
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

pub struct LightClientBlockLiteView {
    pub prev_block_hash: CryptoHash,
    pub inner_rest_hash: CryptoHash,
    pub inner_lite: BlockHeaderInnerLiteView,
}

#[derive(Debug)]
pub struct LightClientBlockView {
    pub prev_block_hash: CryptoHash,
    pub next_block_inner_hash: CryptoHash,
    pub inner_lite: BlockHeaderInnerLiteView,
    pub inner_rest_hash: CryptoHash,
    pub next_bps: Option<Vec<ValidatorStakeView>>,
    pub approvals_after_next: Vec<Option<Signature>>,
}

impl From<NearLightClientBlockView> for LightClientBlockView {
    fn from(near: NearLightClientBlockView) -> Self {
        Self {
            prev_block_hash: near.prev_block_hash,
            next_block_inner_hash: near.next_block_inner_hash,
            inner_lite: near.inner_lite.into(),
            inner_rest_hash: near.inner_rest_hash,
            next_bps: near.next_bps,
            approvals_after_next: near.approvals_after_next,
        }
    }
}

impl From<&LightClientBlockView> for NearLightClientBlockView {
    fn from(near: &LightClientBlockView) -> Self {
        Self {
            prev_block_hash: near.prev_block_hash,
            next_block_inner_hash: near.next_block_inner_hash,
            inner_lite: near.inner_lite.clone().into(),
            inner_rest_hash: near.inner_rest_hash,
            next_bps: near.next_bps.clone(),
            approvals_after_next: near.approvals_after_next.clone(),
        }
    }
}

impl From<NearBlockHeaderInnerLiteView> for BlockHeaderInnerLiteView {
    fn from(near: NearBlockHeaderInnerLiteView) -> Self {
        Self {
            height: near.height,
            epoch_id: near.epoch_id,
            next_epoch_id: near.next_epoch_id,
            prev_state_root: near.prev_state_root,
            outcome_root: near.outcome_root,
            timestamp: near.timestamp,
            next_bp_hash: near.next_bp_hash,
            block_merkle_root: near.block_merkle_root,
        }
    }
}

impl From<BlockHeaderInnerLiteView> for NearBlockHeaderInnerLiteView {
    fn from(internal: BlockHeaderInnerLiteView) -> Self {
        Self {
            height: internal.height,
            epoch_id: internal.epoch_id,
            next_epoch_id: internal.next_epoch_id,
            prev_state_root: internal.prev_state_root,
            outcome_root: internal.outcome_root,
            timestamp: internal.timestamp,
            next_bp_hash: internal.next_bp_hash,
            block_merkle_root: internal.block_merkle_root,
            timestamp_nanosec: 0,
        }
    }
}
impl LightClientBlockLiteView {
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
fn current_block_hash(
    inner_lite_hash: CryptoHash,
    inner_rest_hash: CryptoHash,
    prev_block_hash: CryptoHash,
) -> CryptoHash {
    Sha256::digest(
        [
            Sha256::digest([inner_lite_hash.as_ref(), inner_rest_hash.as_ref()].concat()).as_ref(),
            prev_block_hash.as_ref(),
        ]
        .concat(),
    )
    .as_slice()
    .try_into()
    .unwrap()
}

pub(crate) fn next_block_hash(
    next_block_inner_hash: CryptoHash,
    current_block_hash: CryptoHash,
) -> CryptoHash {
    Sha256::digest([next_block_inner_hash.as_ref(), current_block_hash.as_ref()].concat())
        .as_slice()
        .try_into()
        .unwrap()
}

#[derive(Debug, Clone, BorshSerialize)]
pub enum ApprovalInner {
    Endorsement(CryptoHash),
    Skip(BlockHeight),
}

pub fn reconstruct_light_client_block_view_fields(
    block_view: &LightClientBlockView,
) -> (CryptoHash, CryptoHash, Vec<u8>) {
    let current_block_hash = block_view.current_block_hash();
    let next_block_hash = next_block_hash(block_view.next_block_inner_hash, current_block_hash);
    let approval_message = [
        ApprovalInner::Endorsement(next_block_hash)
            .try_to_vec()
            .unwrap(),
        (block_view.inner_lite.height + 2)
            .to_le()
            .try_to_vec()
            .unwrap(),
    ]
    .concat();
    (current_block_hash, next_block_hash, approval_message)
}

#[derive(Debug, Deserialize)]
pub struct ClientProofResponse {
    block_header_lite: BlockHeaderInnerLiteView,
}

#[derive(Debug, Deserialize)]
struct ResultFromRpc {
    pub result: BlockHeaderLite,
}

#[derive(Debug, Deserialize)]
struct BlockHeaderLite {
    pub block_header_lite: InnerLite,
    pub outcome_proof: OutcomeProof,
    pub outcome_root_proof: Vec<MerklePathItem>,
    pub block_proof: Vec<MerklePathItem>,
}

#[derive(Debug, Deserialize)]
struct InnerLite {
    inner_lite: BlockHeaderInnerLiteView,
    pub inner_rest_hash: CryptoHash,
    pub prev_block_hash: CryptoHash,
}

#[derive(Debug, Deserialize)]
struct OutcomeProof {
    pub block_hash: CryptoHash,
    pub id: CryptoHash,
    pub proof: Vec<MerklePathItem>,
    pub outcome: ExecutionOutcomeView,
}

impl BlockHeaderLite {
    fn get_block_header_inner_lite_view(&self) -> &BlockHeaderInnerLiteView {
        &self.block_header_lite.inner_lite
    }

    fn get_light_client_block_view(&self) -> LightClientBlockLiteView {
        LightClientBlockLiteView {
            prev_block_hash: self.block_header_lite.prev_block_hash,
            inner_rest_hash: self.block_header_lite.inner_rest_hash,
            inner_lite: self.get_block_header_inner_lite_view().clone(),
        }
    }
}

fn get_block_header_lite(
    client_proof_response: &str,
) -> Result<BlockHeaderLite, serde_json::Error> {
    let result_from_rpc = serde_json::from_str::<ResultFromRpc>(client_proof_response)?;
    Ok(result_from_rpc.result)
}

fn calculate_merklelization_hashes(execution_outcome: &ExecutionOutcomeView) -> Vec<CryptoHash> {
    /*

            uint256 start = data.ptr;
        outcome.receipt_ids = new bytes32[](data.decodeU32());
        for (uint i = 0; i < outcome.receipt_ids.length; i++) {
            outcome.receipt_ids[i] = data.decodeBytes32();
        }
        outcome.gas_burnt = data.decodeU64();
        outcome.tokens_burnt = data.decodeU128();
        outcome.executor_id = data.decodeBytes();
        outcome.status = data.decodeExecutionStatus();

        outcome.merkelization_hashes = new bytes32[](1 + outcome.logs.length);
        outcome.merkelization_hashes[0] = Utils.sha256Raw(start, data.ptr - start);

        for (uint i = 0; i < outcome.logs.length; i++) {
            outcome.merkelization_hashes[i + 1] = sha256(outcome.logs[i]);
        }

    } */

    let logs_payload = vec![
        execution_outcome.receipt_ids.try_to_vec().unwrap(),
        execution_outcome.gas_burnt.try_to_vec().unwrap(),
        execution_outcome.tokens_burnt.try_to_vec().unwrap(),
        execution_outcome.executor_id.try_to_vec().unwrap(),
        execution_outcome.status.try_to_vec().unwrap(),
    ]
    .concat();

    let first_element_merkelization_hashes =
        Sha256::digest(logs_payload).as_slice().try_into().unwrap();
    execution_outcome
        .logs
        .iter()
        .fold(vec![first_element_merkelization_hashes], |mut acc, log| {
            acc.push(Sha256::digest(log).as_slice().try_into().unwrap());
            acc
        })
}

// This function is needed in order to calculate the right execution outcome hash
// Currently there is no function that calculates it in the `near-primitive` module
// hence, this is a direct port from the solidity implementation of the rainbow
// bridge written in solidity.
fn calculate_execution_outcome_hash(
    execution_outcome: &ExecutionOutcomeView,
    tx_hash: CryptoHash,
) -> CryptoHash {
    /*
    uint256 len = 1 + outcome.outcome.merkelization_hashes.length;
    https://docs.soliditylang.org/en/latest/abi-spec.html#non-standard-packed-mode
    outcome.hash = sha256(
            abi.encodePacked(
                Utils.swapBytes4(uint32(len)),
                outcome.id,
                outcome.outcome.merkelization_hashes
            )
        );
    */
    let merkelization_hashes = calculate_merklelization_hashes(execution_outcome);

    // outcome.id is the tx hash or receipt id
    // let outcome = vec![merkelization_hashes.len() as u32 + 1, tx_hash, ];
    let pack_merklelization_hashes = merkelization_hashes
        .iter()
        .flat_map(|h| h.as_ref().to_owned())
        .collect::<Vec<u8>>();

    Sha256::digest(
        [
            (merkelization_hashes.len() as u32 + 1)
                .to_le_bytes()
                .as_ref(),
            tx_hash.as_ref(),
            &pack_merklelization_hashes,
        ]
        .concat(),
    )
    .as_slice()
    .try_into()
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_primitives::merkle::compute_root_from_path;
    use near_sdk::{json_types::Base58CryptoHash, CryptoHash as JSONCryptoHash};

    const client_proof_response: &'static str = r#"
    {
        "jsonrpc": "2.0",
        "result": {
            "block_header_lite": {
                "inner_lite": {
                    "block_merkle_root": "D5nnsEuJ2WA4Fua4QJWXa3LF2TGoAqhrW8fctFh7MW2s",
                    "epoch_id": "7e3Vkbngf36bphkBVX98LoRxpoqhvZJbL5Rgb3Yfccy8",
                    "height": 86697768,
                    "next_bp_hash": "Hib973UH8xTq4ReP2urd1bLEaHGmjwWeHCyfQV4ZbHAv",
                    "next_epoch_id": "7AEtEQErauvaagnmmDsxw9qnYqBVuTKjSW4P7DVwZ5z3",
                    "outcome_root": "AZYywqmo6vXvhPdVyuotmoEDgNb2tQzh2A1kV5f4Mxmq",
                    "prev_state_root": "6BWNcpk4chiEXWRWbWum5D4zutZ9pomfwwbmjanLp4sv",
                    "timestamp": 1649062589965425850,
                    "timestamp_nanosec": "1649062589965425850"
                },
                "inner_rest_hash": "DeSCLALKLSEX6pjKVoStCUq3ixkzK4v958TMkdPp1fJJ",
                "prev_block_hash": "Ae7sLAjvHs3gkiU2vFt8Vdxs5RmVUwyxyCwbnqnTkckQ"
            },
            "block_proof": [
                {
                    "direction": "Right",
                    "hash": "BNmeYcDcNoVXgXZyzcoyJiN5UiyLeZTvwSHYRpSfw9fF"
                },
                {
                    "direction": "Right",
                    "hash": "A7HaT2EGxrhJhDK2muP56b6j6c5JL1VAFPE45iB4cxsf"
                },
                {
                    "direction": "Left",
                    "hash": "AjhQk267UxRgxrTtLyjHrVoid7DPRN67aki8GJZttnu4"
                },
                {
                    "direction": "Left",
                    "hash": "4qyS6XAo8fNLYeGQJVN31D8ncr4TfmrvSe3cursw8oM7"
                },
                {
                    "direction": "Right",
                    "hash": "28y98e3vha3vHmkBhgREgxjLzjP7JzfVeu6H6yDHMh4V"
                },
                {
                    "direction": "Left",
                    "hash": "CJRqXDJy8L1oEGJDPxXgPuQhrFmLosoFQAf79Dyfrw3z"
                },
                {
                    "direction": "Left",
                    "hash": "CGaUbgtx9UFf7sZAe5fLdy1ggb5ZGg2oC3LmT2SgnCbz"
                },
                {
                    "direction": "Left",
                    "hash": "EjFednH4uWzcYNJzrfiBPbcDEvVTi7u7MEDFbcJfdPYf"
                },
                {
                    "direction": "Right",
                    "hash": "HAxQFR7SS2gkNUZ4nfSNefo3N1mxsmn3n7sMzhBxxLi"
                },
                {
                    "direction": "Left",
                    "hash": "KQa9Nzw7vPnciog75ZGNriVU7r4aAqKErE15mEBd3sS"
                },
                {
                    "direction": "Left",
                    "hash": "ByNUgeXrsQpeCNeNEqpe8ASw2bh2BfY7knpLaQe1NtXv"
                },
                {
                    "direction": "Left",
                    "hash": "ByrTiguozXfUaufYN8MuWAx7jL1dhZJ7bLzJjpCQjvND"
                },
                {
                    "direction": "Left",
                    "hash": "DvV6ak7n9wP1TQ1a97P81b81xJq1EdnERp8r3GFdP7wU"
                },
                {
                    "direction": "Left",
                    "hash": "Gga62BEfbomV8ZNz3DkPQEFf6UbEqMKngwNAp5zDDoki"
                },
                {
                    "direction": "Left",
                    "hash": "76U6DMh4J4VB5sfVVNRpSTeB4SEVt4HPqhtQi2izGZxt"
                }
            ],
            "outcome_proof": {
                "block_hash": "5aZZNiqUVbXXvRjjf1FB8sbXG3gpJeVCw1bYeREXzHk2",
                "id": "8HoqDvJGYrSjaejXpv2PsK8c5NUvqhU3EcUFkgq18jx9",
                "outcome": {
                    "executor_id": "relay.aurora",
                    "gas_burnt": 2428395018008,
                    "logs": [],
                    "metadata": {
                        "gas_profile": null,
                        "version": 1
                    },
                    "receipt_ids": [
                        "8hxkU4avDWFDCsZckig7oN2ypnYvLyb1qmZ3SA1t8iZK"
                    ],
                    "status": {
                        "SuccessReceiptId": "8hxkU4avDWFDCsZckig7oN2ypnYvLyb1qmZ3SA1t8iZK"
                    },
                    "tokens_burnt": "242839501800800000000"
                },
                "proof": [
                    {
                        "direction": "Right",
                        "hash": "B1Kx1mFhCpjkhon9iYJ5BMdmBT8drgesumGZoohWhAkL"
                    },
                    {
                        "direction": "Right",
                        "hash": "3tTqGEkN2QHr1HQdctpdCoJ6eJeL6sSBw4m5aabgGWBT"
                    },
                    {
                        "direction": "Right",
                        "hash": "FR6wWrpjkV31NHr6BvRjJmxmL4Y5qqmrLRHT42sidMv5"
                    }
                ]
            },
            "outcome_root_proof": [
                {
                    "direction": "Left",
                    "hash": "3hbd1r5BK33WsN6Qit7qJCjFeVZfDFBZL3TnJt2S2T4T"
                },
                {
                    "direction": "Left",
                    "hash": "4A9zZ1umpi36rXiuaKYJZgAjhUH9WoTrnSBXtA3wMdV2"
                }
            ]
        },
        "id": "idontcare"
    }
    "#;

    #[test]
    fn parse_light_client_proof_response() {
        let parsed_response = get_block_header_lite(client_proof_response).unwrap();
        assert_eq!(
            parsed_response
                .get_block_header_inner_lite_view()
                .block_merkle_root
                .as_ref(),
            JSONCryptoHash::from(
                Base58CryptoHash::try_from("D5nnsEuJ2WA4Fua4QJWXa3LF2TGoAqhrW8fctFh7MW2s").unwrap()
            )
            .as_ref(),
        );
        assert_eq!(
            parsed_response
                .get_light_client_block_view()
                .inner_rest_hash
                .as_ref(),
            JSONCryptoHash::from(
                Base58CryptoHash::try_from("DeSCLALKLSEX6pjKVoStCUq3ixkzK4v958TMkdPp1fJJ").unwrap()
            )
            .as_ref(),
        );
        assert_eq!(
            parsed_response
                .get_light_client_block_view()
                .prev_block_hash
                .as_ref(),
            JSONCryptoHash::from(
                Base58CryptoHash::try_from("Ae7sLAjvHs3gkiU2vFt8Vdxs5RmVUwyxyCwbnqnTkckQ").unwrap()
            )
            .as_ref(),
        );

        assert_eq!(
            parsed_response.outcome_proof.id.as_ref(),
            JSONCryptoHash::from(
                Base58CryptoHash::try_from("8HoqDvJGYrSjaejXpv2PsK8c5NUvqhU3EcUFkgq18jx9").unwrap()
            )
            .as_ref(),
        );

        assert_eq!(parsed_response.outcome_proof.proof.len(), 3);
    }

    #[test]
    fn calculate_hash_light_client_block_lite_view() {
        let block_header_lite = get_block_header_lite(client_proof_response).unwrap();
        let current_block_hash = block_header_lite
            .get_light_client_block_view()
            .current_block_hash();
        let expected: JSONCryptoHash =
            Base58CryptoHash::try_from("5aZZNiqUVbXXvRjjf1FB8sbXG3gpJeVCw1bYeREXzHk2")
                .unwrap()
                .into();
        assert_eq!(current_block_hash.as_ref(), expected);
    }

    #[test]
    fn test_validate_transaction() {
        let parsed_response = get_block_header_lite(client_proof_response).unwrap();
        assert_eq!(
            parsed_response
                .get_block_header_inner_lite_view()
                .block_merkle_root
                .as_ref(),
            JSONCryptoHash::from(
                Base58CryptoHash::try_from("D5nnsEuJ2WA4Fua4QJWXa3LF2TGoAqhrW8fctFh7MW2s").unwrap()
            )
            .as_ref(),
        );

        let execution_outcome_hash = calculate_execution_outcome_hash(
            &parsed_response.outcome_proof.outcome,
            parsed_response.outcome_proof.id,
        );
        let shard_outcome_root =
            compute_root_from_path(&parsed_response.outcome_proof.proof, execution_outcome_hash);

        let block_outcome_root = compute_root_from_path(
            &parsed_response.outcome_root_proof,
            Sha256::digest(shard_outcome_root.try_to_vec().unwrap())
                .as_slice()
                .try_into()
                .unwrap(),
        );

        let expected_block_outcome_root = JSONCryptoHash::from(
            Base58CryptoHash::try_from("AZYywqmo6vXvhPdVyuotmoEDgNb2tQzh2A1kV5f4Mxmq").unwrap(),
        );

        assert_eq!(expected_block_outcome_root, block_outcome_root.as_ref());
    }
}
