use near_lite_relayer::blockchain_connector::{BlockchainConnector, NearNetwork};
use std::str::FromStr;

use near_lite_client::{
    CryptoHash, LightClient, LightClientBlockView, StateTransitionVerificator, TrustedCheckpoint,
};
use near_lite_relayer::coerce;
use near_primitives::types::{BlockId, BlockReference};
use near_primitives::views::ExecutionStatusView;

/// ## Both Relayer and Lite Client - testing receipt validation
///
/// Finds one transaction from the last 300 blocks
/// If the transaction has a receipt, validates that the receipt is part of a
/// finalized block that the lite client has already validated.
#[tokio::test]
async fn both_relayer_and_lite_client_validate_receipt() {
    env_logger::init();

    let blockchain_connector = BlockchainConnector::new(NearNetwork::Testnet);

    let (finalized_block_hash, finalized_height) = blockchain_connector
        .get_almost_latest_finalized_block_hash_and_height()
        .unwrap();

    dbg!(&finalized_block_hash);
    let light_client_block_view = blockchain_connector
        .get_light_client_block_view(finalized_block_hash)
        .unwrap();

    let block_view_for_lite_client: LightClientBlockView = coerce(light_client_block_view);

    let trusted_checkpoint = TrustedCheckpoint(block_view_for_lite_client);
    let lite_client = LightClient::new_from_checkpoint(trusted_checkpoint, 10);
    // find a transaction in a block that has been validated
    let mut height = finalized_height - 500;

    'l: loop {
        let chunk_ids = blockchain_connector
            .find_chunk_ids_with_burned_gas(height - 50)
            .unwrap();

        for chunk_id in chunk_ids {
            dbg!(&chunk_id);
            let tx_info = blockchain_connector
                .get_transaction_ids_in_chunk(chunk_id)
                .unwrap();
            if tx_info.is_empty() {
                continue;
            }
            let (tx_hash, sender_id) = tx_info[0].clone();
            dbg!(&tx_hash);
            let status = blockchain_connector
                .get_transaction_status_view(coerce(tx_hash), sender_id.clone())
                .unwrap();

            for receipt_outcome in status.final_outcome.receipts_outcome {
                if let ExecutionStatusView::SuccessValue(..) = receipt_outcome.outcome.status {
                    if receipt_outcome.outcome.gas_burnt == 0 {
                        continue;
                    }
                    let receipt_block = blockchain_connector
                        .get_block(BlockReference::BlockId(BlockId::Hash(
                            receipt_outcome.block_hash,
                        )))
                        .unwrap();
                    if receipt_block.header.height > finalized_height {
                        continue;
                    }

                    // for rcpt_id in receipt_outcome.outcome.receipt_ids {
                    //     let sub_receipt_view = blockchain_connector
                    //         .get_receipt_view(coerce(&rcpt_id))
                    //         .unwrap();
                    //     sub_receipt_view.receipt
                    //     // sub_receipt_view.
                    // }

                    let rcpt_light_client_proof = blockchain_connector
                        .get_light_client_proof_receipt(
                            finalized_block_hash,
                            coerce(&receipt_outcome.id),
                            receipt_outcome.outcome.executor_id.as_str().to_owned(),
                        )
                        .unwrap();

                    assert_eq!(
                        &receipt_outcome.proof,
                        &rcpt_light_client_proof.outcome_proof.proof,
                    );

                    let expected_block_outcome_root = rcpt_light_client_proof
                        .block_header_lite
                        .inner_lite
                        .outcome_root;

                    dbg!(&receipt_outcome);
                    assert!(lite_client
                        .validate_transaction(
                            &coerce(rcpt_light_client_proof.outcome_proof),
                            coerce(rcpt_light_client_proof.outcome_root_proof),
                            coerce(expected_block_outcome_root),
                        )
                        .unwrap());

                    // let _receipt_view = blockchain_connector
                    //     .get_receipt_view(coerce(&receipt_outcome.id))
                    //     .unwrap();
                    // break 'l;
                }
            }
        }
        height -= 1;
        assert!(
            finalized_height < height + 500 + 300,
            "stopping the test - there should be at least one tx in 300 blocks"
        );
    }
}

#[tokio::test]
async fn tst() {
    env_logger::init();

    let blockchain_connector = BlockchainConnector::new(NearNetwork::Testnet);
    let finalized_block_hash =
        coerce(CryptoHash::from_str("Ef8w2PPdXX17ckF4n7YFULpHo2pmjfQTbXM5msjnqYqj").unwrap());
    // let finalized_height = 95528808;

    let light_client_block_view = blockchain_connector
        .get_light_client_block_view(finalized_block_hash)
        .unwrap();

    let block_view_for_lite_client: LightClientBlockView = coerce(light_client_block_view);
    let trusted_checkpoint = TrustedCheckpoint(block_view_for_lite_client);
    let lite_client = LightClient::new_from_checkpoint(trusted_checkpoint, 10);

    // let block_hash =
    //     coerce(CryptoHash::from_str("F7ph3PmGVdrgUY8eYN18kzMoNxT99bs5G7PCPNk76CFB").unwrap());
    // let receipt_block = blockchain_connector
    //     .get_block(BlockReference::BlockId(BlockId::Hash(block_hash)))
    //     .unwrap();

    let id = CryptoHash::from_str("2eEFRvegpRFaXJRMoVvsYnSynGso2fawiUQs8frXpP4o").unwrap();
    let executor_id = "plats-network.registry.test_oct.testnet".to_owned();
    let rcpt_light_client_proof = blockchain_connector
        .get_light_client_proof_receipt(finalized_block_hash, coerce(&id), executor_id)
        .unwrap();

    let expected_block_outcome_root = rcpt_light_client_proof
        .block_header_lite
        .inner_lite
        .outcome_root;

    assert!(lite_client
        .validate_transaction(
            &dbg!(coerce(rcpt_light_client_proof.outcome_proof)),
            dbg!(coerce(rcpt_light_client_proof.outcome_root_proof)),
            dbg!(coerce(expected_block_outcome_root)),
        )
        .unwrap());
}
