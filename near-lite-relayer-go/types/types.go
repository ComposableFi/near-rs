package types

import (
	"math/big"

	"github.com/near/borsh-go"
)

type CryptoHash = [32]uint8
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
	account_id AccountId
	public_key PublicKey
	Stake      Balance
}

type ValidatorStakeViewV2 struct {
	AccountId   AccountId
	PublicKey   PublicKey
	Stake       Balance
	IsChunkOnly bool
}

// crypto primitives
type Signature struct {
	Enum    borsh.Enum `borsh_enum:"true"`
	ED25519 [64]byte
	// SECP256K1 []byte // TODO: be more specific on the number of bytes
}

type ED25519PublicKey = [32]byte

type PublicKey struct {
	Enum    borsh.Enum `borsh_enum:"true"`
	ED25519 ED25519PublicKey
	/// 512 bit elliptic curve based public-key used in Bitcoin's public-key cryptography.
	// SECP256K1 Secp256K1PublicKey
}

// rpc structs
type ValidatorStakeView struct {
	Enum borsh.Enum `borsh_enum:"true"`
	V1   ValidatorStakeViewV1
	V2   ValidatorStakeViewV2
}

type BlockHeaderInnerLiteView struct {
	Height        BlockHeight
	EpochId       CryptoHash
	NextEpochId   CryptoHash
	PrevStateRoot CryptoHash
	OutcomeRoot   CryptoHash
	/// Legacy json number. Should not be used.
	Timestamp        uint64
	TimestampNanosec uint64
	NextBpHash       CryptoHash
	BlockMerkleRoot  CryptoHash
}

type LightClientBlockView struct {
	PrevBlockHash        CryptoHash
	NextBlockInnerHash   CryptoHash
	InneLite             BlockHeaderInnerLiteView
	InnerRestHash        CryptoHash
	NextBps              *[]ValidatorStakeView
	approvals_after_next []*Signature
}

type LightClientBlockLiteView struct {
	PrevBlockHash CryptoHash
	InnerRestHash CryptoHash
	InneLite      BlockHeaderInnerLiteView
}

// RpcLightClientExecutionProofResponse

const (
	Left Direction = iota
	Right
)
