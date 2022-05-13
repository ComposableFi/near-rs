/// package lite client serves to encapsulate the lite client state
/// and to perform simple validations in case that it's desired.
/// Note that the real validations and checks will be performed in the
/// lite client and on-chain. This can be used just to catch certain
/// anomalies faster, and to avoid having to pay for a transaction.

package lite_client

import (
	"crypto/ed25519"
	"crypto/sha256"
	"encoding/binary"
	"errors"
	"fmt"
	"log"
	"math/big"

	"github.com/ComposableFi/near-trustless-bridge/near-lite-relayer-go/types"
	"github.com/mr-tron/base58"
	"github.com/near/borsh-go"
)

type LiteClient struct {
	head                types.LightClientBlockView
	epochBlockProducers map[types.CryptoHash][]types.ValidatorStakeView
}

func NewLiteClientFromCheckpoint(checkpoint types.LightClientBlockView) *LiteClient {
	epochBlockProducers := make(map[types.CryptoHash][]types.ValidatorStakeView)
	epochBlockProducers[checkpoint.InnerLite.NextEpochId] = checkpoint.NextBps
	return &LiteClient{
		head:                checkpoint,
		epochBlockProducers: epochBlockProducers,
	}
}

func (l *LiteClient) ValidateAndUpdateHead(blockView *types.LightClientBlockView) (bool, error) {
	log.Printf("Validating block view for height=%d on epoch=%s",
		blockView.InnerLite.Height, base58.Encode(blockView.InnerLite.EpochId[:]),
	)
	_, _, approvalMessage, err := reconstrunctLightClientBlockViewFields(blockView)
	if err != nil {
		return false, err
	}
	head := l.head

	// (1)
	if blockView.InnerLite.Height <= head.InnerLite.Height {
		return false, nil
	}

	// (2)
	if !(blockView.InnerLite.EpochId == head.InnerLite.EpochId || blockView.InnerLite.EpochId == head.InnerLite.NextEpochId) {
		return false, nil
	}

	// (3)
	if blockView.InnerLite.EpochId == head.InnerLite.NextEpochId && blockView.NextBps == nil {
		return false, nil
	}

	//  (4) and (5)
	totalStake := big.Int{}
	approvedStake := big.Int{}

	epochBlockProducers, ok := l.epochBlockProducers[blockView.InnerLite.EpochId]
	if !ok {
		return false, errors.New(fmt.Sprintf("epochBlockProducer not found for epoch id %s", base58.Encode(blockView.InnerLite.EpochId[:])))
	}

	for i := range blockView.ApprovalsAfterNext {
		maybeSignature := blockView.ApprovalsAfterNext[i]
		blockProducer := epochBlockProducers[i]

		// TODO: handle v2 as well
		bpStake := blockProducer.V1.Stake
		totalStake.Add(&totalStake, &bpStake)
		if maybeSignature == nil {
			continue
		}

		approvedStake.Add(&approvedStake, &bpStake)

		publicKey := blockProducer.V1.PublicKey
		if !ed25519.Verify(publicKey.ED25519.Inner[:], approvalMessage, maybeSignature.ED25519[:]) {
			return false, nil
		}
	}

	t := totalStake.Mul(&totalStake, big.NewInt(2))
	threshold := t.Div(t, big.NewInt(3))
	if approvedStake.Cmp(threshold) == -1 {
		return false, nil
	}

	// # (6)
	if blockView.NextBps != nil {

		serializedNextBps, err := borsh.Serialize(blockView.NextBps)
		if err != nil {
			return false, err
		}
		if sha256.Sum256(serializedNextBps) != blockView.InnerLite.NextBpHash {
			return false, nil
		}
	}

	l.epochBlockProducers[blockView.InnerLite.NextEpochId] = blockView.NextBps
	l.head = *blockView

	return true, nil
}

func reconstrunctLightClientBlockViewFields(blockView *types.LightClientBlockView) (*types.CryptoHash, *types.CryptoHash, []byte, error) {
	currentBlockHash, err := currentBlockHash(blockView)
	if err != nil {
		return nil, nil, nil, err
	}
	nextBlockHash, err := nextBlockHash(blockView.NextBlockInnerHash, *currentBlockHash)
	if err != nil {
		return nil, nil, nil, err
	}
	approvalInner := types.ApprovalInner{
		Enum: 0,
		Endorsement: types.Endorsement{
			Inner: *nextBlockHash,
		},
	}
	approvalInnerSerialized, err := borsh.Serialize(approvalInner)
	if err != nil {
		return nil, nil, nil, err
	}
	b := make([]byte, 8)
	binary.LittleEndian.PutUint64(b, uint64(blockView.InnerLite.Height+2))
	var approvalMessage []byte
	approvalMessage = append(approvalInnerSerialized, b...)
	return currentBlockHash, &blockView.NextBlockInnerHash, approvalMessage, nil

}

func currentBlockHash(blockView *types.LightClientBlockView) (*types.CryptoHash, error) {

	innerLiteSerialized, err := borsh.Serialize(blockView.InnerLite)
	if err != nil {
		return nil, err
	}

	innertLiteHash := sha256.Sum256(innerLiteSerialized)
	// concatenate innerLiteHash with innerRestHash
	x := []byte{}
	x = append(innertLiteHash[:], blockView.InnerRestHash[:]...)
	hashX := sha256.Sum256(x)
	currentBlockHash := sha256.Sum256(append(hashX[:], blockView.PrevBlockHash[:]...))
	return &currentBlockHash, nil
}
func nextBlockHash(nextBlockInnerHash, currentBlockHash types.CryptoHash) (*types.CryptoHash, error) {
	concatHashes := []byte{}
	concatHashes = append(nextBlockInnerHash[:], currentBlockHash[:]...)
	serializedHash, err := borsh.Serialize(sha256.Sum256(concatHashes))
	if err != nil {
		return nil, err
	}
	var result [32]byte
	copy(result[:], serializedHash)
	return &result, nil
}

func calculateMerklelizationHashes(executionOutcome *types.ExecutionOutcomeView) ([]types.CryptoHash, error) {
	receiptIds, err := borsh.Serialize(executionOutcome.ReceiptIds)
	if err != nil {
		return nil, err
	}

	gasBurnt, err := borsh.Serialize(executionOutcome.GasBurnt)
	if err != nil {
		return nil, err
	}

	tokensBurnt, err := borsh.Serialize(executionOutcome.TokensBurnt)
	if err != nil {
		return nil, err
	}

	executorId, err := borsh.Serialize(executionOutcome.ExecutorID)
	if err != nil {
		return nil, err
	}

	status, err := borsh.Serialize(executionOutcome.Status)
	if err != nil {
		return nil, err
	}

	var logsPayload []byte
	logsPayload = append(receiptIds, gasBurnt...)
	logsPayload = append(logsPayload, tokensBurnt...)
	logsPayload = append(logsPayload, executorId...)
	logsPayload = append(logsPayload, status...)

	merklelizationHashes := []types.CryptoHash{sha256.Sum256(logsPayload)}
	for _, log := range executionOutcome.Logs {
		merklelizationHashes = append(merklelizationHashes, sha256.Sum256([]byte(log)))
	}

	return merklelizationHashes, nil
}

func calculateExecutionOutcomeHash(executionOutcome *types.ExecutionOutcomeView, txHash types.CryptoHash) (*types.CryptoHash, error) {
	merkelizationHashes, err := calculateMerklelizationHashes(executionOutcome)
	if err != nil {
		return nil, err
	}

	var packMerklelizationHashes []byte
	for _, mh := range merkelizationHashes {
		packMerklelizationHashes = append(packMerklelizationHashes, mh[:]...)
	}

	var inner []byte
	var b [4]byte
	binary.LittleEndian.PutUint32(b[:], uint32(len(merkelizationHashes)+1))
	inner = append(inner, b[:]...)
	inner = append(inner, txHash[:]...)
	inner = append(inner, packMerklelizationHashes...)

	result := sha256.Sum256(inner)
	return &result, nil
}

func computeRootFromPath(path types.MerklePath, itemHash types.MerkleHash) (types.MerkleHash, error) {
	res := itemHash
	for _, item := range path {
		switch item.Direction {
		case types.Left:
			r, err := combineHash(item.Hash, res)
			if err != nil {
				return types.MerkleHash{}, err
			}
			res = r
		case types.Right:
			r, err := combineHash(res, item.Hash)
			if err != nil {
				return types.MerkleHash{}, err
			}
			res = r

		}
	}
	return res, nil
}

func combineHash(hashOne, hashTwo types.MerkleHash) (types.CryptoHash, error) {

	hashes := [2]types.CryptoHash{hashOne, hashTwo}
	hashesSerialized, err := borsh.Serialize(hashes)
	if err != nil {
		return types.CryptoHash{}, err
	}
	return sha256.Sum256(hashesSerialized), nil

}

// pub fn compute_root_from_path(path: &MerklePath, item_hash: MerkleHash) -> MerkleHash {
//     let mut res = item_hash;
//     for item in path {
//         match item.direction {
//             Direction::Left => {
//                 res = combine_hash(&item.hash, &res);
//             }
//             Direction::Right => {
//                 res = combine_hash(&res, &item.hash);
//             }
//         }
//     }
//     res
// }
