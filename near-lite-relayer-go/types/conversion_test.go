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
