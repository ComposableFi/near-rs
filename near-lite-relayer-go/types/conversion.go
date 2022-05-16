package types

import (
	"encoding/json"
	"errors"
	"log"
	"math/big"
	"strings"

	"github.com/btcsuite/btcutil/base58"
)

func intoCryptoHash(b Base58CryptoHash) CryptoHash {
	var result CryptoHash
	copy(result[:], base58.Decode(b))
	return result
}

func (j *LightClientBlockViewJSON) IntoLightClientBlockView() (*LightClientBlockView, error) {

	nextBps, err := intoNextValidatorStakeViews(j.NextBps)
	if err != nil {
		return nil, err
	}
	approvalsAfterNext, err := intoSignatures(j.ApprovalsAfterNext)
	if err != nil {
		return nil, err
	}

	return &LightClientBlockView{
		PrevBlockHash:      intoCryptoHash(j.PrevBlockHash),
		NextBlockInnerHash: intoCryptoHash(j.NextBlockInnerHash),
		InnerLite:          j.InnerLite.intoBlockHeaderInnerLiteView(),
		InnerRestHash:      intoCryptoHash(j.InnerRestHash),
		NextBps:            nextBps,
		ApprovalsAfterNext: approvalsAfterNext,
	}, err
}

func (j *BlockHeaderInnerLiteViewJSON) intoBlockHeaderInnerLiteView() BlockHeaderInnerLiteView {
	return BlockHeaderInnerLiteView{
		Height:          j.Height,
		EpochID:         intoCryptoHash(j.EpochID),
		NextEpochID:     intoCryptoHash(j.NextEpochID),
		PrevStateRoot:   intoCryptoHash(j.PrevStateRoot),
		OutcomeRoot:     intoCryptoHash(j.OutcomeRoot),
		Timestamp:       j.Timestamp,
		NextBpHash:      intoCryptoHash(j.NextBpHash),
		BlockMerkleRoot: intoCryptoHash(j.BlockMerkleRoot),
	}
}

func intoNextValidatorStakeViews(nextBps []json.RawMessage) ([]ValidatorStakeView, error) {

	type rawStruct struct {
		AccountID                   string `json:"account_id"`
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
		const decimalPower = 10
		b.SetString(r.Stake, decimalPower)
		switch r.ValidatorStakeStructVersion {
		case "V1":
			result = append(result, NewValidatorStakeViewFromV1(ValidatorStakeViewV1{
				AccountID: r.AccountID,
				PublicKey: *publicKey,
				Stake:     b,
			}))
		case "V2":
			result = append(result, NewValidatorStakeViewFromV2(ValidatorStakeViewV2{
				AccountID:   r.AccountID,
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

func intoSignatures(approvalsAfterNext []*json.RawMessage) ([]*Signature, error) {
	// "ed25519:4qnb1YmQngt9X3M88igWTWWPxX8GLwjYh6nHYYBGhZs5vFP5JxRNS8MqTNjn9eBebkd5mw72cM5emDKVfMY7hMrc"
	knownPrefix := "ed25519:"
	var result []*Signature
	for _, signatureEncoded := range approvalsAfterNext {
		if signatureEncoded == nil {
			result = append(result, nil)
			continue
		}
		var serializedData string
		err := json.Unmarshal([]byte(*signatureEncoded), &serializedData)
		if err != nil {
			return nil, err
		}
		if !strings.HasPrefix(serializedData, knownPrefix) {
			return nil, errors.New("unknown signature type")
		}

		splittedString := strings.Split(serializedData, ":")
		// decode the signature (which expressed in base58)
		data := base58.Decode(splittedString[1])
		const validLenghtData = 64
		if len(data) != validLenghtData {
			return nil, errors.New("signature size must be of 64 bytes")
		}

		const signatureSize = 64
		var signatureData [signatureSize]byte
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
	const ed25519Length = 32
	if len(data) != ed25519Length {
		return nil, errors.New("publick key size must be 32 bytes")
	}

	var publicKeyData [ed25519Length]byte
	copy(publicKeyData[:], data)
	return &PublicKey{
		Enum: 0,
		ED25519: ED25519PublicKey{
			Inner: publicKeyData,
		},
	}, nil
}
func (mpi *MerklePathItemJSON) intoMerklePathItem() *MerklePathItem {
	const validHashLength = 32
	var hash [validHashLength]byte
	copy(hash[:], base58.Decode(mpi.Hash))
	base58.Encode(hash[:])
	var direction Direction
	if strings.ToLower(mpi.Direction) == "left" {
		direction = Left
	} else {
		direction = Right
	}

	return &MerklePathItem{
		Direction: direction,
		Hash:      hash,
	}
}
func (op *ExecutionOutcomeWithIDViewJSON) intoExecutionOutcomeWithIDView() (*ExecutionOutcomeWithIDView, error) {
	var blockHash, id [32]byte
	copy(blockHash[:], base58.Decode(op.BlockHash))
	copy(id[:], base58.Decode(op.ID))

	proof := make([]MerklePathItem, len(op.Proof))
	for i := range op.Proof {
		proof[i] = *op.Proof[i].intoMerklePathItem()
	}

	outcome, err := op.Outcome.intoExecutionOutcomeView()
	if err != nil {
		return nil, err
	}

	return &ExecutionOutcomeWithIDView{
		Proof:     proof,
		BlockHash: blockHash,
		ID:        id,
		Outcome:   *outcome,
	}, nil
}

func (lcb *LightClientBlockLiteViewJSON) IntoLightClientBlockView() *LightClientBlockLiteView {
	var prevBlockHash, innerRestHash CryptoHash
	copy(prevBlockHash[:], base58.Decode(lcb.PrevBlockHash))
	copy(innerRestHash[:], base58.Decode(lcb.InnerRestHash))

	return &LightClientBlockLiteView{
		PrevBlockHash: prevBlockHash,
		InnerRestHash: innerRestHash,
		InnerLite:     lcb.InnerLite.intoBlockHeaderInnerLiteView(),
	}
}

func (ep *RPCLightClientExecutionProofResponseJSON) IntoRPCLightClientExecutionProofResponse() (*RPCLightClientExecutionProofResponse, error) {
	blockProof := make([]MerklePathItem, len(ep.BlockProof))
	for i := range ep.BlockProof {
		blockProof[i] = *ep.BlockProof[i].intoMerklePathItem()
	}
	outcomeRootProof := make([]MerklePathItem, len(ep.OutcomeRootProof))
	for i := range ep.OutcomeRootProof {
		outcomeRootProof[i] = *ep.OutcomeRootProof[i].intoMerklePathItem()
	}

	outcomeProof, err := ep.OutcomeProof.intoExecutionOutcomeWithIDView()
	if err != nil {
		return nil, err
	}

	return &RPCLightClientExecutionProofResponse{
		OutcomeProof:     *outcomeProof,
		OutcomeRootProof: outcomeRootProof,
		BlockHeaderLite:  *ep.BlockHeaderLite.IntoLightClientBlockView(),
		BlockProof:       blockProof,
	}, nil
}

func (eo *ExecutionOutcomeViewJSON) intoExecutionOutcomeView() (*ExecutionOutcomeView, error) {
	receiptIds := make([]CryptoHash, len(eo.ReceiptIds))
	for i, ri := range eo.ReceiptIds {
		receiptIds[i] = intoCryptoHash(ri)
	}
	var status ExecutionStatusView
	const unknown = 0
	const executionStatusView = 2
	const successReceiptID = 3
	for k, v := range eo.Status {
		switch k {
		case "Uknonwn":
			var s string
			err := json.Unmarshal([]byte(v), &s)
			if err != nil {
				return nil, err
			}
			status = ExecutionStatusView{
				Enum:    unknown,
				Unknown: Unknown{},
			}
		case "Failure":
			log.Println("Unsupported failure transaction")
			return nil, errors.New("unsupported failure transaction")
		case "SuccessValues":
			var s string
			err := json.Unmarshal([]byte(v), &s)
			if err != nil {
				return nil, err
			}
			status = ExecutionStatusView{
				Enum: executionStatusView,
				SuccessValue: SuccessValue{
					Inner: s,
				},
			}
		case "SuccessReceiptID":
			var s string
			err := json.Unmarshal([]byte(v), &s)
			if err != nil {
				return nil, err
			}
			cryptoHash := intoCryptoHash(s)
			status = ExecutionStatusView{
				Enum: successReceiptID,
				SuccessReceiptID: SuccessReceiptID{
					Inner: cryptoHash,
				},
			}
		}

	}
	var tokensBurnt big.Int

	const numberBase = 10
	tokensBurnt.SetString(eo.TokensBurnt, numberBase)

	return &ExecutionOutcomeView{
		Logs:        eo.Logs,
		ReceiptIds:  receiptIds,
		GasBurnt:    eo.GasBurnt,
		TokensBurnt: tokensBurnt,
		ExecutorID:  eo.ExecutorID,
		Status:      status,
	}, nil
}
