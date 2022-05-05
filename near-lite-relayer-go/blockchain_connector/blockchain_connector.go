package blockchain_connector

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"

	"github.com/ComposableFi/near-trustless-bridge/near-lite-relayer-go/types"
)

type NearNetwork string

const (
	Mainnet        NearNetwork = "Mainnet"
	Testnet        NearNetwork = "Testnet"
	ArchiveMainnet NearNetwork = "ArchiveMainnet"
	ArchiveTestnet NearNetwork = "ArchiveTestnet"
)

func (n NearNetwork) ToString() string {
	switch n {
	case Mainnet, ArchiveMainnet:
		return "mainnet"
	case Testnet, ArchiveTestnet:
		return "testnet"
	}
	panic("unreachable")
}

func (n NearNetwork) getBaseUrl() string {
	switch n {
	case Mainnet, Testnet:
		return fmt.Sprintf("https://rpc.%s.near.org", n.ToString())
	case ArchiveMainnet, ArchiveTestnet:
		return fmt.Sprintf("https://archival-rpc.%s.near.org", n.ToString())
	}
	panic("unreachable")
}

type BlockchainConector struct {
	network NearNetwork
}

func NewBlockchainConnector(network NearNetwork) *BlockchainConector {
	return &BlockchainConector{network: network}
}

func (bc *BlockchainConector) GetLightClientBlockView(lastKnownHash types.Base58CryptoHash) (*types.LightClientBlockView, error) {
	url := fmt.Sprintf("%s/", bc.network.getBaseUrl())

	postBody, _ := json.Marshal(map[string]interface{}{
		"jsonrpc": "2.0",
		"method":  "next_light_client_block",
		"params":  []string{lastKnownHash},
		"id":      "idontcare",
	})
	responseBody := bytes.NewBuffer(postBody)
	resp, err := http.Post(url, "application/json", responseBody)
	if err != nil {
		return nil, err
	}

	defer resp.Body.Close()
	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return nil, err
	}

	type response struct {
		Result types.LightClientBlockViewJson `json:"result"`
	}

	var r response
	err = json.Unmarshal(body, &r)
	if err != nil {
		return nil, err
	}
	return r.Result.IntoLightClientBlockView()
}
