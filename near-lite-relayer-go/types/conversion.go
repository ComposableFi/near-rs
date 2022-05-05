package types

import (
	"encoding/json"
	"errors"
	"math/big"
	"strings"

	"github.com/btcsuite/btcutil/base58"
)

func (j *LightClientBlockViewJson) IntoLightClientBlockView() *LightClientBlockView {
	var prevBlockHash [32]byte
	var nextBlockInnerHash [32]byte
	var innerRestHash [32]byte

	copy(prevBlockHash[:], base58.Decode(j.PrevBlockHash))
	copy(nextBlockInnerHash[:], base58.Decode(j.NextBlockInnerHash))
	copy(innerRestHash[:], base58.Decode(j.InnerRestHash))

	return &LightClientBlockView{
		PrevBlockHash:      prevBlockHash,
		NextBlockInnerHash: nextBlockInnerHash,
		InneLite:           j.InnerLite.IntoBlockHeaderInnerLiteView(),
		InnerRestHash:      innerRestHash,
		NextBps:            nil,
		ApprovalsAfterNext: []*json.RawMessage{},
	}
}

func (j *BlockHeaderInnerLiteViewJson) IntoBlockHeaderInnerLiteView() BlockHeaderInnerLiteView {

	var epochId, nextEpochId, prevStateRoot, outcomeRoot, nextBpHash, blockMerkleRoot [32]byte
	copy(epochId[:], base58.Decode(j.EpochId))
	copy(nextEpochId[:], base58.Decode(j.NextEpochId))
	copy(prevStateRoot[:], base58.Decode(j.PrevStateRoot))
	copy(outcomeRoot[:], base58.Decode(j.OutcomeRoot))
	copy(nextBpHash[:], base58.Decode(j.NextBpHash))
	copy(blockMerkleRoot[:], base58.Decode(j.BlockMerkleRoot))

	return BlockHeaderInnerLiteView{
		Height:           j.Height,
		EpochId:          epochId,
		NextEpochId:      nextEpochId,
		PrevStateRoot:    prevStateRoot,
		OutcomeRoot:      outcomeRoot,
		Timestamp:        j.Timestamp,
		TimestampNanosec: j.TimestampNanosec,
		NextBpHash:       nextBpHash,
		BlockMerkleRoot:  blockMerkleRoot,
	}
}

func IntoNextValidatorStakeViews(nextBps []json.RawMessage) ([]ValidatorStakeView, error) {

	type rawStruct struct {
		AccountId                   string `json:"account_id"`
		PublicKey                   string `json:"public_key"`
		Stake                       string `json:"stake"`
		ValidatorStakeStructVersion string `json:"validator_stake_struct_version"`
	}

	var result []ValidatorStakeView
	for _, nbp := range nextBps {
		var r rawStruct
		err := json.Unmarshal(nbp, &r)
		if err != nil {
			return nil, err
		}

		publicKey, err := unmarshallPublicKey(r.PublicKey)
		if err != nil {
			return nil, err
		}
		b := big.Int{}
		b.SetString(r.Stake, 10)
		switch r.ValidatorStakeStructVersion {
		case "V1":
			result = append(result, NewValidatorStakeViewFromV1(ValidatorStakeViewV1{
				AccountId: r.AccountId,
				PublicKey: *publicKey,
				Stake:     b,
			}))
		case "V2":
			result = append(result, NewValidatorStakeViewFromV2(ValidatorStakeViewV2{
				AccountId:   r.AccountId,
				PublicKey:   *publicKey,
				Stake:       b,
				IsChunkOnly: false, // TODO: validate this
			}))
		default:
			return nil, errors.New("validator stake view only implemented for v1 or v2")
		}
	}
	return result, nil
}

func unmarshallPublicKey(serializedData string) (*PublicKey, error) {

	knownPrefix := "ed25519:"
	// we might want to include other types
	if !strings.HasPrefix(serializedData, knownPrefix) {
		return nil, errors.New("unknown public key type")
	}
	splittedString := strings.Split(serializedData, ":")

	// decode the public key (which expressed in base58)
	data := base58.Decode(splittedString[1])
	if len(data) != 32 {
		return nil, errors.New("publick key size must be 32 bytes")
	}

	var publicKeyData [32]byte
	copy(publicKeyData[:], data)
	return &PublicKey{ED25519: publicKeyData}, nil
}
