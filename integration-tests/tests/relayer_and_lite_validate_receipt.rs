use near_lite_relayer::blockchain_connector::{BlockchainConnector, NearNetwork};

use borsh::{BorshDeserialize, BorshSerialize};
use near_lite_client::{
    LightClient, LightClientBlockView, StateTransitionVerificator, TrustedCheckpoint,
};
use near_lite_relayer::coerce;

/// ## Both Relayer and Lite Client - testing receipt validation
///
/// Finds one transaction from the last 300 blocks
/// If the transaction has a receipt, validates that the receipt is part of a
/// valid block that the lite client has already validates.
#[tokio::test]
async fn both_relayer_and_lite_client_validate_receipt() {
    env_logger::init();

    let blockchain_connector = BlockchainConnector::new(NearNetwork::Testnet);

    let (almost_last_block_hash, almost_latest_height) = blockchain_connector
        .get_almost_latest_finalized_block_hash_and_height()
        .unwrap();

    let light_client_block_view = blockchain_connector
        .get_light_client_block_view(almost_last_block_hash)
        .unwrap();

    let serialized_block_view = light_client_block_view.clone().try_to_vec().unwrap();
    let block_view_for_lite_client =
        LightClientBlockView::try_from_slice(&serialized_block_view).unwrap();

    let trusted_checkpoint = TrustedCheckpoint(block_view_for_lite_client);
    let lite_client = LightClient::new_from_checkpoint(trusted_checkpoint, 10);
    // find a transaction in a block that has been validated
    let mut height = almost_latest_height - 500;

    'l: loop {
        let chunk_ids = blockchain_connector
            .find_chunk_ids_with_burned_gas(height)
            .unwrap();

        for chunk_id in chunk_ids {
            let tx_info = blockchain_connector
                .get_transaction_ids_in_chunk(chunk_id)
                .unwrap();
            if !tx_info.is_empty() {
                let (tx_hash, sender_id) = tx_info[0].clone();

                let status = blockchain_connector
                    .get_transaction_status_view(tx_hash, sender_id.clone())
                    .unwrap();
                if let Some(receipt) = status.receipts.first() {
                    let tx_light_client_rcpt_proof = blockchain_connector
                        .get_light_client_proof_transaction(
                            almost_last_block_hash,
                            coerce(&receipt.receipt_id),
                            sender_id.clone(),
                        )
                        .unwrap();

                    let receipt_outcome = status.final_outcome.receipts_outcome.first().unwrap();
                    assert_eq!(
                        &receipt_outcome.proof,
                        &tx_light_client_rcpt_proof.outcome_proof.proof,
                    );

                    let expected_block_outcome_root = tx_light_client_rcpt_proof
                        .block_header_lite
                        .inner_lite
                        .outcome_root;

                    lite_client
                        .validate_transaction(
                            &coerce(tx_light_client_rcpt_proof.outcome_proof),
                            coerce(tx_light_client_rcpt_proof.outcome_root_proof),
                            coerce(expected_block_outcome_root),
                        )
                        .unwrap();

                    let _receipt_view = blockchain_connector
                        .get_receipt_view(coerce(&receipt.receipt_id))
                        .unwrap();
                    break 'l;
                }
            }
        }
        height -= 1;
        assert!(
            almost_latest_height < height + 500 + 300,
            "stopping the test - there should be at least one tx in 300 blocks"
        );
    }
}
