// Package types defines the types that are used throughout the program
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

// Direction ...
type Direction = borsh.Enum

// DirectionJSON is a string representation of Direction
type DirectionJSON = string

const (
	Left Direction = iota
	Right
)

// MerkleHash ...
type MerkleHash = CryptoHash

// MerklePathItemJson ...
type MerklePathItemJson struct {
	Hash      CryptoHashBase58Encoded `json:"hash"`
	Direction DirectionJSON           `json:"direction"`
}

// MerklePathItem ...
type MerklePathItem struct {
	Hash      MerkleHash
	Direction Direction
}

// MerklePath is an array of MerklePathItem
type MerklePath = []MerklePathItem

// MerklePathJson is a JSON representation of a MerklePath
type MerklePathJson = []MerklePathItemJson

// ValidatorStakeViewV1 ...
type ValidatorStakeViewV1 struct {
	AccountId AccountId
	PublicKey PublicKey
	Stake     Balance
}

// ValidatorStakeViewV2 ...
type ValidatorStakeViewV2 struct {
	AccountId   AccountId
	PublicKey   PublicKey
	Stake       Balance
	IsChunkOnly bool
}

// crypto primitives

// Signature (ED25519 at the moment only)
type Signature struct {
	Enum    uint8
	ED25519 [64]byte
	// SECP256K1 []byte // TODO: be more specific on the number of bytes
}

// ED25519PublicKey ...
type ED25519PublicKey struct {
	Inner CryptoHash
}

// PublicKey (only for ED25519 at the moment)
type PublicKey struct {
	Enum    borsh.Enum `borsh_enum:"true"`
	ED25519 ED25519PublicKey
	/// 512 bit elliptic curve based public-key used in Bitcoin's public-key cryptography.
	// SECP256K1 Secp256K1PublicKey
}

// ApprovalInner ...
type ApprovalInner struct {
	Enum        borsh.Enum `borsh_enum:"true"`
	Endorsement Endorsement
}

// Endorsement ...
type Endorsement struct {
	Inner CryptoHash
}

// ValidatorStakeView ...
type ValidatorStakeView struct {
	Enum borsh.Enum `borsh_enum:"true"`
	V1   ValidatorStakeViewV1
	V2   ValidatorStakeViewV2
}

// BlockHeaderInnerLiteViewJson ...
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

type LightClientBlockLiteViewJson struct {
	PrevBlockHash CryptoHashBase58Encoded      `json:"prev_block_hash"`
	InnerRestHash CryptoHashBase58Encoded      `json:"inner_rest_hash"`
	InnerLite     BlockHeaderInnerLiteViewJson `json:"inner_lite"`
}

type LightClientBlockLiteView struct {
	PrevBlockHash CryptoHash
	InnerRestHash CryptoHash
	InnerLite     BlockHeaderInnerLiteView
}

type Unknown struct{}
type SuccessValue struct {
	Inner string
}
type SuccessReceiptID struct {
	Inner CryptoHash
}

type ExecutionStatusView struct {
	Enum borsh.Enum `borsh_enum:"true"`
	/// The execution is pending or unknown.
	Unknown Unknown
	/// The execution has failed.
	Failure []byte
	/// The final action succeeded and returned some value or an empty vec encoded in base64.
	SuccessValue SuccessValue
	/// The final action of the receipt returned a promise or the signed transaction was converted
	/// to a receipt. Contains the receipt_id of the generated receipt.
	SuccessReceiptID SuccessReceiptID
}
type ExecutionOutcomeViewJSON struct {
	Logs        []string                   `json:"logs"`
	ReceiptIds  []Base58CryptoHash         `json:"receipt_ids"`
	GasBurnt    Gas                        `json:"gas_burnt"`
	TokensBurnt string                     `json:"tokens_burnt"`
	ExecutorID  AccountId                  `json:"executor_id"`
	Status      map[string]json.RawMessage `json:"status"`
}

type ExecutionOutcomeView struct {
	/// Logs from this transaction or receipt.
	Logs []string
	/// Receipt IDs generated by this transaction or receipt.
	ReceiptIds []CryptoHash
	/// The amount of the gas burnt by the given transaction or receipt.
	GasBurnt Gas
	/// The amount of tokens burnt corresponding to the burnt gas amount.
	/// This value doesn't always equal to the `gas_burnt` multiplied by the gas price, because
	/// the prepaid gas price might be lower than the actual gas price and it creates a deficit.
	TokensBurnt big.Int
	/// The id of the account on which the execution happens. For transaction this is signer_id,
	/// for receipt this is receiver_id.
	ExecutorID AccountId
	/// Execution status. Contains the result in case of successful execution.
	Status ExecutionStatusView // NOTE(blas): no need to deserialize this one (in order to avoid having to define too many unnecessary struct
}

type ExecutionOutcomeWithIDViewJSON struct {
	Proof     []MerklePathItemJson     `json:"proof"`
	BlockHash CryptoHashBase58Encoded  `json:"block_hash"`
	ID        CryptoHashBase58Encoded  `json:"id"`
	Outcome   ExecutionOutcomeViewJSON `json:"outcome"`
}

type ExecutionOutcomeWithIDView struct {
	/// Proof of the execution outcome
	Proof MerklePath
	/// Block hash of the block that contains the outcome root
	BlockHash CryptoHash
	/// Id of the execution (transaction or receipt)
	ID CryptoHash
	/// The actual outcome
	Outcome ExecutionOutcomeView
}

type RPCLightClientExecutionProofResponseJSON struct {
	OutcomeProof     ExecutionOutcomeWithIDViewJSON `json:"outcome_proof"`
	OutcomeRootProof MerklePathJson                 `json:"outcome_root_proof"`
	BlockHeaderLite  LightClientBlockLiteViewJson   `json:"block_header_lite"`
	BlockProof       MerklePathJson                 `json:"block_proof"`
}

type RPCLightClientExecutionProofResponse struct {
	/// Proof of execution outcome
	OutcomeProof ExecutionOutcomeWithIDView
	/// Proof of shard execution outcome root
	OutcomeRootProof MerklePath
	/// A light weight representation of block that contains the outcome root
	BlockHeaderLite LightClientBlockLiteView
	/// Proof of the existence of the block in the block merkle tree,
	/// which consists of blocks up to the light client head
	BlockProof MerklePath
}

// Base58CryptoHash represents CryptoHashes in base58
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
