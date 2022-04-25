use std::{
    thread::{self},
    time::Duration,
};

use near_lite_relayer::{
    blockchain_connector::{BlockchainConnector, NearNetwork},
    state::LightClientState,
};

use borsh::{BorshDeserialize, BorshSerialize};
use near_lite_client::{
    LightClient, LightClientBlockView, StateTransitionVerificator, TrustedCheckpoint,
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
async fn both_relayer_and_lite_client() {
    env_logger::init();

    let blockchain_connector = BlockchainConnector::new(NearNetwork::Testnet);

    let (_, almost_latest_height) = blockchain_connector
        .get_almost_latest_finalized_block_hash_and_height()
        .unwrap();

    // asumming almost one block per second, we know that an epoch is ~12 hours (43000 blocks)
    // since we want to start with a checkpoint from a previous epoch, we therefore request for the hash
    // way more than 43000 blocks before to be sure that we're no a past epoch
    let block_hash_past_epoch = blockchain_connector
        .get_block_hash_from_block_number(almost_latest_height - 100_000)
        .map_err(|e| {
            log::error!("{:?}", e);
            e
        })
        .unwrap();

    let light_client_block_view = blockchain_connector
        .get_light_client_block_view(block_hash_past_epoch)
        .unwrap();

    let serialized_block_view = light_client_block_view.try_to_vec().unwrap();
    let block_view_for_lite_client =
        BorshDeserialize::try_from_slice(&serialized_block_view).unwrap();

    // TODO: assert that we're on a past epoch
    let _lite_client_relayer = LightClientState::new_from_checkpoint(light_client_block_view);

    let trusted_checkpoint = TrustedCheckpoint(block_view_for_lite_client);
    let mut lite_client = LightClient::new_from_checkpoint(trusted_checkpoint, 10);

    let (tx, mut rx) = tokio::sync::mpsc::channel(1);

    tokio::task::spawn(async move {
        let result = tokio::task::spawn_blocking(move || {
            let sleep_time = Duration::from_secs(3);
            let mut counter = 0;
            loop {
                let current_block_height = lite_client.current_block_height();
                log::info!("current block height={}", current_block_height);
                thread::sleep(sleep_time);

                let current_block_hash = blockchain_connector
                    .get_block_hash_from_block_number(current_block_height)
                    .unwrap();

                let near_light_client_block_view = blockchain_connector
                    .get_light_client_block_view(current_block_hash)
                    .unwrap();

                log::info!(
                    "fetched block to validate with height={:?}",
                    near_light_client_block_view.inner_lite.height
                );

                let light_client_block_view = LightClientBlockView::try_from_slice(
                    near_light_client_block_view.try_to_vec().unwrap().as_ref(),
                )
                .unwrap();
                if light_client_block_view.inner_lite.height <= lite_client.current_block_height() {
                    log::info!("block has not yet been updated");
                    continue;
                }

                log::info!(
                    "validating block height={}",
                    light_client_block_view.inner_lite.height
                );

                assert!(lite_client
                    .validate_and_update_head(&light_client_block_view)
                    .unwrap());

                log::info!(
                    "validated block height={} and head is on height={}",
                    light_client_block_view.inner_lite.height,
                    lite_client.current_block_height()
                );
                counter += 1;
                if counter == 3 {
                    break;
                }
            }
        })
        .await;
        if let Ok(_) = result {
            tx.send(()).await.unwrap();
        }
    });

    // we do three loops, each of 3 seconds (total of 9 secs)
    // therefore the receiver should have a response before out threshold, which in this case is _epsilon_ higher
    // than those 9 secs
    let waiting_time = 9 + 3;
    tokio::time::timeout(tokio::time::Duration::from_secs(waiting_time), rx.recv())
        .await
        .unwrap();
}
