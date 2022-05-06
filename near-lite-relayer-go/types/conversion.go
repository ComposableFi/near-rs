package types

import (
	"encoding/json"
	"errors"
	"math/big"
	"strings"

	"github.com/btcsuite/btcutil/base58"
)

func (j *LightClientBlockViewJson) IntoLightClientBlockView() (*LightClientBlockView, error) {
	var prevBlockHash [32]byte
	var nextBlockInnerHash [32]byte
	var innerRestHash [32]byte

	copy(prevBlockHash[:], base58.Decode(j.PrevBlockHash))
	copy(nextBlockInnerHash[:], base58.Decode(j.NextBlockInnerHash))
	copy(innerRestHash[:], base58.Decode(j.InnerRestHash))

	nextBps, err := IntoNextValidatorStakeViews(j.NextBps)
	if err != nil {
		return nil, err
	}
	approvalsAfterNext, err := IntoSignatures(j.ApprovalsAfterNext)
	if err != nil {
		return nil, err
	}

	return &LightClientBlockView{
		PrevBlockHash:      prevBlockHash,
		NextBlockInnerHash: nextBlockInnerHash,
		InnerLite:          j.InnerLite.IntoBlockHeaderInnerLiteView(),
		InnerRestHash:      innerRestHash,
		NextBps:            nextBps,
		ApprovalsAfterNext: approvalsAfterNext,
	}, err
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
		Height:          j.Height,
		EpochId:         epochId,
		NextEpochId:     nextEpochId,
		PrevStateRoot:   prevStateRoot,
		OutcomeRoot:     outcomeRoot,
		Timestamp:       j.Timestamp,
		NextBpHash:      nextBpHash,
		BlockMerkleRoot: blockMerkleRoot,
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

func IntoSignatures(approvalsAfterNext []*json.RawMessage) ([]*Signature, error) {
	// "ed25519:4qnb1YmQngt9X3M88igWTWWPxX8GLwjYh6nHYYBGhZs5vFP5JxRNS8MqTNjn9eBebkd5mw72cM5emDKVfMY7hMrc"
	knownPrefix := "ed25519:"
	var result []*Signature
	for _, signatureEncoded := range approvalsAfterNext {
		if signatureEncoded == nil {
			result = append(result, nil)
			continue
		}
		var serializedData string
		json.Unmarshal([]byte(*signatureEncoded), &serializedData)
		if !strings.HasPrefix(serializedData, knownPrefix) {
			return nil, errors.New("unknown signature type")
		}

		splittedString := strings.Split(serializedData, ":")
		// decode the signature (which expressed in base58)
		data := base58.Decode(splittedString[1])
		if len(data) != 64 {
			return nil, errors.New("signature size must be of 64 bytes")
		}

		var signatureData [64]byte
		copy(signatureData[:], data)
		result = append(result, &Signature{
			Enum:    0,
			ED25519: signatureData,
		})
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
	return &PublicKey{
		Enum: 0,
		ED25519: ED25519PublicKey{
			Inner: publicKeyData,
		},
	}, nil
}
