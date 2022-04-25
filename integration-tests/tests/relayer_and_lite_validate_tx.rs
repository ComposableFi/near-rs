use std::{thread::park_timeout, time::Duration};

use near_lite_relayer::{
    blockchain_connector::{BlockchainConnector, NearNetwork},
    state::LightClientState,
};

use borsh::{BorshDeserialize, BorshSerialize};
use near_lite_client::{
    CryptoHash, LightClient, MerklePath, OutcomeProof, StateTransitionVerificator,
    TrustedCheckpoint,
};

/// ## Both Relayer and Lite Client
///
/// Note: we're testing happy path only here
///
/// Connects to Testnet using the relayer logic from `near-lite-relayer`
/// Serializes the data, and feeds it into the light client
/// Validates that the block is correct
///
#[tokio::test]
async fn both_relayer_and_lite_client_validate_tx() {
    env_logger::init();

    let blockchain_connector = BlockchainConnector::new(NearNetwork::Testnet);

    let (almost_last_block_hash, almost_latest_height) = blockchain_connector
        .get_almost_latest_finalized_block_hash_and_height()
        .unwrap();

    dbg!(bs58::encode(almost_last_block_hash.try_to_vec().unwrap()).into_string());

    let light_client_block_view = blockchain_connector
        .get_light_client_block_view(almost_last_block_hash)
        .unwrap();

    let serialized_block_view = light_client_block_view.try_to_vec().unwrap();
    let block_view_for_lite_client =
        BorshDeserialize::try_from_slice(&serialized_block_view).unwrap();

    let trusted_checkpoint = TrustedCheckpoint(block_view_for_lite_client);
    let mut lite_client = LightClient::new_from_checkpoint(trusted_checkpoint, 10);
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
                // lite_client.validate_transaction(outcome_proof, )
                dbg!(bs58::encode(&tx_info[0].0.try_to_vec().unwrap()).into_string());
                dbg!(&tx_info[0].1);
                let (tx_hash, sender_id) = tx_info[0].clone();
                let tx_light_client_proof = blockchain_connector
                    .get_light_client_proof_transaction(almost_last_block_hash, tx_hash, sender_id)
                    .unwrap();

                let expected_block_outcome_root = CryptoHash::try_from_slice(
                    tx_light_client_proof
                        .block_header_lite
                        .inner_lite
                        .outcome_root
                        .try_to_vec()
                        .unwrap()
                        .as_ref(),
                )
                .unwrap();

                let outcome_root_proof = MerklePath::try_from_slice(
                    tx_light_client_proof
                        .outcome_root_proof
                        .try_to_vec()
                        .unwrap()
                        .as_ref(),
                )
                .unwrap();

                let outcome_proof = OutcomeProof::try_from_slice(
                    tx_light_client_proof
                        .outcome_proof
                        .try_to_vec()
                        .unwrap()
                        .as_ref(),
                )
                .unwrap();

                lite_client
                    .validate_transaction(
                        &outcome_proof,
                        outcome_root_proof,
                        expected_block_outcome_root,
                    )
                    .unwrap();

                break 'l;
            } else {
                dbg!("NADA");
            }
        }
        height -= 1;
    }
}
