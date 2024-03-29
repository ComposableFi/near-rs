use integration_tests::{LightClient, NearHostFunctions};
use near_lite_relayer::blockchain_connector::{BlockchainConnector, NearNetwork};

use borsh::{BorshDeserialize, BorshSerialize};
use near_lite_client::{
	validate_transaction, CryptoHash, MerklePath, NearLiteClientTrait, OutcomeProof,
	TrustedCheckpoint,
};

/// ## Both Relayer and Lite Client - testing tx validation
///
/// Finds one transaction from the last 300 blocks
/// Validates that the transaction is part of a valid block that the lite client has already
/// validated.
#[tokio::test]
async fn both_relayer_and_lite_client_validate_tx() {
	env_logger::init();

	let blockchain_connector = BlockchainConnector::new(NearNetwork::Testnet);

	let (almost_last_block_hash, almost_latest_height) = blockchain_connector
		.get_almost_latest_finalized_block_hash_and_height()
		.unwrap();

	let light_client_block_view = blockchain_connector
		.get_light_client_block_view(almost_last_block_hash)
		.unwrap();

	let serialized_block_view = light_client_block_view.try_to_vec().unwrap();
	let block_view_for_lite_client =
		BorshDeserialize::try_from_slice(&serialized_block_view).unwrap();

	let trusted_checkpoint = TrustedCheckpoint(block_view_for_lite_client);
	let _ = LightClient::new_from_checkpoint(trusted_checkpoint, 10);
	// find a transaction in a block that has been validated
	let mut height = almost_latest_height - 500;

	loop {
		let chunk_ids = blockchain_connector.find_chunk_ids_with_burned_gas(height).unwrap();

		for chunk_id in chunk_ids {
			let tx_info = blockchain_connector.get_transaction_ids_in_chunk(chunk_id).unwrap();
			if !tx_info.is_empty() {
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
					tx_light_client_proof.outcome_root_proof.try_to_vec().unwrap().as_ref(),
				)
				.unwrap();

				let outcome_proof = OutcomeProof::try_from_slice(
					tx_light_client_proof.outcome_proof.try_to_vec().unwrap().as_ref(),
				)
				.unwrap();

				validate_transaction::<NearHostFunctions>(
					&outcome_proof,
					outcome_root_proof,
					expected_block_outcome_root,
				)
				.unwrap();

				return;
			}
		}
		height -= 1;
		assert!(
			almost_latest_height < height + 500 + 300,
			"stopping the test - there should be at least one tx in 300 blocks"
		);
	}
}
