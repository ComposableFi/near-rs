use alloc::string::String;
use near_primitives_wasm::HostFunctions;
use sp_std::{collections::btree_map::BTreeMap, vec::Vec};

use crate::{error::NearLiteClientError, LiteClientResult};

use near_primitives_wasm::{
	ApprovalInner, CryptoHash, LightClientBlockView, ValidatorStakeView,
};

use borsh::BorshSerialize;

pub fn validate_light_block<H: HostFunctions>(
	head: &LightClientBlockView,
	block_view: &LightClientBlockView,
	epoch_block_producers_map: &BTreeMap<CryptoHash, Vec<ValidatorStakeView>>,
) -> LiteClientResult<()> {
	//The light client updates its head with the information from LightClientBlockView iff:

	// 1. The height of the block is higher than the height of the current head;
	// 2. The epoch of the block is equal to the epoch_id or next_epoch_id known for the current
	// head; 3. If the epoch of the block is equal to the next_epoch_id of the head, then next_bps
	// is not None; 4. approvals_after_next contain valid signatures on approval_message from the
	// block producers of the corresponding epoch
	// 5. The signatures present in approvals_after_next correspond to more than 2/3 of the total
	// stake (see next section). 6. If next_bps is not none, sha256(borsh(next_bps)) corresponds to
	// the next_bp_hash in inner_lite.

	// QUESTION: do we also want to pass the block hash received from the RPC?
	// it's not on the spec, but it's an extra validation
	let (_current_block_hash, _next_block_hash, approval_message) =
		reconstruct_light_client_block_view_fields::<H>(block_view)?;

	// (1)
	if block_view.inner_lite.height <= head.inner_lite.height {
		return Err(NearLiteClientError::InvalidLiteBlock(String::from(
			"block view height is not ahead of the head's height",
		)));
	}

	// (2)
	if ![head.inner_lite.epoch_id, head.inner_lite.next_epoch_id]
		.contains(&block_view.inner_lite.epoch_id)
	{
		return Err(NearLiteClientError::InvalidLiteBlock(String::from(
			"block view epoch id not present in the head",
		)));
	}

	// (3)
	if block_view.inner_lite.epoch_id == head.inner_lite.next_epoch_id
		&& block_view.next_bps.is_none()
	{
		return Err(NearLiteClientError::InvalidLiteBlock(String::from(
			"block view epoch id is not the next epoch",
		)));
	}

	//  (4) and (5)
	let mut total_stake = 0;
	let mut approved_stake = 0;

	let epoch_block_producers = &epoch_block_producers_map[&block_view.inner_lite.epoch_id];

	for (maybe_signature, block_producer) in
		block_view.approvals_after_next.iter().zip(epoch_block_producers.iter())
	{
		let bp_stake_view = block_producer.clone().into_validator_stake();
		let bp_stake = bp_stake_view.stake;
		total_stake += bp_stake;

		if maybe_signature.is_none() {
			continue;
		}

		approved_stake += bp_stake;

		let validator_public_key = bp_stake_view.public_key.clone();
		if !maybe_signature
			.as_ref()
			.unwrap()
			.verify(&approval_message, validator_public_key.clone())
		{
			return Err(NearLiteClientError::SignatureVerification(String::from(
				"signature is not valid",
			)));
		}
	}

	let threshold = total_stake * 2 / 3;
	if approved_stake <= threshold {
		return Err(NearLiteClientError::InvalidLiteBlock(String::from(
			"block is not final: stake threshold is not reached",
		)));
	}

	// # (6)
	if block_view.next_bps.is_some() {
		let block_view_next_bps_serialized =
			block_view.next_bps.as_deref().unwrap().try_to_vec()?;
		if H::sha256(&block_view_next_bps_serialized).as_slice()
			!= block_view.inner_lite.next_bp_hash.as_ref()
		{
			return Err(NearLiteClientError::InvalidLiteBlock(String::from(
				"inccorect next bp hash in block view",
			)));
		}
	}
	Ok(())
}

pub fn reconstruct_light_client_block_view_fields<H: HostFunctions>(
	block_view: &LightClientBlockView,
) -> LiteClientResult<(CryptoHash, CryptoHash, Vec<u8>)> {
	let current_block_hash = block_view.current_block_hash::<H>();
	let next_block_hash =
		next_block_hash::<H>(block_view.next_block_inner_hash, current_block_hash);
	let approval_message = [
		ApprovalInner::Endorsement(next_block_hash).try_to_vec()?,
		(block_view.inner_lite.height + 2).to_le().try_to_vec()?,
	]
	.concat();
	Ok((current_block_hash, next_block_hash, approval_message))
}

pub(crate) fn next_block_hash<H: HostFunctions>(
	next_block_inner_hash: CryptoHash,
	current_block_hash: CryptoHash,
) -> CryptoHash {
	H::sha256(&[next_block_inner_hash.as_ref(), current_block_hash.as_ref()].concat())
		.as_slice()
		.try_into()
		.expect("Could not hash the next block")
}
