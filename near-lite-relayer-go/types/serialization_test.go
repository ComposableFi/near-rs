package types

import (
	"crypto/sha256"
	"github.com/stretchr/testify/require"
	"testing"

	"github.com/btcsuite/btcutil/base58"
	"github.com/near/borsh-go"
	"github.com/stretchr/testify/assert"
)

func TestPublicKeySerialization(t *testing.T) {
	pubKeyB58 := `ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su`
	publicKey := base58.Decode("ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su")
	var publicKeyFixedArray [32]byte
	copy(publicKeyFixedArray[:], publicKey)
	pk := PublicKey{
		Enum: 0,
		ED25519: ED25519PublicKey{
			Inner: publicKeyFixedArray,
		},
	}
	pkSerialized, err := borsh.Serialize(pk)
	require.Nil(t, err)

	var pkDeserialized PublicKey
	err = borsh.Deserialize(&pkDeserialized, pkSerialized)
	require.Nil(t, err)

	assert.Equal(t, pubKeyB58, base58.Encode(pkDeserialized.ED25519.Inner[:]))
	assert.Equal(t, pkDeserialized.Enum, borsh.Enum(0))
}

func TestSha256(t *testing.T) {
	s := sha256.Sum256([]byte{1, 2, 3})
	ss, err := borsh.Serialize(s)
	require.Nil(t, err)

	assert.Equal(t, s[:], ss)
}
