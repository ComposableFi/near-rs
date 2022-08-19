use std::io;

use borsh::BorshSerialize;
use near_primitives::{
	hash::CryptoHash,
	merkle::MerklePath,
	views::{
		ExecutionOutcomeWithIdView, LightClientBlockLiteView,
		LightClientBlockView as NearLightClientBlockView,
	},
};
use near_sdk::json_types::Base58CryptoHash;
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

use crate::client_proof::ExecutionOutcomeViewForLiteClient;

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

#[derive(Debug, BorshSerialize, Deserialize)]
pub struct RpcLightClientExecutionProofResponse {
	/// Proof of execution outcome
	pub outcome_proof: ExecutionOutcomeWithIdView,
	/// Proof of shard execution outcome root
	pub outcome_root_proof: MerklePath,
	/// A light weight representation of block that contains the outcome root
	pub block_header_lite: LightClientBlockLiteView,
	/// Proof of the existence of the block in the block merkle tree,
	/// which consists of blocks up to the light client head
	pub block_proof: MerklePath,
}

pub struct RpcLightClientExecutionProofResponseForLiteClient {
	/// Proof of execution outcome
	pub outcome_proof: ExecutionOutcomeWithIdViewForLiteClient,
	/// Proof of shard execution outcome root
	pub outcome_root_proof: MerklePath,
	/// A light weight representation of block that contains the outcome root
	pub block_header_lite: LightClientBlockLiteView,
	/// Proof of the existence of the block in the block merkle tree,
	/// which consists of blocks up to the light client head
	pub block_proof: MerklePath,
}

#[derive(Debug, BorshSerialize, Deserialize)]
pub struct ExecutionOutcomeWithIdViewForLiteClient {
	pub proof: MerklePath,
	pub block_hash: CryptoHash,
	pub id: CryptoHash,
	pub outcome: ExecutionOutcomeViewForLiteClient,
}

impl From<ExecutionOutcomeWithIdView> for ExecutionOutcomeWithIdViewForLiteClient {
	fn from(view: ExecutionOutcomeWithIdView) -> Self {
		Self {
			proof: view.proof,
			block_hash: view.block_hash,
			id: view.id,
			outcome: view.outcome.into(),
		}
	}
}
impl From<RpcLightClientExecutionProofResponse>
	for RpcLightClientExecutionProofResponseForLiteClient
{
	fn from(response: RpcLightClientExecutionProofResponse) -> Self {
		Self {
			outcome_proof: response.outcome_proof.into(),
			outcome_root_proof: response.outcome_root_proof,
			block_header_lite: response.block_header_lite,
			block_proof: response.block_proof,
		}
	}
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
		struct Response {
			pub result: Result,
		}

		#[derive(Debug, Deserialize)]
		struct Result {
			pub header: Header,
		}

		#[derive(Debug, Deserialize)]
		struct Header {
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
		let r = body.into_json::<Response>()?;
		Ok((r.result.header.prev_hash, r.result.header.height))
	}

	/// gets almost the latest finalized block that's available on the NearNetwork
	/// helpful for testing purposes where we just want to get a hash, and based on it
	/// retrieve the block view for the next block
	pub fn get_block_hash_from_block_number(&self, height: u64) -> io::Result<Base58CryptoHash> {
		#[derive(Debug, Deserialize)]
		struct Response {
			pub result: Result,
		}

		#[derive(Debug, Deserialize)]
		struct Result {
			pub header: Header,
		}

		#[derive(Debug, Deserialize)]
		struct Header {
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
		Ok(body.into_json::<Response>()?.result.header.prev_hash)
	}

	pub fn find_chunk_ids_with_burned_gas(
		&self,
		block_height: u64,
	) -> io::Result<Vec<Base58CryptoHash>> {
		#[derive(Debug, Deserialize)]
		struct Response {
			pub result: Result,
		}

		#[derive(Debug, Deserialize)]
		struct Result {
			pub chunks: Vec<Chunks>,
		}

		#[derive(Debug, Deserialize)]
		struct Chunks {
			#[serde(deserialize_with = "deserialize_number_from_string")]
			pub balance_burnt: u128,
			pub chunk_hash: Base58CryptoHash,
		}

		let url = format!("{}/", self.network.get_base_url());
		let params = ureq::json!({ "block_id": block_height });
		let body = ureq::post(&url)
			.send_json(ureq::json!({
				"jsonrpc": "2.0",
				"method": "block",
				"params": params,
				"id": "dontcare",
			}))
			.map_err(|_| io::Error::from(io::ErrorKind::Unsupported))?; // TODO: improve error message
		Ok(body
			.into_json::<Response>()?
			.result
			.chunks
			.into_iter()
			.filter_map(|chunk| if chunk.balance_burnt > 0 { Some(chunk.chunk_hash) } else { None })
			.collect())
	}

	pub fn get_transaction_ids_in_chunk(
		&self,
		chunk_id: Base58CryptoHash,
	) -> io::Result<Vec<(Base58CryptoHash, String)>> {
		#[derive(Debug, Deserialize)]
		struct Response {
			pub result: Result,
		}

		#[derive(Debug, Deserialize)]
		struct Result {
			pub transactions: Vec<Tx>,
		}

		#[derive(Debug, Deserialize)]
		struct Tx {
			pub hash: Base58CryptoHash,
			pub signer_id: String,
		}

		let url = format!("{}/", self.network.get_base_url());
		let params = ureq::json!({ "chunk_id": chunk_id });
		let body = ureq::post(&url)
			.send_json(ureq::json!({
				"jsonrpc": "2.0",
				"method": "chunk",
				"params": params,
				"id": "dontcare",
			}))
			.map_err(|_| io::Error::from(io::ErrorKind::Unsupported))?; // TODO: improve error message
		Ok(body
			.into_json::<Response>()?
			.result
			.transactions
			.into_iter()
			.flat_map(|tx| Some((tx.hash, tx.signer_id)))
			.collect())
	}

	pub fn get_light_client_proof_transaction(
		&self,
		light_client_head: Base58CryptoHash,
		tx_hash: Base58CryptoHash,
		sender_id: String,
	) -> io::Result<RpcLightClientExecutionProofResponseForLiteClient> {
		#[derive(Debug, Deserialize)]
		struct Response {
			pub result: RpcLightClientExecutionProofResponse,
		}

		let url = format!("{}/", self.network.get_base_url());
		let params = ureq::json!({ "type": "transaction" , "transaction_hash": tx_hash, "sender_id": sender_id, "light_client_head": light_client_head});
		let body = ureq::post(&url)
			.send_json(ureq::json!({
				"jsonrpc": "2.0",
				"method": "EXPERIMENTAL_light_client_proof",
				"params": params,
				"id": "dontcare",
			}))
			.map_err(|_| io::Error::from(io::ErrorKind::Unsupported))?; // TODO: improve error message
		Ok(body.into_json::<Response>()?.result.into())
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

		blockchain_connector.get_block_hash_from_block_number(block_height).unwrap();
	}

	#[test]
	fn test_get_light_client_block_view() {
		let blockchain_connector = BlockchainConnector::new(NearNetwork::Testnet);
		let (block_hash, _) = blockchain_connector
			.get_almost_latest_finalized_block_hash_and_height()
			.unwrap();

		blockchain_connector.get_light_client_block_view(block_hash).unwrap();
	}
}
