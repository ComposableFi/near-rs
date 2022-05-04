package blockchain_connector

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestNearNetwork(t *testing.T) {
	testnet := Testnet
	mainnet := Mainnet
	assert.Equal(t, "testnet", testnet.ToString())
	assert.Equal(t, "mainnet", mainnet.ToString())
	assert.Equal(t, "https://rpc.testnet.near.org", testnet.GetBaseUrl())
	assert.Equal(t, "https://rpc.mainnet.near.org", mainnet.GetBaseUrl())
}
