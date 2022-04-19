use crate::{
    block_validation::{validate_light_block, Digest},
    merkle_tree::compute_root_from_path,
    storage::StateStorage,
    types::{
        ApprovalInner, CryptoHash, ExecutionOutcomeView, LightClientBlockView, MerklePath,
        OutcomeProof,
    },
};

use borsh::BorshSerialize;
pub trait StateTransitionVerificator: StateStorage {
    type D: Digest;

    fn validate_and_update_head(&mut self, block_view: &LightClientBlockView) -> bool {
        let head = self.get_head();
        let epoch_block_producers = self.get_epoch_block_producers();
        if !validate_light_block::<Self::D>(head, block_view, epoch_block_producers) {}
        self.get_epoch_block_producers_mut().insert(
            block_view.inner_lite.next_epoch_id,
            block_view.next_bps.as_ref().unwrap().clone(),
        );

        let head = self.get_head_mut();
        *head = block_view.clone();
        true
    }

    fn validate_transaction(
        &self,
        outcome_proof: &OutcomeProof,
        outcome_root_proof: MerklePath,
        expected_block_outcome_root: CryptoHash,
    ) -> bool {
        let execution_outcome_hash =
            calculate_execution_outcome_hash::<Self::D>(&outcome_proof.outcome, outcome_proof.id);
        let shard_outcome_root =
            compute_root_from_path::<Self::D>(&outcome_proof.proof, execution_outcome_hash);

        let block_outcome_root = compute_root_from_path::<Self::D>(
            &outcome_root_proof,
            Self::D::digest(shard_outcome_root.try_to_vec().unwrap())
                .as_slice()
                .try_into()
                .unwrap(),
        );

        expected_block_outcome_root == block_outcome_root
    }
}

// This function is needed in order to calculate the right execution outcome hash
// Currently there is no function that calculates it in the `near-primitive` module
// hence, this is a direct port from the solidity implementation of the rainbow
// bridge written in solidity.
fn calculate_execution_outcome_hash<D: Digest>(
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
    let merkelization_hashes = calculate_merklelization_hashes::<D>(execution_outcome);

    // outcome.id is the tx hash or receipt id
    // let outcome = vec![merkelization_hashes.len() as u32 + 1, tx_hash, ];
    let pack_merklelization_hashes = merkelization_hashes
        .iter()
        .flat_map(|h| h.as_ref().to_owned())
        .collect::<Vec<u8>>();

    D::digest(
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

fn calculate_merklelization_hashes<D: Digest>(
    execution_outcome: &ExecutionOutcomeView,
) -> Vec<CryptoHash> {
    let logs_payload = vec![
        execution_outcome.receipt_ids.try_to_vec().unwrap(),
        execution_outcome.gas_burnt.try_to_vec().unwrap(),
        execution_outcome.tokens_burnt.try_to_vec().unwrap(),
        execution_outcome.executor_id.try_to_vec().unwrap(),
        execution_outcome.status.to_vec(), // This one comes already serialized (to make our lives simpler -- TODO: validate whether there's any risk associated with this)
    ]
    .concat();

    let first_element_merkelization_hashes = D::digest(logs_payload).as_slice().try_into().unwrap();
    execution_outcome
        .logs
        .iter()
        .fold(vec![first_element_merkelization_hashes], |mut acc, log| {
            acc.push(D::digest(log).as_slice().try_into().unwrap());
            acc
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{block_validation::SubstrateDigest, types::MerklePathItem};

    #[test]
    fn test_calculate_execution_outcome_hash() {
        // status comes serialized directly for convenience
        // otherwise we need to implement multiple of NEAR primitives enums
        let serialized_status = vec![
            3, 114, 128, 19, 177, 40, 127, 16, 184, 156, 69, 215, 55, 142, 98, 142, 27, 111, 246,
            232, 85, 207, 169, 209, 101, 242, 113, 144, 111, 227, 117, 100, 30,
        ];
        let decoded_hash = bs58::decode("8hxkU4avDWFDCsZckig7oN2ypnYvLyb1qmZ3SA1t8iZK")
            .into_vec()
            .unwrap();

        let receipt_id = CryptoHash::try_from(decoded_hash.as_ref()).unwrap();
        let execution_outcome = ExecutionOutcomeView {
            logs: vec![],
            receipt_ids: vec![receipt_id],
            gas_burnt: 2428395018008,
            tokens_burnt: 242839501800800000000,
            executor_id: "relay.aurora".into(),
            status: serialized_status,
        };

        let tx_hash = CryptoHash::try_from(
            bs58::decode("8HoqDvJGYrSjaejXpv2PsK8c5NUvqhU3EcUFkgq18jx9")
                .into_vec()
                .unwrap()
                .as_ref(),
        )
        .unwrap();

        let expected_execution_outcome_hash =
            bs58::decode("8QtUAFNktUqLp9fg9ohp5PAHjemxMcG6ryW2z5DcUK6C")
                .into_vec()
                .unwrap();
        assert_eq!(
            CryptoHash::try_from(expected_execution_outcome_hash.as_ref()).unwrap(),
            calculate_execution_outcome_hash::<SubstrateDigest>(&execution_outcome, tx_hash)
        );
    }

    #[test]
    fn test_compute_from_path() {
        let path = vec![
            MerklePathItem {
                hash: CryptoHash::try_from(
                    bs58::decode("3hbd1r5BK33WsN6Qit7qJCjFeVZfDFBZL3TnJt2S2T4T")
                        .into_vec()
                        .unwrap()
                        .as_ref(),
                )
                .unwrap(),
                direction: crate::types::Direction::Left,
            },
            MerklePathItem {
                hash: CryptoHash::try_from(
                    bs58::decode("4A9zZ1umpi36rXiuaKYJZgAjhUH9WoTrnSBXtA3wMdV2")
                        .into_vec()
                        .unwrap()
                        .as_ref(),
                )
                .unwrap(),
                direction: crate::types::Direction::Left,
            },
        ];
        let item_hash = CryptoHash::try_from(
            bs58::decode("2gvBz5DDhPVuy7fSPAu8Xei8oc92W2JtVf4SQRjupoQF")
                .into_vec()
                .unwrap()
                .as_ref(),
        )
        .unwrap();
        let expected_block_outcome_root = CryptoHash::try_from(
            bs58::decode("AZYywqmo6vXvhPdVyuotmoEDgNb2tQzh2A1kV5f4Mxmq")
                .into_vec()
                .unwrap()
                .as_ref(),
        )
        .unwrap();

        assert_eq!(
            expected_block_outcome_root,
            compute_root_from_path::<SubstrateDigest>(&path, item_hash)
        );
    }

    #[test]
    fn test_validate_transaction() {
        struct VeryDummyLiteClient;
        impl StateStorage for VeryDummyLiteClient {
            fn get_head(&self) -> &LightClientBlockView {
                todo!()
            }

            fn get_head_mut(&mut self) -> &mut LightClientBlockView {
                todo!()
            }

            fn get_epoch_block_producers(
                &self,
            ) -> &std::collections::HashMap<CryptoHash, Vec<crate::types::ValidatorStakeView>>
            {
                todo!()
            }

            fn get_epoch_block_producers_mut(
                &mut self,
            ) -> &mut std::collections::HashMap<CryptoHash, Vec<crate::types::ValidatorStakeView>>
            {
                todo!()
            }
        };
        impl StateTransitionVerificator for VeryDummyLiteClient {
            type D = SubstrateDigest;
        };

        let tx_hash = CryptoHash::try_from(
            bs58::decode("8HoqDvJGYrSjaejXpv2PsK8c5NUvqhU3EcUFkgq18jx9")
                .into_vec()
                .unwrap()
                .as_ref(),
        )
        .unwrap();
        let outcome_proof_proot = vec![
            MerklePathItem {
                hash: CryptoHash::try_from(
                    bs58::decode("B1Kx1mFhCpjkhon9iYJ5BMdmBT8drgesumGZoohWhAkL")
                        .into_vec()
                        .unwrap()
                        .as_ref(),
                )
                .unwrap(),
                direction: crate::types::Direction::Right,
            },
            MerklePathItem {
                hash: CryptoHash::try_from(
                    bs58::decode("3tTqGEkN2QHr1HQdctpdCoJ6eJeL6sSBw4m5aabgGWBT")
                        .into_vec()
                        .unwrap()
                        .as_ref(),
                )
                .unwrap(),
                direction: crate::types::Direction::Right,
            },
            MerklePathItem {
                hash: CryptoHash::try_from(
                    bs58::decode("FR6wWrpjkV31NHr6BvRjJmxmL4Y5qqmrLRHT42sidMv5")
                        .into_vec()
                        .unwrap()
                        .as_ref(),
                )
                .unwrap(),
                direction: crate::types::Direction::Right,
            },
        ];

        let outcome_root_proof = vec![
            MerklePathItem {
                hash: CryptoHash::try_from(
                    bs58::decode("3hbd1r5BK33WsN6Qit7qJCjFeVZfDFBZL3TnJt2S2T4T")
                        .into_vec()
                        .unwrap()
                        .as_ref(),
                )
                .unwrap(),
                direction: crate::types::Direction::Left,
            },
            MerklePathItem {
                hash: CryptoHash::try_from(
                    bs58::decode("4A9zZ1umpi36rXiuaKYJZgAjhUH9WoTrnSBXtA3wMdV2")
                        .into_vec()
                        .unwrap()
                        .as_ref(),
                )
                .unwrap(),
                direction: crate::types::Direction::Left,
            },
        ];

        let serialized_status = vec![
            3, 114, 128, 19, 177, 40, 127, 16, 184, 156, 69, 215, 55, 142, 98, 142, 27, 111, 246,
            232, 85, 207, 169, 209, 101, 242, 113, 144, 111, 227, 117, 100, 30,
        ];
        let decoded_hash = bs58::decode("8hxkU4avDWFDCsZckig7oN2ypnYvLyb1qmZ3SA1t8iZK")
            .into_vec()
            .unwrap();

        let receipt_id = CryptoHash::try_from(decoded_hash.as_ref()).unwrap();
        let execution_outcome = ExecutionOutcomeView {
            logs: vec![],
            receipt_ids: vec![receipt_id],
            gas_burnt: 2428395018008,
            tokens_burnt: 242839501800800000000,
            executor_id: "relay.aurora".into(),
            status: serialized_status,
        };
        let outcome_proof = OutcomeProof {
            block_hash: CryptoHash([0; 32]),
            id: tx_hash,
            proof: outcome_proof_proot,
            outcome: execution_outcome,
        };
        let expected_block_outcome_root = CryptoHash::try_from(
            bs58::decode("AZYywqmo6vXvhPdVyuotmoEDgNb2tQzh2A1kV5f4Mxmq")
                .into_vec()
                .unwrap()
                .as_ref(),
        )
        .unwrap();
        let dummy_lite_client = VeryDummyLiteClient {};
        assert!(dummy_lite_client.validate_transaction(
            &outcome_proof,
            outcome_root_proof,
            expected_block_outcome_root,
        ));
    }
}
