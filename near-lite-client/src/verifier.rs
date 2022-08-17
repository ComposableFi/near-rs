use crate::{
    block_validation::validate_light_block, merkle_tree::compute_root_from_path,
    storage::StateStorage, LiteClientResult,
};
use near_merkle_proofs::{HostFunctions, ProofBatchVerifier};
use near_primitives_wasm_friendly::Digest;
use near_primitives_wasm_friendly::{
    CryptoHash, ExecutionOutcomeView, LightClientBlockView, MerklePath, OutcomeProof,
};
use sp_std::{borrow::ToOwned, vec, vec::Vec};

use borsh::BorshSerialize;
pub trait StateTransitionVerificator: StateStorage {
    type D: Digest;
    type HF: HostFunctions;

    fn validate_and_update_head(
        &mut self,
        block_view: &LightClientBlockView,
    ) -> LiteClientResult<bool> {
        let head = self.get_head();
        let epoch_block_producers = self.get_epoch_block_producers();
        if !validate_light_block::<Self::D>(head, block_view, epoch_block_producers)? {
            return Ok(false);
        }
        self.insert_epoch_block_producers(
            block_view.inner_lite.next_epoch_id,
            block_view.next_bps.as_ref().unwrap().clone(),
        );

        self.set_new_head(block_view.clone());

        #[cfg(test)]
        log::info!(
            "updated head to height = {}",
            self.get_head().inner_lite.height
        );

        Ok(true)
    }

    fn validate_transaction(
        &self,
        outcome_proof: &OutcomeProof,
        outcome_root_proof: MerklePath,
        expected_block_outcome_root: CryptoHash,
    ) -> LiteClientResult<bool> {
        let execution_outcome_hash =
            calculate_execution_outcome_hash::<Self::D>(&outcome_proof.outcome, outcome_proof.id);
        let shard_outcome_root =
            compute_root_from_path::<Self::D>(&outcome_proof.proof, execution_outcome_hash)?;

        let block_outcome_root = compute_root_from_path::<Self::D>(
            &outcome_root_proof,
            Self::D::digest(shard_outcome_root.try_to_vec().unwrap())
                .as_slice()
                .try_into()
                .unwrap(),
        )?;

        // TODO: validate that the block_outcome_root is present in the state
        Ok(expected_block_outcome_root == block_outcome_root)
    }

    fn validate_transactions(
        &self,
        outcome_proofs: Vec<OutcomeProof>,
        outcome_root_proofs: Vec<MerklePath>,
        expected_block_outcome_root: CryptoHash,
    ) -> LiteClientResult<bool> {
        if outcome_proofs.len() != outcome_root_proofs.len() {
            return Ok(false);
        }
        if outcome_proofs.len() == 0 {
            // TODO: validate this
            return Ok(false);
        }

        let mut execution_outcome_hashes = vec![];
        for outcome_proof in &outcome_proofs {
            execution_outcome_hashes.push(calculate_execution_outcome_hash::<Self::D>(
                &outcome_proof.outcome,
                outcome_proof.id,
            ));
        }

        let mut proof_verifier_shard_outcome = ProofBatchVerifier::<Self::HF>::new();
        let outcome_proofs_iter = outcome_proofs.iter().map(|op| &op.proof);
        proof_verifier_shard_outcome.update_cache(outcome_proofs_iter.clone())?;

        let mut shard_outcome_roots = vec![];
        for (outcome_root_proof, execution_outcome_hash) in
            outcome_proofs_iter.zip(execution_outcome_hashes)
        {
            shard_outcome_roots.push(
                proof_verifier_shard_outcome
                    .calculate_root_hash(&outcome_root_proof, execution_outcome_hash)?,
            );
        }

        // confirm that all shard outcome roots are the same
        let shard_outcome_root_sample = &shard_outcome_roots[0];
        if shard_outcome_roots
            .iter()
            .skip(1)
            .any(|hash| hash != shard_outcome_root_sample)
        {
            return Ok(false);
        }

        let mut block_outcome_root_verifier = ProofBatchVerifier::<Self::HF>::new();
        let outcome_root_proofs_iter = outcome_root_proofs.iter();
        block_outcome_root_verifier.update_cache(outcome_root_proofs_iter.clone())?;

        for (outcome_root_proof, shard_outcome_root) in
            outcome_root_proofs_iter.zip(shard_outcome_roots)
        {
            let block_outcome_root = block_outcome_root_verifier.calculate_root_hash(
                &outcome_root_proof,
                Self::D::digest(shard_outcome_root.try_to_vec().unwrap())
                    .as_slice()
                    .try_into()
                    .unwrap(),
            )?;

            if expected_block_outcome_root != block_outcome_root {
                return Ok(false);
            }
        }
        // TODO: validate that the block_outcome_root is present in the state
        Ok(true)
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

    use crate::{block_validation::SubstrateDigest, test_utils::MockedHostFunctions};
    use borsh::BorshDeserialize;
    use near_primitives::views::LightClientBlockView as NearLightClientBlockView;
    use near_primitives_wasm_friendly::{Direction, MerklePathItem, ValidatorStakeView};

    use std::{collections::BTreeMap, io};
    #[derive(Debug, serde::Deserialize)]
    struct ResultFromRpc {
        pub result: NearLightClientBlockView,
    }

    pub fn get_client_block_view(
        client_block_response: &str,
    ) -> io::Result<NearLightClientBlockView> {
        Ok(
            serde_json::from_str::<ResultFromRpc>(client_block_response)?.result, // .into(),
        )
    }

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
                direction: Direction::Left,
            },
            MerklePathItem {
                hash: CryptoHash::try_from(
                    bs58::decode("4A9zZ1umpi36rXiuaKYJZgAjhUH9WoTrnSBXtA3wMdV2")
                        .into_vec()
                        .unwrap()
                        .as_ref(),
                )
                .unwrap(),
                direction: Direction::Left,
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
            compute_root_from_path::<SubstrateDigest>(&path, item_hash).unwrap()
        );
    }

    #[test]
    fn test_validate_transaction() {
        struct VeryDummyLiteClient;
        impl StateStorage for VeryDummyLiteClient {
            fn get_head(&self) -> &LightClientBlockView {
                todo!()
            }

            fn set_new_head(&mut self, _new_head: LightClientBlockView) {
                todo!()
            }

            fn get_epoch_block_producers(
                &self,
            ) -> &std::collections::BTreeMap<CryptoHash, Vec<ValidatorStakeView>> {
                todo!()
            }

            fn insert_epoch_block_producers(
                &mut self,
                _epoch: CryptoHash,
                _bps: Vec<ValidatorStakeView>,
            ) {
                todo!()
            }
        }

        impl StateTransitionVerificator for VeryDummyLiteClient {
            type D = SubstrateDigest;
            type HF = MockedHostFunctions;
        }

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
                direction: Direction::Right,
            },
            MerklePathItem {
                hash: CryptoHash::try_from(
                    bs58::decode("3tTqGEkN2QHr1HQdctpdCoJ6eJeL6sSBw4m5aabgGWBT")
                        .into_vec()
                        .unwrap()
                        .as_ref(),
                )
                .unwrap(),
                direction: Direction::Right,
            },
            MerklePathItem {
                hash: CryptoHash::try_from(
                    bs58::decode("FR6wWrpjkV31NHr6BvRjJmxmL4Y5qqmrLRHT42sidMv5")
                        .into_vec()
                        .unwrap()
                        .as_ref(),
                )
                .unwrap(),
                direction: Direction::Right,
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
                direction: Direction::Left,
            },
            MerklePathItem {
                hash: CryptoHash::try_from(
                    bs58::decode("4A9zZ1umpi36rXiuaKYJZgAjhUH9WoTrnSBXtA3wMdV2")
                        .into_vec()
                        .unwrap()
                        .as_ref(),
                )
                .unwrap(),
                direction: Direction::Left,
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
        assert!(dummy_lite_client
            .validate_transaction(
                &outcome_proof,
                outcome_root_proof.clone(),
                expected_block_outcome_root,
            )
            .unwrap());

        // test trivial version of validate transactions (only one transaction)
        assert!(dummy_lite_client
            .validate_transactions(
                vec![outcome_proof],
                vec![outcome_root_proof],
                expected_block_outcome_root,
            )
            .unwrap());
    }

    #[test]
    fn test_validate_light_block() {
        struct LessDummyLiteClient {
            head: LightClientBlockView,
            /// set of validators that can sign a mined block
            block_producers_per_epoch: BTreeMap<CryptoHash, Vec<ValidatorStakeView>>,
        }

        impl LessDummyLiteClient {
            fn new_from_checkpoint(checkpoint_head: LightClientBlockView) -> Self {
                let next_bps = checkpoint_head.next_bps.as_ref().unwrap().clone();
                let head = checkpoint_head.clone();
                Self {
                    head,
                    block_producers_per_epoch: [(
                        checkpoint_head.inner_lite.next_epoch_id,
                        next_bps,
                    )]
                    .into_iter()
                    .collect(),
                }
            }
        }

        impl StateStorage for LessDummyLiteClient {
            fn get_head(&self) -> &LightClientBlockView {
                &self.head
            }

            fn set_new_head(&mut self, new_head: LightClientBlockView) {
                self.head = new_head;
            }

            fn get_epoch_block_producers(&self) -> &BTreeMap<CryptoHash, Vec<ValidatorStakeView>> {
                &self.block_producers_per_epoch
            }

            fn insert_epoch_block_producers(
                &mut self,
                epoch: CryptoHash,
                bps: Vec<ValidatorStakeView>,
            ) {
                self.block_producers_per_epoch.insert(epoch, bps);
            }
        }

        impl StateTransitionVerificator for LessDummyLiteClient {
            type D = SubstrateDigest;
            type HF = MockedHostFunctions;
        }

        const CLIENT_RESPONSE_PREVIOUS_EPOCH: &str = r#"
        {
            "jsonrpc": "2.0",
            "result": {
                "approvals_after_next": [
                    "ed25519:4aQRJy2p92CYMc3EuRfM4oCHyobAL4VyL72e4n314ypQcUxZm7ynyCGh2Sb4kj3ESmEJeKxXZ6ejDcGhLd3UWqFc",
                    null,
                    "ed25519:4aQRJy2p92CYMc3EuRfM4oCHyobAL4VyL72e4n314ypQcUxZm7ynyCGh2Sb4kj3ESmEJeKxXZ6ejDcGhLd3UWqFc",
                    null,
                    "ed25519:4iaRL3pZfCjizdByKTxhBPYGc53UKQvN4Fe8S9RFJbvMcUFztJidwP4VtS9JNw8qWzu9Jt9mEe1XFRQwsDEm2jPT",
                    "ed25519:geFMne98ZmAwJrVaNn81gX7K7yEuWUjPWeAyn3hvP1pwUN4t4BXbAguw4bCN8S2sxBoWmh8Yys4A63go2a6SeSN",
                    "ed25519:5Kah5UWp685LL3eDKGcKr6XtEr8VzjWZZiNm46d2zuQSg3uG9fek6S9woiSMQqVZ3BA2MQDBNEbvYsBwDpmhsYYS",
                    "ed25519:2Lv7M71AFYGSmc6deuBSSG8aC26qqyNjgV1i1zn2HEMDoi2D72JZgNPLc7HsV5hT18Z5s5fBp7zCAvSuxG94MV3S",
                    "ed25519:4KGpWgCpoUQVbzfqc5kr2ZgMFeuM2einP6P3HMyiNi2QAk45oYNYHA88K9yhxsLqpxvqxo9UW6hYAsgDdNLjxG3g",
                    "ed25519:ZBaanAehTj12JS49WsHoGfF2Koyo9sDad31mdKfSb1akXELSfHR1dJobH5EzYLxtH8njFqrakbVnSsy6WjNyLrk",
                    "ed25519:4fjsB471GwjVfDzBZHpgurmZHe8aDazBav6BmGuSogoD5cd2u8B5k1qsC4EeKnphtKxfxZTUB8dHR88qd3KGogap",
                    "ed25519:5FhdghVU5yxRybuaRg7g6ygi58dHXT4JbwXd4WA6UZnVcRnS6piePYGP5pn1c6xzMEVQVPXWKTGttXkeFwgprXwb",
                    null,
                    "ed25519:4SdYWFriHrku3c5zMHPBdJhfBpi3dvK5rGeRFy8c6jxychejj5qoCdSYhqqdqBDcBsXEJfJgfjo2RNBSc5Ap7zcF",
                    null,
                    "ed25519:2x1sRijVLZ49janrQgVF5dDCXBXb5QFHTySYnQ1VSCHHKJM3SxJLADbDazNBZbgTeh5frqpkzKAgvthpzoakXLdo",
                    "ed25519:443peivFnX2QDNU5xdkRsQSK33AE1zBGjfELkpREGYDELeacfSYULG1kewQgEWBwBgDx6VSEewktjLHEeVEGqWbw",
                    "ed25519:BBxXPu9z3gdvXfdbPLHgEVqE3G9TtPuCSn2MFa25tc1VZtxNByEEWUpj6sqfjVysdHgrRmN4tZY3nMBhznAApPV",
                    "ed25519:3E9WDSv45wvqYDramYrTcBdDfsBEQBqEgFKxkR9L6oh2FtZnpyiMRLvXoEZkUPgsvmtueUXZbAYcrEc5cNTPx5Lr",
                    "ed25519:2h4dRgT7VhMUsptSK4Qjj8Dhv1rqVAFfYEVHGMwSebc4mc5TviUtyp8PWKy4XwvAa18yBC3ePQU7zxaERPuTMZbJ",
                    "ed25519:4jc1eJkqXActEwFSLBjcbqTqv15RWc5HnK8qHTJBRKXy4RSnB5DkhBmwWCnaz2gKVkqobmBpnE4ALZqDTKsTKLrH",
                    null,
                    "ed25519:KfXXkXvwNX3zzJ3WjBW1Rtg5pVdrq7xG1nuRmnErnAGms8zvavxUuoVXqxVRoipm6Toir57oAGuPZVxCqYmwHpB",
                    "ed25519:3YjV8TNydcGxtBBwq9MxqbdLbzvaPhbFMqtho1a2AFSPPveNCp74QowoZhhjUYThaHMzpV5pW8GfbErjmgLoESmB",
                    "ed25519:4rSsSHsXDE3HEwy7ACeiocRz7zT2cdUxVWjePyVjBueVEByTcZevFfGRWWG8LsJFC579vVYxMhWScgeGLvQUT6VU",
                    "ed25519:JXUvESCXuJ4R6pWJcPQ191tvLcjGqQFv6bjYWEuNpepei49582hniphS9y2pNURx31LWtGZRNLJc6dpZpdCaNcF",
                    "ed25519:1tqpLxMHba87vbjDg9oiAMrqkgUax8tHrYDMQQuYQx2YAeZ4QGPXAuh9R2XG54A3HfhKZizQid4Z7Q2PDoLF3CZ",
                    "ed25519:2yktL6ZrmK2WPQYjWgGkubeCdbrLCrMvFcFxUMALwEymkdtbB4QFHUFRX5yZyqcZ1GUjBdyrMGAAajHA5khsTCRQ",
                    "ed25519:4xfJQU4rdMGQKhS5aEvTa1AtSm4BPhPG4NgLqDCyYrMBwc8NPPJRdfVPNRHrUTxkXYwk5pVthngrWgxur64KVey2",
                    "ed25519:iqShSEPpuQaAero3bq6qh9oCEZYpSVYD5Fe2gQwFLi39z36vhF77H5VsgRjcsx7EpvzSjgcNbwD2CWRBBrXTLi1",
                    "ed25519:2xEAtUCqTZF8p9mPAj3nwkYLadWPB2wRZBbMaMhajBU7nuPbCZVVs3Ffo19L6BnLVWxRYdpx5qBwD3nhnprhERFP",
                    "ed25519:4bv4NZ5FrwECgiowGGWAfQEC4JF2wHExN98aPwpEi5EeF5jZkFnAHv1ScXTUZPavzh8uXNBRpXLZCmJBsiLmKQzy",
                    "ed25519:23SViFtn6fnAVSF5LiCyrYugpZpSaDWiMyahMuzxGyRgEJbv4js4BRLSixFL5gcDc7HzoA3D3fFjFRvW1XLiEyYb",
                    "ed25519:HEiaox139rUMwt92L6RLvitqRSSqVhvv28cooZCQPVpCspz6vuYgWQF2o2jFbcW86AXybE2D4jBNwtJ7HQDYiFa",
                    "ed25519:48o4rRJmqu1GJvkNBWUeCQ9UBiw4URfiakqGgvsas9aemK2gjHdUX3wePrPAEWRxnT3MPybJnsFbKBBRNuSXduTt",
                    null,
                    "ed25519:4XKAh8JrcvEnBfsLJvbUAjcPc8oR4Ep6fH1QvLyT85qvb9Z7DqkaEjbRJegcYVppSsHkEg25648TRHiFZPYi9yeK",
                    "ed25519:ZDGgMkurGMnYsEcJKxqC6fBMfFfuGhNFDutomST4fHSRWHDwmUFZkhx74L9kAQkLvZAs2awTvf4FTWdcvvFnnbr",
                    "ed25519:4pyZeRSH5WngPrKHs4SUYyziatDvpu2K97SC18BGFcSTHjzfE5evUQ8tnxWAUb17PjyyzuT8xhTqZrYDhNPyJBGR",
                    "ed25519:k69YEdonZBgRBRVWnTnDTRLCGkPk7MPHdxystmPckCszGtyH9HzouE3xaZDWeSaY4zzYfVjKPq4j6kxFTa58fAq",
                    null,
                    "ed25519:xHxCLLN8J7JmTJVzctsaJMhSZjTkdwPAZSnYXNz76u2V1HPTBsvbw7zraYGK5wYFqtpEEo8M27xvy22pfWWWsVk",
                    "ed25519:RdPGAPDp6eEWs7NnnHcQoNeAYaz2W93piweGkwfFv3kYgiaEYmX6tRmndVzqWoWrLJ1EarmmgAs6HQ4PFqPhxYn",
                    "ed25519:37EebdaMgzYHSZghHJbMmbVVLzjMrYUyPYgD65YKm9LeKC8eYcqc9meJJgVfTYYufpzSHNftqt8wjrjtU9kwEo8n",
                    "ed25519:pv3VngWFpHciNGF2rZvZEQhwXq83gYjhaRXetqwQgPfWbBa21WLjMHZB5jweVAt45Em6b7GzwYMRem2PaZXWQAT",
                    "ed25519:33DPjeMKnQ367HuUpN2xgESQJhMidu84yuynLNPQLfmbFb8EBPMA1emqZx1r3t5YUvWS3ncvkrWzGuQkapWCbPW6",
                    "ed25519:3YZNBgBzbM1fZKfnGCCfMdX6DxSgXXojTTr3q4P8T75P5iUcATQKKVVZcr8t5udWYsAzdi9Jj3S69L15HLDTT3X3",
                    null,
                    null,
                    null,
                    null,
                    null,
                    "ed25519:42bwC671pjEm9GP4mSdrTFtYaua8ThaETQGXK6o4EdBPo3PYfxufPkWhgduBaJ2UfACGxkb7tQTUyySKYAxmeiW7",
                    null,
                    null,
                    null,
                    null,
                    null,
                    null,
                    "ed25519:46iybWtKT1Wk1Ads2isiD1Y2Qg6uKxngev16JPsjnX73kRWagpttLLw1ZahoYUQ5YyT159nnmHSzqHyTgkxaL6pB",
                    "ed25519:3yn9MvYxWzXQkJmMX49dH1SLtKtK2vLHov5iEQgD4hAasH9BUcMV1ijvMnWjoWzPqk5UXa1DnwX2LNsT21YdqG6k",
                    null,
                    "ed25519:4N2tyS15xAtjtDiStx97PbJt8BbpEPYLJFwEuWpgP2M2foQWhyyHabPgWL9wNZmNobkPgVqgo5jQgWK61stMavN9",
                    "ed25519:zpByZEzShynMRqDHkthxGSXeAsi28Dt9VBMrUqSSEKxuAqCToJKQGeiPNyFB28xpMbUnwsPiaBJJiCUxRBjjWm9",
                    "ed25519:cZU7ShgEgFnKhNy9Cxq9dz3nC6vbZRWYSudyGXu2b5EgDmVTkvVXcSeiyJD1pJqUuRH84LDWBKbBkSG89B9xnHu",
                    "ed25519:4idcFJZUq2z27H9AtAUKX9LmoSH6XRrMiEonCcQHzBNkBoJ34UnBQLDpYvQMe6S9QHWgKeq5DHtqWUeeheAg2LDs",
                    "ed25519:4hveFV1Emah81wjDEoRNXgmUiFuEHMw58UqRHPkWEe41ResC1ceUynkJJEqz42XfdJEroqP8yMUo8YxmAEnKx4Te",
                    "ed25519:44EaZSwnoqKYeccSsuut3oincFu1RWiCwbGhpxiiWj6TecZtvaoCwtq9sMYnw6iCbLfD2L6MtwQX9mUPJcGMnMrp",
                    null,
                    null,
                    null,
                    null,
                    null,
                    null
                ],
                "inner_lite": {
                    "block_merkle_root": "DC3LrxVzqdthS9jGFrL3fNAHPkPtWxDyAw6UjcGhMz13",
                    "epoch_id": "5iyA2nJxvV4CVJwmfkK72X2f8s57shB2E174vWAPwHB7",
                    "height": 86441383,
                    "next_bp_hash": "8Uak4kmtmEmC9EN6TFJkhGd6UHHN9NSoJVZYZhJhf4hX",
                    "next_epoch_id": "GHmqgUX59irTdh31mtuEs3uEaPNBY5sQTZjEX5w7ASgW",
                    "outcome_root": "2AmR8gycvFzCp9sffcdrAW8RzBiVDwtL8sUjXA8MHMT3",
                    "prev_state_root": "FkH8tWA59SEGKDwZSWtceR2GfCbyXHFQCHKXXgcPRqNb",
                    "timestamp": 1648794682287664503,
                    "timestamp_nanosec": "1648794682287664503"
                },
                "inner_rest_hash": "H7YRMumnCoyRVtthcU4UJLaZeorFbvKAKspbHbK2pzjf",
                "next_block_inner_hash": "CbrBCRpefTYNYfsgnhiDH76J4LBF4bZTRhMYg2FhtRrg",
                "next_bps": [
                    {
                        "account_id": "node1",
                        "public_key": "ed25519:ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su",
                        "stake": "22922510070824652286443844340832",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "node0",
                        "public_key": "ed25519:ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su",
                        "stake": "16925122454732557817312342323673",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "node2",
                        "public_key": "ed25519:ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su",
                        "stake": "16874501568381514412356471157535",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "node3",
                        "public_key": "ed25519:ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su",
                        "stake": "8567814429874820736296779398515",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "legends.pool.f863973.m0",
                        "public_key": "ed25519:AhQ6sUifJYgjqarXSAzdDZU9ZixpUesP9JEH1Vr7NbaF",
                        "stake": "5786557069698344213896712425679",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "staked.pool.f863973.m0",
                        "public_key": "ed25519:D2afKYVaKQ1LGiWbMAZRfkKLgqimTR74wvtESvjx5Ft2",
                        "stake": "4555135305460764820741929416211",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "masternode24.pool.f863973.m0",
                        "public_key": "ed25519:9E3JvrQN6VGDGg1WJ3TjBsNyfmrU6kncBcDvvJLj6qHr",
                        "stake": "3412581904036190940565941993636",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "01node.pool.f863973.m0",
                        "public_key": "ed25519:3iNqnvBgxJPXCxu6hNdvJso1PEAc1miAD35KQMBCA3aL",
                        "stake": "3057699516361921995426438471613",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "p2p.pool.f863973.m0",
                        "public_key": "ed25519:4ie5979JdSR4f7MRAG58eghRxndVoKnAYAKa1PLoMYSS",
                        "stake": "2954970535640565975430641632183",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "nodeasy.pool.f863973.m0",
                        "public_key": "ed25519:25Dhg8NBvQhsVTuugav3t1To1X1zKiomDmnh8yN9hHMb",
                        "stake": "1573328151521226380938038278779",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "tribe-pool.pool.f863973.m0",
                        "public_key": "ed25519:CRS4HTSAeiP8FKD3c3ZrCL5pC92Mu1LQaWj22keThwFY",
                        "stake": "1427529115848797786690040790199",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "chorusone.pool.f863973.m0",
                        "public_key": "ed25519:3TkUuDpzrq75KtJhkuLfNNJBPHR5QEWpDxrter3znwto",
                        "stake": "1277333300998711575968735209879",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "hotones.pool.f863973.m0",
                        "public_key": "ed25519:2fc5xtbafKiLtxHskoPL2x7BpijxSZcwcAjzXceaxxWt",
                        "stake": "1272041758046518449192873006540",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "foundryusa.pool.f863973.m0",
                        "public_key": "ed25519:ABGnMW8c87ZKWxvZLLWgvrNe72HN7UoSf4cTBxCHbEE5",
                        "stake": "1254473966505567163739276072923",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "lunanova2.pool.f863973.m0",
                        "public_key": "ed25519:9Jv6e9Kye4wM9EL1XJvXY8CYsLi1HLdRKnTzXBQY44w9",
                        "stake": "1245973800630588895851102069596",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "chorus-one.pool.f863973.m0",
                        "public_key": "ed25519:6LFwyEEsqhuDxorWfsKcPPs324zLWTaoqk4o6RDXN7Qc",
                        "stake": "1109131454872548236874586501001",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "ni.pool.f863973.m0",
                        "public_key": "ed25519:GfCfFkLk2twbAWdsS3tr7C2eaiHN3znSfbshS5e8NqBS",
                        "stake": "1075644847325960842995579102016",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "cryptogarik.pool.f863973.m0",
                        "public_key": "ed25519:FyFYc2MVwgitVf4NDLawxVoiwUZ1gYsxGesGPvaZcv6j",
                        "stake": "839670626002142575910140045408",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "pathrocknetwork.pool.f863973.m0",
                        "public_key": "ed25519:CGzLGZEMb84nRSRZ7Au1ETAoQyN7SQXQi55fYafXq736",
                        "stake": "748664174850316067955626836603",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "stakely_v2.pool.f863973.m0",
                        "public_key": "ed25519:7BanKZKGvFjK5Yy83gfJ71vPhqRwsDDyVHrV2FMJCUWr",
                        "stake": "733920838342885843756234789662",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "solidstate.pool.f863973.m0",
                        "public_key": "ed25519:DTDhqoMXDWhKedWpH7DPvR6dPDcXrk5pTHJw2bkFFvQy",
                        "stake": "714369900526852891761727522625",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "aurora.pool.f863973.m0",
                        "public_key": "ed25519:9c7mczZpNzJz98V1sDeGybfD4gMybP4JKHotH8RrrHTm",
                        "stake": "702270431224324013411083156419",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "namdokmai.pool.f863973.m0",
                        "public_key": "ed25519:9uGeeM7j1fimpG7vn6EMcBXMei8ttWCohiMf44SoTzaz",
                        "stake": "698608807949059141913608464868",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "freshtest.pool.f863973.m0",
                        "public_key": "ed25519:5cbAt8uzmRztXWXKUYivtLsT2kMC414oHYDapfSJcgwv",
                        "stake": "696258382452410785529813203905",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "optimusvalidatornetwork.pool.f863973.m0",
                        "public_key": "ed25519:BGoxGmpvN7HdUSREQXfjH6kw5G6ph7NBXVfBVfUSH85V",
                        "stake": "660410303330680943777398509110",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "baziliknear.pool.f863973.m0",
                        "public_key": "ed25519:9Rbzfkhkk6RSa1HoPnJXS4q2nn1DwYeB4HMfJBB4WQpU",
                        "stake": "650389309598466011397359969840",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "blockscope.pool.f863973.m0",
                        "public_key": "ed25519:6K6xRp88BCQX5pcyrfkXDU371awMAmdXQY4gsxgjKmZz",
                        "stake": "648847313919662030580998874408",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "tagard.pool.f863973.m0",
                        "public_key": "ed25519:3KyziFgx3PpzorJnMFifXU4KsK4nwPFaxCGWTHaFBADK",
                        "stake": "646030292693079789339891316676",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "leadnode.pool.f863973.m0",
                        "public_key": "ed25519:CdP6CBFETfWYzrEedmpeqkR6rsJNeT22oUFn2mEDGk5i",
                        "stake": "643614800541578264591193039953",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "stakesstone.pool.f863973.m0",
                        "public_key": "ed25519:3aAdsKUuzZbjW9hHnmLWFRKwXjmcxsnLNLfNL4gP1wJ8",
                        "stake": "640424273252137206131666996427",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "basilisk-stake.pool.f863973.m0",
                        "public_key": "ed25519:CFo8vxoEUZoxbs87mGtG8qWUvSBHB91Vc6qWsaEXQ5cY",
                        "stake": "639170811025048237433711633196",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "shardlabs.pool.f863973.m0",
                        "public_key": "ed25519:DxmhGQZ6oqdxw7qGBvzLuBzE6XQjEh67hk5tt66vhLqL",
                        "stake": "636859864974473764754276519109",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "al3c5.pool.f863973.m0",
                        "public_key": "ed25519:BoYixTjyBePQ1VYP3s29rZfjtz1FLQ9og4FWZB5UgWCZ",
                        "stake": "635910968806069871670455050245",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "dehashed.pool.f863973.m0",
                        "public_key": "ed25519:EmPyD1DV9ajWJxjNN8GGACMyhM9w14brwNwYA5WvVaw",
                        "stake": "634481857193800599750766324892",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "machfund.pool.f863973.m0",
                        "public_key": "ed25519:G6fJ79oM6taQGhHeQZrg8N36nkCPMEVPyQMHfFT2wAKc",
                        "stake": "633945472280870168216616557188",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "projecttent.pool.f863973.m0",
                        "public_key": "ed25519:2ueHfYVewchegMmae9bc86ngdD1FWTbxewVb8sr4cABx",
                        "stake": "633694652515862591275483076651",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "blockngine.pool.f863973.m0",
                        "public_key": "ed25519:CZrTtCP6XkkxWtr3ATnXE8FL6bcG5cHcxfmdRgN7Lm7m",
                        "stake": "633004909477506166031191906205",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "grassets.pool.f863973.m0",
                        "public_key": "ed25519:3S4967Dt1VeeKrwBdTTR5tFEUFSwh17hEFLATRmtUNYV",
                        "stake": "621795588696433313338045066893",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "bflame.pool.f863973.m0",
                        "public_key": "ed25519:4uYM5RXgR9D6VAGKHgQTVNLEmCgMVX7PzpBstT92Me6R",
                        "stake": "616319592025058485087576215014",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "shurik.pool.f863973.m0",
                        "public_key": "ed25519:9zEn7DVpvQDxWdj5jSgrqJzqsLo8T9Wv37t83NXBiWi6",
                        "stake": "615607598217041309240812408441",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "p0s.pool.f863973.m0",
                        "public_key": "ed25519:B4YpQ7qtD9w6VwujjJmZW8yrN5U13S5xuiTRiK63EzuF",
                        "stake": "614976797913850275582890973277",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "dsrvlabs.pool.f863973.m0",
                        "public_key": "ed25519:61ei2efmmLkeDR1CG6JDEC2U3oZCUuC2K1X16Vmxrud9",
                        "stake": "613074857526823170190346894417",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "zetsi.pool.f863973.m0",
                        "public_key": "ed25519:6rYx5w1Z2pw46NBHv6Wo4JEUMNtqnDGqPaHT4wm15YRw",
                        "stake": "611167150967495369878691581128",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "n0ok.pool.f863973.m0",
                        "public_key": "ed25519:D6Gq2RpUoDUojmE2vLpqQzuZwYmFPW6rMcXPrwRYhqN8",
                        "stake": "593639947175383273636863070076",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "chelovek_iz_naroda.pool.f863973.m0",
                        "public_key": "ed25519:89aWsXXytjAZxyefXuGN73efnM9ugKTjPEGV4hDco8AZ",
                        "stake": "592047145048110825510136899314",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "lavenderfive.pool.f863973.m0",
                        "public_key": "ed25519:AzwAiLDqprZKpDjhsH7dfyvFdfSasmPTjuJUAHfX1Pg4",
                        "stake": "585546022984649856687381527767",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "latenthero.pool.f863973.m0",
                        "public_key": "ed25519:EQqmjRNouRKhwGL7Hnp3vcbDywg2Boj6to2gmnXybhEM",
                        "stake": "579758122709566871952544545842",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "smcvalidator.pool.f863973.m0",
                        "public_key": "ed25519:pG4LYsyoAa8yWYG9nsTQ5yBcwke51i3VqeRcMVbE9Q7",
                        "stake": "555422197586970576403131175346",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "phet90testnet.pool.f863973.m0",
                        "public_key": "ed25519:AVaLksnE1S1A3mC6Mr3t9KnD67aA2R2vw68qTZ92MNu2",
                        "stake": "549661980894401124487359434513",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "everstake.pool.f863973.m0",
                        "public_key": "ed25519:4LDN8tZUTRRc4siGmYCPA67tRyxStACDchdGDZYKdFsw",
                        "stake": "545692905175646615423583560444",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "rossi-validator.pool.f863973.m0",
                        "public_key": "ed25519:2eRx2c3KX9wFd3EzuuajFQoSxRTKDqSbxcF13LfkrxCR",
                        "stake": "545396693341301216467002952473",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "gullffa.pool.f863973.m0",
                        "public_key": "ed25519:79HUZcLERE4kLTraoaiEtJYCYeH6NZi6mYQ7YpbENazE",
                        "stake": "540374898942935788531590509248",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "infiniteloop.pool.f863973.m0",
                        "public_key": "ed25519:2fbiLqksH5viWXYoteyfKP9qQawkRKw4YogRFcvG3Z7f",
                        "stake": "537692918603524974448590839751",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "mintia.pool.f863973.m0",
                        "public_key": "ed25519:JAWDzHY7Ku99rW45WjS1Wh9fMc6CJ7M3vncnzoiTwfkL",
                        "stake": "513896136137419546135584944589",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "lusienda.pool.f863973.m0",
                        "public_key": "ed25519:HdQb2HEiaMgvUdemTt5rkrFbxTpzZyELvg1Vov4LQAGU",
                        "stake": "508420470175664819906389753582",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "anchikovproduction.pool.f863973.m0",
                        "public_key": "ed25519:HDadu8UN6KTwenWdZRVmjsVnZhhKyLHLSNBYGCvrWmWg",
                        "stake": "502965985183139930884631241050",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "ino.pool.f863973.m0",
                        "public_key": "ed25519:B75h2eqpaMgh6WkAvgnz2FsEC9s5TwVx7zwTjqXKfRs6",
                        "stake": "494974817176910920780821071715",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "pontiff.pool.f863973.m0",
                        "public_key": "ed25519:4i8j7nwNyy18hfARtrVpckT8MiicdCXuWBX1TubdMb5Y",
                        "stake": "478587210879643963063840990682",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "prophet.pool.f863973.m0",
                        "public_key": "ed25519:HYJ9mUhxLhzSVtbjj89smAaZkMqXca68iCumZy3gySoB",
                        "stake": "353790651507020866141066683643",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "sashamaxymchuk.pool.f863973.m0",
                        "public_key": "ed25519:84G4fGj5nvuNq6WLqbBejApRjbRKztiWkqkLJ96gBwz7",
                        "stake": "152678736698423437298364034529",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "kiln.pool.f863973.m0",
                        "public_key": "ed25519:Bq8fe1eUgDRexX2CYDMhMMQBiN13j8vTAVFyTNhEfh1W",
                        "stake": "96495618271143227175107926458",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "nodemeister.pool.f863973.m0",
                        "public_key": "ed25519:85EMyaNGMFuHK2RDH7KHno6fVYBR6iykUXHPPmFTGuTB",
                        "stake": "46966596609590542375097063730",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "nala.pool.f863973.m0",
                        "public_key": "ed25519:Fzwndob2h5PFdEuwo9eRFJV3BLLurcNaw2SGob5rMPEn",
                        "stake": "44714508225885118642457972042",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "happystake.pool.f863973.m0",
                        "public_key": "ed25519:3APqZiwzeZLzgfkJyGGTfepDYHA2d8NF1wZi4mCpZnaJ",
                        "stake": "43908619030797948815749119844",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "wolfedge-capital-testnet.pool.f863973.m0",
                        "public_key": "ed25519:CQEMcPQz6sqhAgoBm9ka9UeVcXj5NpNpRtDYYGkPggvg",
                        "stake": "37421154012756824118554142900",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "jstaking.pool.f863973.m0",
                        "public_key": "ed25519:fui1E5XwnAWGYDBSQ3168aDfsW1KDFH8A7nBHvZiqGv",
                        "stake": "36368375187772860724451257216",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "4ire-pool.pool.f863973.m0",
                        "public_key": "ed25519:EWPSvYN9pGPMmCLjVxx96stWdqksXNSGnfnuWYn9iiE5",
                        "stake": "33830317130679015785634403282",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "lionstake.pool.f863973.m0",
                        "public_key": "ed25519:Fy6quR4nBhrEnDyEuPWoAdBP5tzNbuEZsEd91Q5pQnXB",
                        "stake": "33726419125235534318928178338",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "zentriav2.factory.colorpalette.testnet",
                        "public_key": "ed25519:4rCwSFzJ2e6suD5Yi7pgLidcAJ8Zt9BXieLzVedJDwmE",
                        "stake": "30560796702897684567471814711",
                        "validator_stake_struct_version": "V1"
                    },
                    {
                        "account_id": "lastnode.pool.f863973.m0",
                        "public_key": "ed25519:811gesxXYdYeThry96ZiWn8chgWYNyreiScMkmxg4U9u",
                        "stake": "24118112236988428679747661113",
                        "validator_stake_struct_version": "V1"
                    }
                ],
                "prev_block_hash": "FD128fQ4vBeCKqEnkfCGdbVWXZPcaCfrAY3MSpib1mDr"
            },
            "id": "idontcare"
        }"#;

        // Block #86455884
        const CLIENT_BLOCK_RESPONSE: &str = r#"
    {
        "jsonrpc": "2.0",
        "result": {
            "approvals_after_next": [
                "ed25519:26AdvKhPjpJSvednVPzvKzzauvEixBHaatmjR5P1jNYKPShgdi6BgMFUrebGbS4aAAA7CbE8JamcJ13SoKBKYQEX",
                null,
                "ed25519:26AdvKhPjpJSvednVPzvKzzauvEixBHaatmjR5P1jNYKPShgdi6BgMFUrebGbS4aAAA7CbE8JamcJ13SoKBKYQEX",
                "ed25519:26AdvKhPjpJSvednVPzvKzzauvEixBHaatmjR5P1jNYKPShgdi6BgMFUrebGbS4aAAA7CbE8JamcJ13SoKBKYQEX",
                "ed25519:38zDqDYHaW35Ag9U1YZudiMdUZBKtQqKgo8S7rKWP6Kt2dx6RDbBZDSYTeojhY6WkdKaG7tdPXZgJFEvRU4TjMsS",
                "ed25519:5zApVpNrJ1xhqtBACN6ZpR6ePcrKAHvvKfKpx365n87XXWTLj8MtGZvkZ6netaX9wesHPejmABJwfTvMCcv8ofay",
                "ed25519:42gdZewdzK3EHY4zre69rVTMU2yuwpFd13tDv4JopPpqE7x218xDGmtThrii3r7AVLMJc49TvuueUc87JKJKi1g9",
                "ed25519:EEXpy5RLuCBHe8XWQio8QjzaJTRb7RmJCtubwaNUGm1tja2zF2YgEfpVQx2ywsNCtsJvGqMGW6JWaDZLnErCFVJ",
                "ed25519:3EhiCzKkmcviyhdf7rfjhSZjm8soxp2DyZj41ryhHJ8FpW7QXU7VmBpeDbN1zG8AuAzbSxeHTRVLAhegnQVtbb5J",
                "ed25519:3uU4J5dc7VZpNqwB1CXaFpCzAzxiAcZNF73jEwcGHrT7bqFZ2mZtFGKmHZxEnDVJdPjQNXrgXePaWSZdvpiXWf7G",
                "ed25519:2SvJzrJZhabvpuwTmGXAxkjJvjKEVWVp3Q63k3u5AyDbbZZTWFU6UDyTJt1YLKmuSFD9p6t4BUWE5E3S88mvmzvG",
                "ed25519:24vdiR1GuDShms851Pd2LreemL9NRqacXzfbHN2zH7iNoSQ4sVz5QhhDaH6kuibRFMT1cqok8zU6o6PNWXbkrkia",
                null,
                "ed25519:5dw8MnX6xt6QnnDhSfjaJAo4gHWb2QSLacLcgGQoUwRq12E5inzfcJgkG742LbFq5xKhvZCshKiNvTPmK5oZWnny",
                null,
                "ed25519:2HXALBATYbkj35ioUqmRtBU7wxcc8CHiNtZS8wk7c95Y229bqP8wznK8RG53dLYXcm9647BRh9SFXMt2eFnmNB3B",
                "ed25519:3VKHEkpXNCWR5jjsuVDfMCdxbYjtcqmWqW5823HDrgyWG7mr1cWZActykTvRR56EDbDsFfDLU1cjHGgrg1yrbJpX",
                null,
                "ed25519:UWFcxWgoq3EnKQV7sfmmCeg56jdhVAD7enpE1mVQkRsRc29cp8FQSeuMw5PgfdvLQDsBvh3jE9e7mHbZdUhwugF",
                null,
                null,
                null,
                null,
                "ed25519:3X2DCE39PMbXAkPSUUv26RuvrkMD4mgxFer4Vx1NyRu5G8gaMmDF5anASkWwAUranoJgcdb17iHiVqGXKi8j3kww",
                null,
                "ed25519:3BY1dzwxJ3jyPVcSMnRYm2NLytV3Hn7WDPwrwjwotkTzM3bQbj5soXQjtfpChWkoExAvF8nRteLQ7xCiiGjTNyWs",
                "ed25519:473YNvCkBZSDX3G4x8hvcpnEtogKhxBnLYa5pGykKourLcR3vKZCeUQ19SEfMk1K1qreUL692N1qicN8FtCG745n",
                null,
                "ed25519:5E1TJ2akcffmbTFb2LbDpScdqicyck8WKsm9gZZs9vgPgo2HxLxd9JJib79TqgDWRB9WUiiB2cxmDXBiPwXbjyAp",
                null,
                "ed25519:5qAHAoYpVB86gWBfakLxJYD1aSetXUfmass4Q4hiXiJ6KZgktzFqVaxUjYyBwtpjcqQrtGbsLpjkRz4orjENkJiv",
                null,
                "ed25519:5M2J4oDYi798Zzsp5KyusUYTaGeTZPEZAPjoPUUDkRLZDu4omMnVMArJABu7geFGGJ24RvcuMFvxxmgLFjuhyEkx",
                "ed25519:5wG7q6qucqDMZmDq8KYdisSUuGxMc795c7KoM2BMT2J8LRoQLsLESjZRxk8CEF3GdzHdhfy5NFaopRuNLoePzksz",
                null,
                null,
                null,
                "ed25519:5QatrWTnrjdCiXmiG95gAhvyeggaXwCzHVmTQQJ5YRTejuJRjkiKJJ6VKHQG1ENfWkLFce4ho4sP9U5xjd3z82m9",
                null,
                "ed25519:5cByB2PH8qzWJcqvx4sTPi59kC6iNk2mWLo3aBuAUY8c5kGZu1Co3q9KVxLdnwBhdjw6RDFuefqWtqHmENP2HnMb",
                null,
                null,
                "ed25519:53euGtGneS9pMRfoak8TwpKnFxngWXEs11GStuhkrZAKgWAGFcc6FMtPHrLS6h9jZvYZ1DRArTy1RRuwcVENuBMA",
                "ed25519:3tSn2cFRcKPfRNx4ayC8JUsV2FV3ZRuCUe4FUL71HFmDoE9TXzEZbLnPCEDm4fDYAibHs6AQGZeAqGBKvodNZLj5",
                "ed25519:Nz1xPY9VFdtidAP3rBmdRbn3g8gBvP5a3eTik9GyBmaNfiJWqg8GFM2nVHa4o9rjtenmbhTSrQszX6XGoKDcsGU",
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                "ed25519:5HTWzh8SBSnJwPrPUuog7DCu5KnnPibvt2bAY8acC9XwGTy3n2kbJViwXvgpFmcoPFpyNSTR2uA6v5sqb1URAdEz",
                null,
                null,
                null,
                null,
                null,
                "ed25519:3dE44DTNBcJjvQ7ZZcXCUnqnWN1Ut4swxRqzv1frdzFN2MYsNBx7rMmmiXGr33zJ7jXWRTLbJgBHG8b9Tsy8kdnZ",
                "ed25519:4R8wrJjPinQAeyXxHHZuu25nsQVdw5RcN8fF1hzaGHTycjonj1WgeAJzm88WJNcgjMbbRgn8BJzR4bQCF3UfN7Pr",
                null,
                "ed25519:5aLBAW42sdqJhQe51jvw21XACgBXgSV2R4bBFjc8LpJBhSFu6USoNzNkAEcrtH4fiFC5eU8mh2cNWJYRS7eBabi9",
                "ed25519:2iW39NZjxGq6XU7miQWmDsMNnbVXKu48P87PJoX2sDTz74GegGcoD71eX8Fm8K9umyARFffFU1CKXyA1eoFnefs1",
                null,
                "ed25519:3kNinyfW8exUi8Qi5nr1MX1DB27yDEfjnmq4hsAVXbACFgBfcJgLAysSp7Ue38URQtwEVnt9pefVbZ8PPP6wDKGz",
                "ed25519:5X6Fq8PeNtc6sv84QeKPd5MG4La9K3rBMDYbKtkJ8VZcC6k1ehFd9NP3PuBqwL5gMqoqj7nkzSQZzJzKDJLPJRCA",
                null,
                null
            ],
            "inner_lite": {
                "block_merkle_root": "9RzsdrtaXvCup17nAfVdHwxTd76j1FNaW7e7z2McuZHs",
                "epoch_id": "GHmqgUX59irTdh31mtuEs3uEaPNBY5sQTZjEX5w7ASgW",
                "height": 86455909,
                "next_bp_hash": "9VPzyStHi4X2T7VAbfSTbLXEd8vjFP7wFJjYyjSJxQik",
                "next_epoch_id": "8nVTHDfxg2G8AWbKhVfFtnEb5jJeiXV2XBFsdyt2cif1",
                "outcome_root": "ceLGxBWvaytQzku8q3NoxuE3922MnMFxsNEVCLPBeUD",
                "prev_state_root": "AJuNNNFMaeKJc4Ufapuy3hc1jkoPLvFbVWpXcZdLfg1U",
                "timestamp": 1648810040622608566,
                "timestamp_nanosec": "1648810040622608566"
            },
            "inner_rest_hash": "6SK4tgTksZyLx9cW2neu4xBXuJPkaseFrR9riTKMLJwv",
            "next_block_inner_hash": "2fwovRgRy3GNv4nivi8PgwM3aJAeedbvWYMCDYt5CsYG",
            "next_bps": [
                {
                    "account_id": "node1",
                    "public_key": "ed25519:ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su",
                    "stake": "22949327592242450816363151898853",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "node0",
                    "public_key": "ed25519:ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su",
                    "stake": "16944923507607057621836326590864",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "node2",
                    "public_key": "ed25519:ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su",
                    "stake": "16894243398827941870356919783063",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "node3",
                    "public_key": "ed25519:ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su",
                    "stake": "8577838094223400746241842212915",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "legends.pool.f863973.m0",
                    "public_key": "ed25519:AhQ6sUifJYgjqarXSAzdDZU9ZixpUesP9JEH1Vr7NbaF",
                    "stake": "5793326871499643941084500854531",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "staked.pool.f863973.m0",
                    "public_key": "ed25519:D2afKYVaKQ1LGiWbMAZRfkKLgqimTR74wvtESvjx5Ft2",
                    "stake": "4559762052294055739961541809028",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "masternode24.pool.f863973.m0",
                    "public_key": "ed25519:9E3JvrQN6VGDGg1WJ3TjBsNyfmrU6kncBcDvvJLj6qHr",
                    "stake": "3416574120678826701003147150326",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "01node.pool.f863973.m0",
                    "public_key": "ed25519:3iNqnvBgxJPXCxu6hNdvJso1PEAc1miAD35KQMBCA3aL",
                    "stake": "3061276782639300406837420592214",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "p2p.pool.f863973.m0",
                    "public_key": "ed25519:4ie5979JdSR4f7MRAG58eghRxndVoKnAYAKa1PLoMYSS",
                    "stake": "2958427611565856637171061933942",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "nodeasy.pool.f863973.m0",
                    "public_key": "ed25519:25Dhg8NBvQhsVTuugav3t1To1X1zKiomDmnh8yN9hHMb",
                    "stake": "1575068818350064235628643461649",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "tribe-pool.pool.f863973.m0",
                    "public_key": "ed25519:CRS4HTSAeiP8FKD3c3ZrCL5pC92Mu1LQaWj22keThwFY",
                    "stake": "1429199212043501677779067532132",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "chorusone.pool.f863973.m0",
                    "public_key": "ed25519:3TkUuDpzrq75KtJhkuLfNNJBPHR5QEWpDxrter3znwto",
                    "stake": "1278827676875609593894511486301",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "hotones.pool.f863973.m0",
                    "public_key": "ed25519:2fc5xtbafKiLtxHskoPL2x7BpijxSZcwcAjzXceaxxWt",
                    "stake": "1273529881837124230828073909315",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "foundryusa.pool.f863973.m0",
                    "public_key": "ed25519:ABGnMW8c87ZKWxvZLLWgvrNe72HN7UoSf4cTBxCHbEE5",
                    "stake": "1256081604638924285747937189845",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "lunanova2.pool.f863973.m0",
                    "public_key": "ed25519:9Jv6e9Kye4wM9EL1XJvXY8CYsLi1HLdRKnTzXBQY44w9",
                    "stake": "1247431491303762172509349058430",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "chorus-one.pool.f863973.m0",
                    "public_key": "ed25519:6LFwyEEsqhuDxorWfsKcPPs324zLWTaoqk4o6RDXN7Qc",
                    "stake": "1110429050842727763339891353120",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "ni.pool.f863973.m0",
                    "public_key": "ed25519:GfCfFkLk2twbAWdsS3tr7C2eaiHN3znSfbshS5e8NqBS",
                    "stake": "1076903268858699791106964347506",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "cryptogarik.pool.f863973.m0",
                    "public_key": "ed25519:FyFYc2MVwgitVf4NDLawxVoiwUZ1gYsxGesGPvaZcv6j",
                    "stake": "840652974653901124214299092043",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "pathrocknetwork.pool.f863973.m0",
                    "public_key": "ed25519:CGzLGZEMb84nRSRZ7Au1ETAoQyN7SQXQi55fYafXq736",
                    "stake": "749739988926667488225409312930",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "stakely_v2.pool.f863973.m0",
                    "public_key": "ed25519:7BanKZKGvFjK5Yy83gfJ71vPhqRwsDDyVHrV2FMJCUWr",
                    "stake": "734779467803676488422251769143",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "solidstate.pool.f863973.m0",
                    "public_key": "ed25519:DTDhqoMXDWhKedWpH7DPvR6dPDcXrk5pTHJw2bkFFvQy",
                    "stake": "715205657993906057594050568659",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "aurora.pool.f863973.m0",
                    "public_key": "ed25519:9c7mczZpNzJz98V1sDeGybfD4gMybP4JKHotH8RrrHTm",
                    "stake": "703162032315675728652111978820",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "namdokmai.pool.f863973.m0",
                    "public_key": "ed25519:9uGeeM7j1fimpG7vn6EMcBXMei8ttWCohiMf44SoTzaz",
                    "stake": "699426128043696790256527911933",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "freshtest.pool.f863973.m0",
                    "public_key": "ed25519:5cbAt8uzmRztXWXKUYivtLsT2kMC414oHYDapfSJcgwv",
                    "stake": "697072950038835725218153979145",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "optimusvalidatornetwork.pool.f863973.m0",
                    "public_key": "ed25519:BGoxGmpvN7HdUSREQXfjH6kw5G6ph7NBXVfBVfUSH85V",
                    "stake": "661182931526239970852421432715",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "baziliknear.pool.f863973.m0",
                    "public_key": "ed25519:9Rbzfkhkk6RSa1HoPnJXS4q2nn1DwYeB4HMfJBB4WQpU",
                    "stake": "651150213650042597898598894903",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "blockscope.pool.f863973.m0",
                    "public_key": "ed25519:6K6xRp88BCQX5pcyrfkXDU371awMAmdXQY4gsxgjKmZz",
                    "stake": "649506414222131713576984442889",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "tagard.pool.f863973.m0",
                    "public_key": "ed25519:3KyziFgx3PpzorJnMFifXU4KsK4nwPFaxCGWTHaFBADK",
                    "stake": "646786097203475534304943885178",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "leadnode.pool.f863973.m0",
                    "public_key": "ed25519:CdP6CBFETfWYzrEedmpeqkR6rsJNeT22oUFn2mEDGk5i",
                    "stake": "644367778886663802105399198378",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "stakesstone.pool.f863973.m0",
                    "public_key": "ed25519:3aAdsKUuzZbjW9hHnmLWFRKwXjmcxsnLNLfNL4gP1wJ8",
                    "stake": "641198519157648602505664886163",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "basilisk-stake.pool.f863973.m0",
                    "public_key": "ed25519:CFo8vxoEUZoxbs87mGtG8qWUvSBHB91Vc6qWsaEXQ5cY",
                    "stake": "639918590440004706626411243128",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "shardlabs.pool.f863973.m0",
                    "public_key": "ed25519:DxmhGQZ6oqdxw7qGBvzLuBzE6XQjEh67hk5tt66vhLqL",
                    "stake": "637803882455578964186296090355",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "al3c5.pool.f863973.m0",
                    "public_key": "ed25519:BoYixTjyBePQ1VYP3s29rZfjtz1FLQ9og4FWZB5UgWCZ",
                    "stake": "636854880374440657378246667596",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "dehashed.pool.f863973.m0",
                    "public_key": "ed25519:EmPyD1DV9ajWJxjNN8GGACMyhM9w14brwNwYA5WvVaw",
                    "stake": "635224150718459403099965806552",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "machfund.pool.f863973.m0",
                    "public_key": "ed25519:G6fJ79oM6taQGhHeQZrg8N36nkCPMEVPyQMHfFT2wAKc",
                    "stake": "634686788251976758263963874506",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "blockngine.pool.f863973.m0",
                    "public_key": "ed25519:CZrTtCP6XkkxWtr3ATnXE8FL6bcG5cHcxfmdRgN7Lm7m",
                    "stake": "633656065475669726280826427959",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "grassets.pool.f863973.m0",
                    "public_key": "ed25519:3S4967Dt1VeeKrwBdTTR5tFEUFSwh17hEFLATRmtUNYV",
                    "stake": "622722987982775798532829252304",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "bflame.pool.f863973.m0",
                    "public_key": "ed25519:4uYM5RXgR9D6VAGKHgQTVNLEmCgMVX7PzpBstT92Me6R",
                    "stake": "617234461115345372278772960093",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "shurik.pool.f863973.m0",
                    "public_key": "ed25519:9zEn7DVpvQDxWdj5jSgrqJzqsLo8T9Wv37t83NXBiWi6",
                    "stake": "616327809807619407716759066614",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "dsrvlabs.pool.f863973.m0",
                    "public_key": "ed25519:61ei2efmmLkeDR1CG6JDEC2U3oZCUuC2K1X16Vmxrud9",
                    "stake": "613792106557214713239288385761",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "zetsi.pool.f863973.m0",
                    "public_key": "ed25519:6rYx5w1Z2pw46NBHv6Wo4JEUMNtqnDGqPaHT4wm15YRw",
                    "stake": "611882168159257611258042281605",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "n0ok.pool.f863973.m0",
                    "public_key": "ed25519:D6Gq2RpUoDUojmE2vLpqQzuZwYmFPW6rMcXPrwRYhqN8",
                    "stake": "594349395199079126466241101938",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "chelovek_iz_naroda.pool.f863973.m0",
                    "public_key": "ed25519:89aWsXXytjAZxyefXuGN73efnM9ugKTjPEGV4hDco8AZ",
                    "stake": "592739793772796190513231168872",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "lavenderfive.pool.f863973.m0",
                    "public_key": "ed25519:AzwAiLDqprZKpDjhsH7dfyvFdfSasmPTjuJUAHfX1Pg4",
                    "stake": "586231008421809079867645695624",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "latenthero.pool.f863973.m0",
                    "public_key": "ed25519:EQqmjRNouRKhwGL7Hnp3vcbDywg2Boj6to2gmnXybhEM",
                    "stake": "579738101137715103577294987834",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "tayang.pool.f863973.m0",
                    "public_key": "ed25519:G9XWX55MfWEpT84ckcsJxVTKeZK4WqBGJX3xVpnPB5vv",
                    "stake": "563498889920635651950224126233",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "smcvalidator.pool.f863973.m0",
                    "public_key": "ed25519:pG4LYsyoAa8yWYG9nsTQ5yBcwke51i3VqeRcMVbE9Q7",
                    "stake": "555422197586970576403131175346",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "everstake.pool.f863973.m0",
                    "public_key": "ed25519:4LDN8tZUTRRc4siGmYCPA67tRyxStACDchdGDZYKdFsw",
                    "stake": "546400197607367519956748211889",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "rossi-validator.pool.f863973.m0",
                    "public_key": "ed25519:2eRx2c3KX9wFd3EzuuajFQoSxRTKDqSbxcF13LfkrxCR",
                    "stake": "545396693549586230215202952473",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "infiniteloop.pool.f863973.m0",
                    "public_key": "ed25519:2fbiLqksH5viWXYoteyfKP9qQawkRKw4YogRFcvG3Z7f",
                    "stake": "538321976932135835213436874121",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "lusienda.pool.f863973.m0",
                    "public_key": "ed25519:HdQb2HEiaMgvUdemTt5rkrFbxTpzZyELvg1Vov4LQAGU",
                    "stake": "509015164869674763004419847436",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "ino.pool.f863973.m0",
                    "public_key": "ed25519:B75h2eqpaMgh6WkAvgnz2FsEC9s5TwVx7zwTjqXKfRs6",
                    "stake": "494974817444468749939621071716",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "pontiff.pool.f863973.m0",
                    "public_key": "ed25519:4i8j7nwNyy18hfARtrVpckT8MiicdCXuWBX1TubdMb5Y",
                    "stake": "478587210879643963063840990682",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "kiln.pool.f863973.m0",
                    "public_key": "ed25519:Bq8fe1eUgDRexX2CYDMhMMQBiN13j8vTAVFyTNhEfh1W",
                    "stake": "96608509421037438882028377566",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "nodemeister.pool.f863973.m0",
                    "public_key": "ed25519:85EMyaNGMFuHK2RDH7KHno6fVYBR6iykUXHPPmFTGuTB",
                    "stake": "47021543808070096585479049932",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "nala.pool.f863973.m0",
                    "public_key": "ed25519:Fzwndob2h5PFdEuwo9eRFJV3BLLurcNaw2SGob5rMPEn",
                    "stake": "44766587364445748049092546945",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "happystake.pool.f863973.m0",
                    "public_key": "ed25519:3APqZiwzeZLzgfkJyGGTfepDYHA2d8NF1wZi4mCpZnaJ",
                    "stake": "43959988855512773720415910025",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "ibb.pool.f863973.m0",
                    "public_key": "ed25519:7gvdHhcMcXT1jMZoxDKy7yXnRiPVX1tAFTa7HWTHbe8C",
                    "stake": "42001690004861681144621857517",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "mateennala.pool.f863973.m0",
                    "public_key": "ed25519:9kNpQKUKzhc1AiFSEoZcTNapTnywjbXBPngH3EDpD1tw",
                    "stake": "40056014128143748170300000000",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "wolfedge-capital-testnet.pool.f863973.m0",
                    "public_key": "ed25519:CQEMcPQz6sqhAgoBm9ka9UeVcXj5NpNpRtDYYGkPggvg",
                    "stake": "37464905110868615156797728096",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "jstaking.pool.f863973.m0",
                    "public_key": "ed25519:fui1E5XwnAWGYDBSQ3168aDfsW1KDFH8A7nBHvZiqGv",
                    "stake": "36368375383183646876651257216",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "dariya.pool.f863973.m0",
                    "public_key": "ed25519:A5Rx38TsNKWXzF5o18HpaRrPeBzv3riqur51bqhU1Qbp",
                    "stake": "36211347514033914937590010268",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "4ire-pool.pool.f863973.m0",
                    "public_key": "ed25519:EWPSvYN9pGPMmCLjVxx96stWdqksXNSGnfnuWYn9iiE5",
                    "stake": "33869896086305183386478534323",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "lionstake.pool.f863973.m0",
                    "public_key": "ed25519:Fy6quR4nBhrEnDyEuPWoAdBP5tzNbuEZsEd91Q5pQnXB",
                    "stake": "33765876364623459491244697143",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "zentriav2.factory.colorpalette.testnet",
                    "public_key": "ed25519:4rCwSFzJ2e6suD5Yi7pgLidcAJ8Zt9BXieLzVedJDwmE",
                    "stake": "30596434283244809799848018489",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "lastnode.pool.f863973.m0",
                    "public_key": "ed25519:811gesxXYdYeThry96ZiWn8chgWYNyreiScMkmxg4U9u",
                    "stake": "24146328727357015429360981746",
                    "validator_stake_struct_version": "V1"
                }
            ],
            "prev_block_hash": "9aHDvg6TV44qRSoaiYR98ZxaQNufs7vQXV6w6Jpy5oe9"
        },
        "id": "idontcare"
    }
    "#;

        const CLIENT_BLOCK_RESPONSE_NEXT_BLOCK: &str = r#"
    {
        "jsonrpc": "2.0",
        "result": {
            "approvals_after_next": [
                null,
                "ed25519:24pvVMA2ybxuk7fsCNAxDRnby5KQbGBM61T4Am74grDRuhiPbtYWBrubeSNWTejiAwiMZZt1zvLKSR8Djr4nDfHz",
                "ed25519:24pvVMA2ybxuk7fsCNAxDRnby5KQbGBM61T4Am74grDRuhiPbtYWBrubeSNWTejiAwiMZZt1zvLKSR8Djr4nDfHz",
                "ed25519:24pvVMA2ybxuk7fsCNAxDRnby5KQbGBM61T4Am74grDRuhiPbtYWBrubeSNWTejiAwiMZZt1zvLKSR8Djr4nDfHz",
                "ed25519:c78hanGiPzZ5iq9GPQET9pTh6J8pw5YgRGjtbNq35LuCzyTa5b4vdjzcAfHuRznfbTis77nF1aL6zm4CTJTesgU",
                "ed25519:65mYbzdjVUkWCh1wL81kZu96XphPP8X5McUVo2ScSKPgNiNBd3AsyR5XbJE7MGW5GnBwaqDPK8ft3yyRa3UMJnua",
                "ed25519:4akzNHFaa7w1LvaBTFKir9ExKStoRo44rm7YJ7XvtrigDnWmQ41EV7SyEFqcSqbDSznoxZybLQUV8ccCbia1daNT",
                "ed25519:4AGwZcCRk5WhCEuvEk12ANyJKHwwoLAPmGjU9Vqf7Xn7pDQcXw5sY8sPt3LazU7EYVaDUnZwWJUp2cAGHXkuLLyL",
                "ed25519:31anmHx3XEyPCnn7Mth5oppwbXoJbDQQjw3WjcDLGs8167RBE4WgCPaHn8kfHyhQ3tWWHudi1CFhy92yjJKPdLNK",
                "ed25519:4qLsTS1cF9ahcAnddUGjY7yFx2Sd8gJpp2dU3LjRpzT9vpG3grrGDcqCxVRAgjq7tEyuKXsbL7zyxVgpXjbicVwK",
                "ed25519:LtwgqDVPQWvomdx2zoXmfopgRgzLjxpovmjRXgetpZvc3E19iKjHbYtcs8FgGS4b6AT9GqgtoGfuLD1qdR94i2D",
                "ed25519:4pqiJapEEMS3czyAwM7QW4qecjT5u4EQkFa79rtCmSEACnKtxuU5PGBwsbZJkq7h8xhS24vN7d5AzszuKGWviNR9",
                "ed25519:38FxGgLRJMoD2cC3zX93c47iD51pGMvcVpPzCX9hSYfexJg8st7Ny4vr6U4sBfiyLeToqJTuoobEUzEts2eZXBxa",
                "ed25519:2Srg7nZ29C8ySxMKkMFXzFXj5i1RL4NGQ19GQryMmHcEYSVLFGBauhjydbEtaEQ5tzpZpMFPeGLnyV88GQSUHcPA",
                null,
                "ed25519:2LX76ZV8iB7ZyaVAAtpUjQDwKAshix3zLk9X5kF6gsn7oqMT6Rw6Mns3HZkD9M4mmMGEiUQqETw8P36Kymb1GUjb",
                "ed25519:2e39nRRS97kvfkjjGohtggubeTBGX8sqGSuL83nH1PYWDGoSANcUZqYeWZKxy8dzW44HEc9ptHYgGsynf6m4RY5S",
                "ed25519:2s6yBzV5D8VFS9hDsyqwHw3QQu4mQhjq4R6VHYMVXgbbogCe11eP4xZYUtw44gZrPawV3yxeqQH2RQFngdw5fABH",
                "ed25519:5kZ6EbdnhfwxdzwhEBarnMeNi5ng2UujvYNkafUVEeN39Bbap3WgiNz1j697WrW9Zw1HNNu4ZEGxk4ad96Z3e6rB",
                "ed25519:3qXYatuMPnxyRKyytzSBTtRxQ38Sm42Asf9jDo5MPoNnQVhiHBdiAydZWyrKfdqgHnibVc6Xxh6yPzSQxK67xCFu",
                null,
                "ed25519:5AnKpS5LaHayrW8pFoCkNugAEfSvniMJEzCSq1u4NPYTrkzsrLRiQ2SANVwA3PkXJrz6hd1abQCCWNhJMPsNNHQu",
                "ed25519:5D867Gg5xv9XiBWXMxhzx3cfY41moU5g7E62PyQLrEmvLSL5px67ojzasVd4whdqF3CzkN8wuuzGi2vvqqPNLPkr",
                "ed25519:5Bi9FH8gmnncJJNjpcNQx3AV12VpmF6Mk3pCGVvitBUrMMAKrYUEHh3knZpGJVWCVjP4TyxfKRwvCGVH2VNbHXMF",
                "ed25519:2J13vtY7vzxREcYQU4micZMpNskakdbvbxC5CMUnf4BSRf6my2nQ5g77GWSH4DNC9FTSW6ZQACJHfXyy9opMqLfT",
                "ed25519:sLUzhmxwGgVRLePVBvwMrW3Ny7E2ftWVnRAntbqF4sempASFMbwhjHvcBUfmNtJUSL9Qc5gEwbDgMMrriEuK2JK",
                "ed25519:2547xqoEPW9hR2Jh5FDgZDvuxmacMdPUdq4mqpJt18StSLoWN3B5ojSztBMdRaRNga5DWneNL9GViB712BJqYksh",
                "ed25519:3XKJohJC5Vr79FM5aVfjGkBP2Ck8hNEtEemmMRKfNh8NLQyVAED8rqyHhxSG2G7tnmt37tUgaZQcyNaQe9AC7zB3",
                "ed25519:wRkBDZg2MyGHZqhGYq8Pyv3uvu15jghNVWgdhmtsE5CFqzp4ws2YTCRnS4KUe3U7canCByh9hJHGetj9EHaGUop",
                null,
                "ed25519:5MiiKrUGXpcZB9VDVxUSpvKumFR9yiZgWEsVjHK4erJ4JEfzd7M17KnaaDWdLWn3w23drpqBLZCLxP6d1FqcvWP2",
                "ed25519:3WC83k8v1AqtK8QzUsNFWKSQrTNxbfxmtm19xDwnTDG5W22uzYb232eBAwALFqZjSbNifr1DXd25fyE7msM6kfjc",
                "ed25519:3UvniTBSgJPp8Mv1b8Z39pj7DSKZ3Epfy5xC7Mo4SAFDhgTZ7rxABtjT7tKj1S73JoREkzvdW2H1zRfeCRpWYyMU",
                "ed25519:36XryCFKF9tVv4x6FhaJT8iYfrZPCFYbjnmteNkzXQcJRiRp2MwivcYpvrkUUzFYMDuN4uSdYgozs3uPqgKha6Mb",
                null,
                null,
                "ed25519:5znLSC9mJRDEt5ozPc9cBisW5fn3matgmEcNBQtvvgpNNGgXYxHzW8aJkTmrovXRDiyWDbwkpY3GYbqPy66zGNSW",
                "ed25519:35uHJvJ8cmQGxHWsPjkBMg4SCmDEEYmLgUuqvQZBarqx5uck6apdi3SRp3AgSPDzT8tFuGCiXz8EHByHjDmoGbiz",
                "ed25519:45Qq5tSNJbzphyGerqEKCsEBq8bmrza5aferuEEijmhdgATdt6f4RDE9PDc86AwdURTLd7UVerkTtHheofa2YJet",
                "ed25519:3G1gba8V5YsFdKnQmwpGfcy47J6etLeBz57oZwdrRnqWboQd15TRzJxzfmrMgMn415CpwLFq3iXWBrUUA2B3ZiPe",
                null,
                null,
                "ed25519:5u63DbmzPiyB1R7DeCpKUAN1fJUTUpmh7FieGm6w1JWcJtHcD3EkMQGs5eoQ4XZbStACc3f9CSeQrz13hm7B2ipN",
                "ed25519:2vgaVE77b38bFYNJort92hRJQrtxZp13vLCK6WQqs9cbWAQfk5pMnPcUZis2z9rbk411QhmtWo9WPHHspAgMUEaf",
                "ed25519:TdUvco7vQAXqor6fBcwPBhyDaVKffYXknRB3T7cyWwDBNJ9etJtNje7wL6oQmUkQwndqwzKscNg8nKN38M5Fzdt",
                "ed25519:dYFyNK7uNQECEXzrj4eQZAGdkeKrVxnsR6u3rRDE43uJTBf1tXPffspeonwMuFx9DqsGg4DSRy6hPPmzdNQruNz",
                null,
                null,
                null,
                "ed25519:5vsSMabj5pz7um6fvVKwF6WyJvsaEZ8YjyeqgSxSkZGWB2Zm2yaV7QqzTnzurx4KT7Zhdvow4HjA3hBWt8Wt1ti1",
                null,
                null,
                "ed25519:3HfQt71AT6iVygpeNBHUaimx3iNnApbfTSsL5u65uyzkSHPCYwwSoQ7GfUCSuMp7HAm1cvpTf8RxzimKu9WeGa88",
                null,
                "ed25519:5ds369kTT4eUM1gcebAuuShPtft7LEZTA5oDwsxVv3Bazpb15WmDhSRuhUztGVTpDwMXijs68Gt7kUu3bD45KJpH",
                null,
                null,
                null,
                null,
                null,
                "ed25519:q7c7Mu5mKvZuBfpeRChGMbL1BZuwv72k2YvF9QoQHZE1yMRYnyQxvnAuHMiYLnnqKyD9PKA9ncssJfZcoL6jV2a",
                "ed25519:5u6PWvtS88g13Z5aS6y57uBfuDXvXTw7Hr9ZaxcicadfyHZCft9tq71heoUva9ewZLznWsCBy7JCJ7m75JG1CDPA",
                "ed25519:xjH8MR2JGH9ofpFPaU8GcoidsAohePQjtSi4M7T6SgXC1qZzhst6WLuguBaKTBqoPZU75N2Kkztfv3SKdyJKUQm",
                "ed25519:3nVu8XDkNep3UDWF8QVvf9NvfL37Z5DBFEXJ6VbsosRjpH8NSuU8DquYfd55rfDNHDUxY1yerk1grz1GvorhUon6",
                "ed25519:28qtBZAsbnBiZ6wPynzgTXuRm2fB4SPGmeBgyrfY61VSCNkr7LZ5zwLUxhrDUGQnLiaVS13tU9eBECvbocGZBgXE",
                null,
                "ed25519:M8ybFBsk3xZuXE48RwxSCVwyZB2srJVQ85cWazneyc1SQzuHXciKzouw3NXzwussKvpvvV4jsyPyEosfVmifnMm",
                "ed25519:38yS9p1AcoXiS7E4EMn9gpCvAppCrdvygvDQwnP5VTjHahTbyGLV67mre1k4x9TZ2JD36sffYZzh5BBgaXpNSCJF",
                "ed25519:G9H34TNeTP5QgifK9a6Y8PQVpnM5x7V2M7zSzsYCjdc1GUVsFFrMWPiJsigKnrV5pKi6yvWFUDwhYgXUPGGKZiY",
                "ed25519:4tC17LadtbHChDDvEJaGrsmc1Jj7F6PT7GQq9Ncd8tykG5tNYxfA9kXz57tvwRbzeZjqjPykAPY2KrN4XMs4M9sB"
            ],
            "inner_lite": {
                "block_merkle_root": "3MBnipBo8GnqJisZN3uFjHLuvMusBSCjMaQmUmj5u4J6",
                "epoch_id": "GHmqgUX59irTdh31mtuEs3uEaPNBY5sQTZjEX5w7ASgW",
                "height": 86456070,
                "next_bp_hash": "9VPzyStHi4X2T7VAbfSTbLXEd8vjFP7wFJjYyjSJxQik",
                "next_epoch_id": "8nVTHDfxg2G8AWbKhVfFtnEb5jJeiXV2XBFsdyt2cif1",
                "outcome_root": "56KJ7kyW7aADwfDdNE4fz7pmPccBqkmxvnJ3nR1fewop",
                "prev_state_root": "2VJekkjBnP36c3sGo9P2YxkEu9dabK9r5VdMTt7jADLv",
                "timestamp": 1648810204507638699,
                "timestamp_nanosec": "1648810204507638699"
            },
            "inner_rest_hash": "GQHrWtXByznAWcawC7GoEMumZ3GUi2T82MXV56c2x8KS",
            "next_block_inner_hash": "8rHAfAgpXQKXTDWwEPvHwzKaBjG67nv4eNGSfc7A8FZ5",
            "next_bps": [
                {
                    "account_id": "node1",
                    "public_key": "ed25519:ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su",
                    "stake": "22949327592242450816363151898853",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "node0",
                    "public_key": "ed25519:ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su",
                    "stake": "16944923507607057621836326590864",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "node2",
                    "public_key": "ed25519:ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su",
                    "stake": "16894243398827941870356919783063",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "node3",
                    "public_key": "ed25519:ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su",
                    "stake": "8577838094223400746241842212915",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "legends.pool.f863973.m0",
                    "public_key": "ed25519:AhQ6sUifJYgjqarXSAzdDZU9ZixpUesP9JEH1Vr7NbaF",
                    "stake": "5793326871499643941084500854531",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "staked.pool.f863973.m0",
                    "public_key": "ed25519:D2afKYVaKQ1LGiWbMAZRfkKLgqimTR74wvtESvjx5Ft2",
                    "stake": "4559762052294055739961541809028",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "masternode24.pool.f863973.m0",
                    "public_key": "ed25519:9E3JvrQN6VGDGg1WJ3TjBsNyfmrU6kncBcDvvJLj6qHr",
                    "stake": "3416574120678826701003147150326",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "01node.pool.f863973.m0",
                    "public_key": "ed25519:3iNqnvBgxJPXCxu6hNdvJso1PEAc1miAD35KQMBCA3aL",
                    "stake": "3061276782639300406837420592214",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "p2p.pool.f863973.m0",
                    "public_key": "ed25519:4ie5979JdSR4f7MRAG58eghRxndVoKnAYAKa1PLoMYSS",
                    "stake": "2958427611565856637171061933942",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "nodeasy.pool.f863973.m0",
                    "public_key": "ed25519:25Dhg8NBvQhsVTuugav3t1To1X1zKiomDmnh8yN9hHMb",
                    "stake": "1575068818350064235628643461649",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "tribe-pool.pool.f863973.m0",
                    "public_key": "ed25519:CRS4HTSAeiP8FKD3c3ZrCL5pC92Mu1LQaWj22keThwFY",
                    "stake": "1429199212043501677779067532132",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "chorusone.pool.f863973.m0",
                    "public_key": "ed25519:3TkUuDpzrq75KtJhkuLfNNJBPHR5QEWpDxrter3znwto",
                    "stake": "1278827676875609593894511486301",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "hotones.pool.f863973.m0",
                    "public_key": "ed25519:2fc5xtbafKiLtxHskoPL2x7BpijxSZcwcAjzXceaxxWt",
                    "stake": "1273529881837124230828073909315",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "foundryusa.pool.f863973.m0",
                    "public_key": "ed25519:ABGnMW8c87ZKWxvZLLWgvrNe72HN7UoSf4cTBxCHbEE5",
                    "stake": "1256081604638924285747937189845",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "lunanova2.pool.f863973.m0",
                    "public_key": "ed25519:9Jv6e9Kye4wM9EL1XJvXY8CYsLi1HLdRKnTzXBQY44w9",
                    "stake": "1247431491303762172509349058430",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "chorus-one.pool.f863973.m0",
                    "public_key": "ed25519:6LFwyEEsqhuDxorWfsKcPPs324zLWTaoqk4o6RDXN7Qc",
                    "stake": "1110429050842727763339891353120",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "ni.pool.f863973.m0",
                    "public_key": "ed25519:GfCfFkLk2twbAWdsS3tr7C2eaiHN3znSfbshS5e8NqBS",
                    "stake": "1076903268858699791106964347506",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "cryptogarik.pool.f863973.m0",
                    "public_key": "ed25519:FyFYc2MVwgitVf4NDLawxVoiwUZ1gYsxGesGPvaZcv6j",
                    "stake": "840652974653901124214299092043",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "pathrocknetwork.pool.f863973.m0",
                    "public_key": "ed25519:CGzLGZEMb84nRSRZ7Au1ETAoQyN7SQXQi55fYafXq736",
                    "stake": "749739988926667488225409312930",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "stakely_v2.pool.f863973.m0",
                    "public_key": "ed25519:7BanKZKGvFjK5Yy83gfJ71vPhqRwsDDyVHrV2FMJCUWr",
                    "stake": "734779467803676488422251769143",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "solidstate.pool.f863973.m0",
                    "public_key": "ed25519:DTDhqoMXDWhKedWpH7DPvR6dPDcXrk5pTHJw2bkFFvQy",
                    "stake": "715205657993906057594050568659",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "aurora.pool.f863973.m0",
                    "public_key": "ed25519:9c7mczZpNzJz98V1sDeGybfD4gMybP4JKHotH8RrrHTm",
                    "stake": "703162032315675728652111978820",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "namdokmai.pool.f863973.m0",
                    "public_key": "ed25519:9uGeeM7j1fimpG7vn6EMcBXMei8ttWCohiMf44SoTzaz",
                    "stake": "699426128043696790256527911933",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "freshtest.pool.f863973.m0",
                    "public_key": "ed25519:5cbAt8uzmRztXWXKUYivtLsT2kMC414oHYDapfSJcgwv",
                    "stake": "697072950038835725218153979145",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "optimusvalidatornetwork.pool.f863973.m0",
                    "public_key": "ed25519:BGoxGmpvN7HdUSREQXfjH6kw5G6ph7NBXVfBVfUSH85V",
                    "stake": "661182931526239970852421432715",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "baziliknear.pool.f863973.m0",
                    "public_key": "ed25519:9Rbzfkhkk6RSa1HoPnJXS4q2nn1DwYeB4HMfJBB4WQpU",
                    "stake": "651150213650042597898598894903",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "blockscope.pool.f863973.m0",
                    "public_key": "ed25519:6K6xRp88BCQX5pcyrfkXDU371awMAmdXQY4gsxgjKmZz",
                    "stake": "649506414222131713576984442889",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "tagard.pool.f863973.m0",
                    "public_key": "ed25519:3KyziFgx3PpzorJnMFifXU4KsK4nwPFaxCGWTHaFBADK",
                    "stake": "646786097203475534304943885178",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "leadnode.pool.f863973.m0",
                    "public_key": "ed25519:CdP6CBFETfWYzrEedmpeqkR6rsJNeT22oUFn2mEDGk5i",
                    "stake": "644367778886663802105399198378",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "stakesstone.pool.f863973.m0",
                    "public_key": "ed25519:3aAdsKUuzZbjW9hHnmLWFRKwXjmcxsnLNLfNL4gP1wJ8",
                    "stake": "641198519157648602505664886163",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "basilisk-stake.pool.f863973.m0",
                    "public_key": "ed25519:CFo8vxoEUZoxbs87mGtG8qWUvSBHB91Vc6qWsaEXQ5cY",
                    "stake": "639918590440004706626411243128",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "shardlabs.pool.f863973.m0",
                    "public_key": "ed25519:DxmhGQZ6oqdxw7qGBvzLuBzE6XQjEh67hk5tt66vhLqL",
                    "stake": "637803882455578964186296090355",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "al3c5.pool.f863973.m0",
                    "public_key": "ed25519:BoYixTjyBePQ1VYP3s29rZfjtz1FLQ9og4FWZB5UgWCZ",
                    "stake": "636854880374440657378246667596",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "dehashed.pool.f863973.m0",
                    "public_key": "ed25519:EmPyD1DV9ajWJxjNN8GGACMyhM9w14brwNwYA5WvVaw",
                    "stake": "635224150718459403099965806552",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "machfund.pool.f863973.m0",
                    "public_key": "ed25519:G6fJ79oM6taQGhHeQZrg8N36nkCPMEVPyQMHfFT2wAKc",
                    "stake": "634686788251976758263963874506",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "blockngine.pool.f863973.m0",
                    "public_key": "ed25519:CZrTtCP6XkkxWtr3ATnXE8FL6bcG5cHcxfmdRgN7Lm7m",
                    "stake": "633656065475669726280826427959",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "grassets.pool.f863973.m0",
                    "public_key": "ed25519:3S4967Dt1VeeKrwBdTTR5tFEUFSwh17hEFLATRmtUNYV",
                    "stake": "622722987982775798532829252304",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "bflame.pool.f863973.m0",
                    "public_key": "ed25519:4uYM5RXgR9D6VAGKHgQTVNLEmCgMVX7PzpBstT92Me6R",
                    "stake": "617234461115345372278772960093",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "shurik.pool.f863973.m0",
                    "public_key": "ed25519:9zEn7DVpvQDxWdj5jSgrqJzqsLo8T9Wv37t83NXBiWi6",
                    "stake": "616327809807619407716759066614",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "dsrvlabs.pool.f863973.m0",
                    "public_key": "ed25519:61ei2efmmLkeDR1CG6JDEC2U3oZCUuC2K1X16Vmxrud9",
                    "stake": "613792106557214713239288385761",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "zetsi.pool.f863973.m0",
                    "public_key": "ed25519:6rYx5w1Z2pw46NBHv6Wo4JEUMNtqnDGqPaHT4wm15YRw",
                    "stake": "611882168159257611258042281605",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "n0ok.pool.f863973.m0",
                    "public_key": "ed25519:D6Gq2RpUoDUojmE2vLpqQzuZwYmFPW6rMcXPrwRYhqN8",
                    "stake": "594349395199079126466241101938",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "chelovek_iz_naroda.pool.f863973.m0",
                    "public_key": "ed25519:89aWsXXytjAZxyefXuGN73efnM9ugKTjPEGV4hDco8AZ",
                    "stake": "592739793772796190513231168872",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "lavenderfive.pool.f863973.m0",
                    "public_key": "ed25519:AzwAiLDqprZKpDjhsH7dfyvFdfSasmPTjuJUAHfX1Pg4",
                    "stake": "586231008421809079867645695624",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "latenthero.pool.f863973.m0",
                    "public_key": "ed25519:EQqmjRNouRKhwGL7Hnp3vcbDywg2Boj6to2gmnXybhEM",
                    "stake": "579738101137715103577294987834",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "tayang.pool.f863973.m0",
                    "public_key": "ed25519:G9XWX55MfWEpT84ckcsJxVTKeZK4WqBGJX3xVpnPB5vv",
                    "stake": "563498889920635651950224126233",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "smcvalidator.pool.f863973.m0",
                    "public_key": "ed25519:pG4LYsyoAa8yWYG9nsTQ5yBcwke51i3VqeRcMVbE9Q7",
                    "stake": "555422197586970576403131175346",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "everstake.pool.f863973.m0",
                    "public_key": "ed25519:4LDN8tZUTRRc4siGmYCPA67tRyxStACDchdGDZYKdFsw",
                    "stake": "546400197607367519956748211889",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "rossi-validator.pool.f863973.m0",
                    "public_key": "ed25519:2eRx2c3KX9wFd3EzuuajFQoSxRTKDqSbxcF13LfkrxCR",
                    "stake": "545396693549586230215202952473",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "infiniteloop.pool.f863973.m0",
                    "public_key": "ed25519:2fbiLqksH5viWXYoteyfKP9qQawkRKw4YogRFcvG3Z7f",
                    "stake": "538321976932135835213436874121",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "lusienda.pool.f863973.m0",
                    "public_key": "ed25519:HdQb2HEiaMgvUdemTt5rkrFbxTpzZyELvg1Vov4LQAGU",
                    "stake": "509015164869674763004419847436",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "ino.pool.f863973.m0",
                    "public_key": "ed25519:B75h2eqpaMgh6WkAvgnz2FsEC9s5TwVx7zwTjqXKfRs6",
                    "stake": "494974817444468749939621071716",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "pontiff.pool.f863973.m0",
                    "public_key": "ed25519:4i8j7nwNyy18hfARtrVpckT8MiicdCXuWBX1TubdMb5Y",
                    "stake": "478587210879643963063840990682",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "kiln.pool.f863973.m0",
                    "public_key": "ed25519:Bq8fe1eUgDRexX2CYDMhMMQBiN13j8vTAVFyTNhEfh1W",
                    "stake": "96608509421037438882028377566",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "nodemeister.pool.f863973.m0",
                    "public_key": "ed25519:85EMyaNGMFuHK2RDH7KHno6fVYBR6iykUXHPPmFTGuTB",
                    "stake": "47021543808070096585479049932",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "nala.pool.f863973.m0",
                    "public_key": "ed25519:Fzwndob2h5PFdEuwo9eRFJV3BLLurcNaw2SGob5rMPEn",
                    "stake": "44766587364445748049092546945",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "happystake.pool.f863973.m0",
                    "public_key": "ed25519:3APqZiwzeZLzgfkJyGGTfepDYHA2d8NF1wZi4mCpZnaJ",
                    "stake": "43959988855512773720415910025",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "ibb.pool.f863973.m0",
                    "public_key": "ed25519:7gvdHhcMcXT1jMZoxDKy7yXnRiPVX1tAFTa7HWTHbe8C",
                    "stake": "42001690004861681144621857517",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "mateennala.pool.f863973.m0",
                    "public_key": "ed25519:9kNpQKUKzhc1AiFSEoZcTNapTnywjbXBPngH3EDpD1tw",
                    "stake": "40056014128143748170300000000",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "wolfedge-capital-testnet.pool.f863973.m0",
                    "public_key": "ed25519:CQEMcPQz6sqhAgoBm9ka9UeVcXj5NpNpRtDYYGkPggvg",
                    "stake": "37464905110868615156797728096",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "jstaking.pool.f863973.m0",
                    "public_key": "ed25519:fui1E5XwnAWGYDBSQ3168aDfsW1KDFH8A7nBHvZiqGv",
                    "stake": "36368375383183646876651257216",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "dariya.pool.f863973.m0",
                    "public_key": "ed25519:A5Rx38TsNKWXzF5o18HpaRrPeBzv3riqur51bqhU1Qbp",
                    "stake": "36211347514033914937590010268",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "4ire-pool.pool.f863973.m0",
                    "public_key": "ed25519:EWPSvYN9pGPMmCLjVxx96stWdqksXNSGnfnuWYn9iiE5",
                    "stake": "33869896086305183386478534323",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "lionstake.pool.f863973.m0",
                    "public_key": "ed25519:Fy6quR4nBhrEnDyEuPWoAdBP5tzNbuEZsEd91Q5pQnXB",
                    "stake": "33765876364623459491244697143",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "zentriav2.factory.colorpalette.testnet",
                    "public_key": "ed25519:4rCwSFzJ2e6suD5Yi7pgLidcAJ8Zt9BXieLzVedJDwmE",
                    "stake": "30596434283244809799848018489",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "lastnode.pool.f863973.m0",
                    "public_key": "ed25519:811gesxXYdYeThry96ZiWn8chgWYNyreiScMkmxg4U9u",
                    "stake": "24146328727357015429360981746",
                    "validator_stake_struct_version": "V1"
                }
            ],
            "prev_block_hash": "4E2VN7cUVSb8ek761H4cRo57ERTWBKbcB9uEBDS2cWhD"
        },
        "id": "idontcare"
    }
    "#;
        let near_client_block_view_checkpoint =
            get_client_block_view(CLIENT_RESPONSE_PREVIOUS_EPOCH).unwrap();

        let client_block_view_checkpoint = LightClientBlockView::try_from_slice(
            near_client_block_view_checkpoint
                .try_to_vec()
                .unwrap()
                .as_ref(),
        )
        .unwrap();

        let near_client_block_view = get_client_block_view(CLIENT_BLOCK_RESPONSE).unwrap();
        let client_block_view = LightClientBlockView::try_from_slice(
            near_client_block_view.try_to_vec().unwrap().as_ref(),
        )
        .unwrap();
        let near_client_block_view_next_epoch =
            get_client_block_view(CLIENT_BLOCK_RESPONSE_NEXT_BLOCK).unwrap();

        let client_block_view_next_epoch = LightClientBlockView::try_from_slice(
            near_client_block_view_next_epoch
                .try_to_vec()
                .unwrap()
                .as_ref(),
        )
        .unwrap();

        let mut light_client =
            LessDummyLiteClient::new_from_checkpoint(client_block_view_checkpoint);
        assert!(light_client
            .validate_and_update_head(&client_block_view)
            .unwrap());
        assert!(light_client
            .validate_and_update_head(&client_block_view_next_epoch)
            .unwrap());
        // previous epoch should fail
        assert!(!light_client
            .validate_and_update_head(&client_block_view)
            .unwrap());
    }
}
