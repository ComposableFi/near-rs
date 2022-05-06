package types

import (
	"testing"

	"log"

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
	if err != nil {
		log.Fatal(err)
	}

	var pkDeserialized PublicKey
	err = borsh.Deserialize(&pkDeserialized, pkSerialized)
	if err != nil {
		log.Fatal(err)
	}
	assert.Equal(t, pubKeyB58, base58.Encode(pkDeserialized.ED25519.Inner[:]))
	assert.Equal(t, pkDeserialized.Enum, borsh.Enum(0))
}
