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

func (j *LightClientBlockViewJson) IntoLightClientBlockView() (*LightClientBlockView, error) {

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

func (j *BlockHeaderInnerLiteViewJson) intoBlockHeaderInnerLiteView() BlockHeaderInnerLiteView {
	return BlockHeaderInnerLiteView{
		Height:          j.Height,
		EpochId:         intoCryptoHash(j.EpochId),
		NextEpochId:     intoCryptoHash(j.NextEpochId),
		PrevStateRoot:   intoCryptoHash(j.PrevStateRoot),
		OutcomeRoot:     intoCryptoHash(j.OutcomeRoot),
		Timestamp:       j.Timestamp,
		NextBpHash:      intoCryptoHash(j.NextBpHash),
		BlockMerkleRoot: intoCryptoHash(j.BlockMerkleRoot),
	}
}

func intoNextValidatorStakeViews(nextBps []json.RawMessage) ([]ValidatorStakeView, error) {

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
func (mpi *MerklePathItemJson) intoMerklePathItem() *MerklePathItem {
	var hash [32]byte
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
func (op *ExecutionOutcomeWithIdViewJson) intoExecutionOutcomeWithIdView() (*ExecutionOutcomeWithIdView, error) {
	var blockHash, id [32]byte
	copy(blockHash[:], base58.Decode(op.BlockHash))
	copy(id[:], base58.Decode(op.Id))

	proof := make([]MerklePathItem, len(op.Proof))
	for i := range op.Proof {
		proof[i] = *op.Proof[i].intoMerklePathItem()
	}

	outcome, err := op.Outcome.intoExecutionOutcomeView()
	if err != nil {
		return nil, err
	}

	return &ExecutionOutcomeWithIdView{
		Proof:     proof,
		BlockHash: blockHash,
		Id:        id,
		Outcome:   *outcome,
	}, nil
}

func (lcb *LightClientBlockLiteViewJson) IntoLightClientBlockView() *LightClientBlockLiteView {
	var prevBlockHash, innerRestHash CryptoHash
	copy(prevBlockHash[:], base58.Decode(lcb.PrevBlockHash))
	copy(innerRestHash[:], base58.Decode(lcb.InnerRestHash))

	return &LightClientBlockLiteView{
		PrevBlockHash: prevBlockHash,
		InnerRestHash: innerRestHash,
		InnerLite:     lcb.InnerLite.intoBlockHeaderInnerLiteView(),
	}
}

func (ep *RpcLightClientExecutionProofResponseJson) IntoRpcLightClientExecutionProofResponse() (*RpcLightClientExecutionProofResponse, error) {
	blockProof := make([]MerklePathItem, len(ep.BlockProof))
	for i := range ep.BlockProof {
		blockProof[i] = *ep.BlockProof[i].intoMerklePathItem()
	}
	outcomeRootProof := make([]MerklePathItem, len(ep.OutcomeRootProof))
	for i := range ep.OutcomeRootProof {
		outcomeRootProof[i] = *ep.OutcomeRootProof[i].intoMerklePathItem()
	}

	outcomeProof, err := ep.OutcomeProof.intoExecutionOutcomeWithIdView()
	if err != nil {
		return nil, err
	}

	return &RpcLightClientExecutionProofResponse{
		OutcomeProof:     *outcomeProof,
		OutcomeRootProof: outcomeRootProof,
		BlockHeaderLite:  *ep.BlockHeaderLite.IntoLightClientBlockView(),
		BlockProof:       blockProof,
	}, nil
}

func (eo *ExecutionOutcomeViewJson) intoExecutionOutcomeView() (*ExecutionOutcomeView, error) {
	receiptIds := make([]CryptoHash, len(eo.ReceiptIds))
	for i, ri := range eo.ReceiptIds {
		receiptIds[i] = intoCryptoHash(ri)
	}
	var status ExecutionStatusView
	for k, v := range eo.Status {
		switch k {
		case "Uknonwn":
			var s string
			json.Unmarshal([]byte(v), &s)
			status = ExecutionStatusView{
				Enum:    0,
				Unknown: Unknown{},
			}
		case "Failure":
			log.Println("Unsupported failure transaction")
			return nil, errors.New("Unsupported failure transaction")
		case "SuccessValues":
			var s string
			json.Unmarshal([]byte(v), &s)
			status = ExecutionStatusView{
				Enum: 2,
				SuccessValue: SuccessValue{
					Inner: s,
				},
			}
		case "SuccessReceiptId":
			var s string
			json.Unmarshal([]byte(v), &s)
			cryptoHash := intoCryptoHash(s)
			status = ExecutionStatusView{
				Enum: 3,
				SuccessReceiptId: SuccessReceiptId{
					Inner: cryptoHash,
				},
			}
		}

	}
	var tokensBurnt big.Int

	tokensBurnt.SetString(eo.TokensBurnt, 10)

	return &ExecutionOutcomeView{
		Logs:        eo.Logs,
		ReceiptIds:  receiptIds,
		GasBurnt:    eo.GasBurnt,
		TokensBurnt: tokensBurnt,
		ExecutorId:  eo.ExecutorId,
		Status:      status,
	}, nil
}
