use crate::{
    block_validation::{validate_light_block, Digest},
    storage::StateStorage,
    types::{
        ApprovalInner, CryptoHash, ExecutionOutcomeView, LightClientBlockView, MerklePath,
        OutcomeProof,
    },
};

use borsh::BorshSerialize;
pub trait StateTransitionVerificator: StateStorage {
    type D: Digest;

    fn validate_and_update_head(&mut self, block_view: &LightClientBlockView) -> bool {
        let head = self.get_head();
        let epoch_block_producers = self.get_epoch_block_producers();
        if !validate_light_block::<Self::D>(head, block_view, epoch_block_producers) {}
        self.get_epoch_block_producers_mut().insert(
            block_view.inner_lite.next_epoch_id,
            block_view.next_bps.as_ref().unwrap().clone(),
        );

        let head = self.get_head_mut();
        *head = block_view.clone();
        true
    }

    fn validate_transaction<D: Digest>(
        &self,
        outcome: &ExecutionOutcomeView,
        outcome_proof: &OutcomeProof,
        outcome_root_proof: MerklePath,
        tx_hash: CryptoHash,
    ) -> bool {
        let execution_outcome_hash =
            calculate_execution_outcome_hash(&outcome_proof.outcome, outcome_proof.id);
        let shard_outcome_root =
            compute_root_from_path(&outcome_proof.proof, execution_outcome_hash);

        let block_outcome_root = compute_root_from_path(
            &outcome_root_proof,
            D::digest(shard_outcome_root.try_to_vec().unwrap())
                .as_slice()
                .try_into()
                .unwrap(),
        );

        let expected_block_outcome_root = todo!();

        expected_block_outcome_root == block_outcome_root.as_ref()
    }
}

fn reconstruct_light_client_block_view_fields<D: Digest>(
    block_view: &LightClientBlockView,
) -> (CryptoHash, CryptoHash, Vec<u8>) {
    let current_block_hash = block_view.current_block_hash::<D>();
    let next_block_hash =
        next_block_hash::<D>(block_view.next_block_inner_hash, current_block_hash);
    let approval_message = [
        ApprovalInner::Endorsement(next_block_hash)
            .try_to_vec()
            .unwrap()
            .as_ref(),
        (block_view.inner_lite.height + 2).to_le_bytes().as_ref(), // TODO: double check this one
    ]
    .concat();
    (current_block_hash, next_block_hash, approval_message)
}

pub(crate) fn next_block_hash<D: Digest>(
    next_block_inner_hash: CryptoHash,
    current_block_hash: CryptoHash,
) -> CryptoHash {
    D::digest([next_block_inner_hash.as_ref(), current_block_hash.as_ref()].concat())
        .as_slice()
        .try_into()
        .unwrap()
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
        execution_outcome.status.try_to_vec().unwrap(),
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
