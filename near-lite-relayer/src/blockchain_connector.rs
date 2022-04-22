use std::io;

use borsh::BorshSerialize;
use near_primitives::views::LightClientBlockView as NearLightClientBlockView;
use near_sdk::json_types::Base58CryptoHash;
use serde::Deserialize;

pub enum NearNetwork {
    Mainnet,
    Testnet,
}

impl ToString for NearNetwork {
    fn to_string(&self) -> String {
        match self {
            Self::Mainnet => "mainnet".to_owned(),
            Self::Testnet => "testnet".to_owned(),
        }
    }
}

impl NearNetwork {
    fn get_base_url(&self) -> String {
        format!("https://rpc.{}.near.org", self.to_string())
    }
}

/// Connects to Near RPC and submits requests
pub struct BlockchainConnector {
    network: NearNetwork,
}

impl BlockchainConnector {
    pub fn new(network: NearNetwork) -> Self {
        Self { network }
    }

    /// gets the next client block view given a block hash that already has been validated
    pub fn get_light_client_block_view(
        &self,
        last_known_hash: Base58CryptoHash,
    ) -> io::Result<NearLightClientBlockView> {
        #[derive(Debug, Deserialize)]
        struct ResultFromRpc {
            result: NearLightClientBlockView,
        }

        // http post http://127.0.0.1:3030/ jsonrpc=2.0 method=next_light_client_block params:="[<last known hash>]" id="dontcare"
        let url = format!("{}/", self.network.get_base_url());
        let last_known_hash_string = String::from(&last_known_hash);
        let body = ureq::post(&url)
            .send_json(ureq::json!({
                "jsonrpc": "2.0",
                "method": "next_light_client_block",
                "params": [last_known_hash_string],
                "id": "dontcare",
            }))
            .map_err(|_| io::Error::from(io::ErrorKind::Unsupported))?; // TODO: improve error message

        Ok(serde_json::from_value::<ResultFromRpc>(body.into_json().unwrap())?.result)
    }

    /// gets almost the latest finalized block that's available on the NearNetwork
    /// helpful for testing purposes where we just want to get a hash, and based on it
    /// retrieve the block view for the next block
    pub fn get_almost_latest_finalized_block_hash_and_height(
        &self,
    ) -> io::Result<(Base58CryptoHash, u64)> {
        #[derive(Debug, Deserialize)]
        struct response {
            pub result: result,
        }

        #[derive(Debug, Deserialize)]
        struct result {
            pub header: header,
        }

        #[derive(Debug, Deserialize)]
        struct header {
            pub prev_hash: Base58CryptoHash,
            pub height: u64,
        }

        let url = format!("{}/", self.network.get_base_url());
        let params = ureq::json!({"finality": "final"});
        let body = ureq::post(&url)
            .send_json(ureq::json!({
                "jsonrpc": "2.0",
                "method": "block",
                "params": params,
                "id": "dontcare",
            }))
            .map_err(|_| io::Error::from(io::ErrorKind::Unsupported))?; // TODO: improve error message
        let r = body.into_json::<response>()?;
        Ok((r.result.header.prev_hash, r.result.header.height))
    }

    /// gets almost the latest finalized block that's available on the NearNetwork
    /// helpful for testing purposes where we just want to get a hash, and based on it
    /// retrieve the block view for the next block
    pub fn get_block_hash_from_block_number(&self, height: u64) -> io::Result<Base58CryptoHash> {
        #[derive(Debug, Deserialize)]
        struct response {
            pub result: result,
        }

        #[derive(Debug, Deserialize)]
        struct result {
            pub header: header,
        }

        #[derive(Debug, Deserialize)]
        struct header {
            pub prev_hash: Base58CryptoHash,
        }

        let url = format!("{}/", self.network.get_base_url());
        let params = ureq::json!({ "block_id": height });
        let body = ureq::post(&url)
            .send_json(ureq::json!({
                "jsonrpc": "2.0",
                "method": "block",
                "params": params,
                "id": "dontcare",
            }))
            .map_err(|_| io::Error::from(io::ErrorKind::Unsupported))?; // TODO: improve error message
        Ok(body.into_json::<response>()?.result.header.prev_hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_base_url() {
        for (network, expected_url) in [
            (NearNetwork::Testnet, "https://rpc.testnet.near.org"),
            (NearNetwork::Mainnet, "https://rpc.mainnet.near.org"),
        ] {
            assert_eq!(expected_url.to_owned(), network.get_base_url());
        }
    }

    #[test]
    fn test_get_latest_finalized_block() {
        let blockchain_connector = BlockchainConnector::new(NearNetwork::Testnet);
        blockchain_connector
            .get_almost_latest_finalized_block_hash_and_height()
            .unwrap();
    }

    #[test]
    fn test_get_block_by_number() {
        let blockchain_connector = BlockchainConnector::new(NearNetwork::Testnet);
        let (_, block_height) = blockchain_connector
            .get_almost_latest_finalized_block_hash_and_height()
            .unwrap();

        blockchain_connector
            .get_block_hash_from_block_number(block_height)
            .unwrap();
    }

    #[test]
    fn test_get_light_client_block_view() {
        let blockchain_connector = BlockchainConnector::new(NearNetwork::Testnet);
        let (block_hash, _) = blockchain_connector
            .get_almost_latest_finalized_block_hash_and_height()
            .unwrap();

        blockchain_connector
            .get_light_client_block_view(block_hash)
            .unwrap();
    }
}
