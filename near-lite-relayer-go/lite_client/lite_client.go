/// package lite client serves to encapsulate the lite client state
/// and to perform simple validations in case that it's desired.
/// Note that the real validations and checks will be performed in the
/// lite client and on-chain. This can be used just to catch certain
/// anomalies faster, and to avoid having to pay for a transaction.

package lite_client

import (
	"crypto/sha256"
	"encoding/binary"
	"log"

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
	epochBlockProducers[checkpoint.InneLite.NextEpochId] = checkpoint.NextBps
	return &LiteClient{
		head:                checkpoint,
		epochBlockProducers: map[types.CryptoHash][]types.ValidatorStakeView{},
	}
}

func (l *LiteClient) ValidateAndUpdateHead(blockView types.LightClientBlockView) bool {
	log.Printf("Validating block view for height=%d on epoch=%s",
		blockView.InneLite.Height, base58.Encode(blockView.InneLite.EpochId[:]),
	)

	return true
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
		Endorsement: *nextBlockHash,
	}
	approvalInnerSerialized, err := borsh.Serialize(approvalInner)
	if err != nil {
		return nil, nil, nil, err
	}
	b := make([]byte, 8)
	binary.LittleEndian.PutUint64(b, uint64(blockView.InneLite.Height+2))
	var approvalMessage []byte
	approvalMessage = append(approvalInnerSerialized, b...)

	return currentBlockHash, &blockView.NextBlockInnerHash, approvalMessage, nil

}

func currentBlockHash(blockView *types.LightClientBlockView) (*types.CryptoHash, error) {

	innerLiteSerialized, err := borsh.Serialize(blockView.InneLite)
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
