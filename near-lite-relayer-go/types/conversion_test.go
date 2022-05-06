package types

import (
	"encoding/json"
	"math/big"
	"testing"

	"github.com/btcsuite/btcutil/base58"
	"github.com/stretchr/testify/assert"
)

func TestIntoNextValidatorStakeViews(t *testing.T) {
	payload := `{
		"account_id": "node1",
		"public_key": "ed25519:ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su",
		"stake": "23274801326131528968305033242063",
		"validator_stake_struct_version": "V1"
	}`

	var r json.RawMessage
	err := json.Unmarshal([]byte(payload), &r)
	if err != nil {
		panic(err)
	}
	result, err := IntoNextValidatorStakeViews([]json.RawMessage{r})
	if err != nil {
		panic(err)
	}
	assert.Equal(t, result[0].V1.AccountId, "node1")
	assert.Equal(t, result[0].V1.PublicKey.ED25519[:], base58.Decode("ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su"))
	b := big.Int{}
	b.SetString("23274801326131528968305033242063", 10)
	assert.Equal(t, result[0].V1.Stake, b)
}

func TestIntoIntoSignatures(t *testing.T) {
	payload := []byte(`
		[
			null,
			"ed25519:4qnb1YmQngt9X3M88igWTWWPxX8GLwjYh6nHYYBGhZs5vFP5JxRNS8MqTNjn9eBebkd5mw72cM5emDKVfMY7hMrc",
			null
		]
	`)

	var response []*json.RawMessage
	err := json.Unmarshal(payload, &response)
	if err != nil {
		panic(err)
	}
	signatures, err := IntoSignatures(response)
	if err != nil {
		panic(err)
	}
	var s [64]byte
	copy(s[:], base58.Decode("4qnb1YmQngt9X3M88igWTWWPxX8GLwjYh6nHYYBGhZs5vFP5JxRNS8MqTNjn9eBebkd5mw72cM5emDKVfMY7hMrc"))
	assert.Nil(t, signatures[0])
	assert.Equal(t, signatures[1], &Signature{
		ED25519: s,
	})
	assert.Nil(t, signatures[2])

}