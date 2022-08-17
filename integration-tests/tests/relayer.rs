use std::{thread::park_timeout, time::Duration};

use near_lite_relayer::{
	blockchain_connector::{BlockchainConnector, NearNetwork},
	state::LightClientState,
};

#[tokio::test]
async fn relayer_fetches_data_and_does_simple_validation() {
	env_logger::init();

	let blockchain_connector = BlockchainConnector::new(NearNetwork::Testnet);

	let (_, almost_latest_height) = blockchain_connector
		.get_almost_latest_finalized_block_hash_and_height()
		.unwrap();

	// asumming almost one block per second, we know that an epoch is ~12 hours (43000 blocks)
	// since we want to start with a checkpoint from a previous epoch, we therefore request for the
	// hash way more than 43000 blocks before to be sure that we're no a past epoch
	let block_hash_past_epoch = blockchain_connector
		.get_block_hash_from_block_number(almost_latest_height - 100_000)
		.map_err(|e| {
			log::error!("{:?}", e);
			e
		})
		.unwrap();

	let light_client_block_view =
		blockchain_connector.get_light_client_block_view(block_hash_past_epoch).unwrap();

	// TODO: assert that we're on a past epoch
	let mut lite_client = LightClientState::new_from_checkpoint(light_client_block_view);

	let (tx, mut rx) = tokio::sync::mpsc::channel(1);

	tokio::task::spawn(async move {
		tokio::task::spawn_blocking(move || {
			let sleep_time = Duration::from_secs(3);
			let mut counter = 0;
			loop {
				let current_block_height = lite_client.current_block_hash();
				park_timeout(sleep_time);

				let current_block_hash = blockchain_connector
					.get_block_hash_from_block_number(current_block_height)
					.unwrap();

				let light_client_block_view =
					blockchain_connector.get_light_client_block_view(current_block_hash).unwrap();

				assert!(lite_client.validate_and_update_head(&light_client_block_view.into()));
				counter += 1;
				if counter == 3 {
					break
				}
			}
		})
		.await
		.unwrap();
		tx.send(()).await.unwrap();
	});

	// we do three loops, each of 3 seconds (total of 9 secs)
	// therefore the receiver should have a response before out threshold, which in this case is
	// _epsilon_ higher than those 9 secs
	let waiting_time = 9 + 3;
	tokio::time::timeout(tokio::time::Duration::from_secs(waiting_time), rx.recv())
		.await
		.unwrap();
}
