package types

import (
	"encoding/json"
	"math/big"

	"github.com/near/borsh-go"
)

type CryptoHash = [32]uint8
type CryptoHashBase58Encoded = string
type Gas = uint64
type Balance = big.Int // borsh maps u128 -> big.Int
type BlockHeight = uint64
type AccountId = string

// for merkle tree
type Direction = borsh.Enum
type MerkleHash = CryptoHash
type MerklePathItem struct {
	Hash      MerkleHash
	Direction Direction
}
type MerklePath = []MerklePathItem

type ValidatorStakeViewV1 struct {
	AccountId AccountId
	PublicKey PublicKey
	Stake     Balance
}

type ValidatorStakeViewV2 struct {
	AccountId   AccountId
	PublicKey   PublicKey
	Stake       Balance
	IsChunkOnly bool
}

// crypto primitives
type Signature struct {
	Enum    uint8
	ED25519 [64]byte
	// SECP256K1 []byte // TODO: be more specific on the number of bytes
}

type ED25519PublicKey struct {
	Inner CryptoHash
}

type PublicKey struct {
	Enum    borsh.Enum `borsh_enum:"true"`
	ED25519 ED25519PublicKey
	/// 512 bit elliptic curve based public-key used in Bitcoin's public-key cryptography.
	// SECP256K1 Secp256K1PublicKey
}

type ApprovalInner struct {
	Enum        borsh.Enum `borsh_enum:"true"`
	Endorsement Endorsement
}

type Endorsement struct {
	Inner CryptoHash
}

// rpc structs
type ValidatorStakeView struct {
	Enum borsh.Enum `borsh_enum:"true"`
	V1   ValidatorStakeViewV1
	V2   ValidatorStakeViewV2
}

type BlockHeaderInnerLiteViewJson struct {
	Height        BlockHeight             `json:"height"`
	EpochId       CryptoHashBase58Encoded `json:"epoch_id"`
	NextEpochId   CryptoHashBase58Encoded `json:"next_epoch_id"`
	PrevStateRoot CryptoHashBase58Encoded `json:"prev_state_root"`
	OutcomeRoot   CryptoHashBase58Encoded `json:"outcome_root"`
	/// Legacy json number. Should not be used.
	Timestamp        uint64                  `json:"timestamp"`
	TimestampNanosec string                  `json:"timestamp_nanosec"`
	NextBpHash       CryptoHashBase58Encoded `json:"next_bp_hash"`
	BlockMerkleRoot  CryptoHashBase58Encoded `json:"block_merkle_root"`
}

type BlockHeaderInnerLiteView struct {
	Height        BlockHeight
	EpochId       CryptoHash
	NextEpochId   CryptoHash
	PrevStateRoot CryptoHash
	OutcomeRoot   CryptoHash
	/// Legacy json number. Should not be used.
	Timestamp       uint64
	NextBpHash      CryptoHash
	BlockMerkleRoot CryptoHash
}

type LightClientBlockViewJson struct {
	PrevBlockHash      CryptoHashBase58Encoded      `json:"prev_block_hash"`
	NextBlockInnerHash CryptoHashBase58Encoded      `json:"next_block_inner_hash"`
	InnerLite          BlockHeaderInnerLiteViewJson `json:"inner_lite"`
	InnerRestHash      CryptoHashBase58Encoded      `json:"inner_rest_hash"`
	NextBps            []json.RawMessage            `json:"next_bps"`
	ApprovalsAfterNext []*json.RawMessage           `json:"approvals_after_next"`
}

type LightClientBlockView struct {
	PrevBlockHash      CryptoHash
	NextBlockInnerHash CryptoHash
	InnerLite          BlockHeaderInnerLiteView
	InnerRestHash      CryptoHash
	NextBps            []ValidatorStakeView
	ApprovalsAfterNext []*Signature
}

type LightClientBlockLiteView struct {
	PrevBlockHash CryptoHash
	InnerRestHash CryptoHash
	InnerLite     BlockHeaderInnerLiteView
}

// RpcLightClientExecutionProofResponse

const (
	Left Direction = iota
	Right
)

// rpc types
type Base58CryptoHash = string

func NewValidatorStakeViewFromV1(v1 ValidatorStakeViewV1) ValidatorStakeView {
	return ValidatorStakeView{
		Enum: 0,
		V1:   v1,
	}
}

func NewValidatorStakeViewFromV2(v2 ValidatorStakeViewV2) ValidatorStakeView {
	return ValidatorStakeView{
		Enum: 1,
		V2:   v2,
	}
}
