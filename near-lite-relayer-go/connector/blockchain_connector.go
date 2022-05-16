// Package connector implements a blockchain connector to NEAR
package connector

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
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

func (n NearNetwork) getBaseURL() string {
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

	postBody, _ := json.Marshal(map[string]interface{}{
		"jsonrpc": "2.0",
		"method":  "next_light_client_block",
		"params":  []string{lastKnownHash},
		"id":      "idontcare",
	})
	responseBody := bytes.NewBuffer(postBody)
	ctx := context.Background()
	req, err := http.NewRequestWithContext(ctx, "POST", fmt.Sprintf("%s/", bc.network.getBaseURL()), responseBody)
	req.Header.Set("Content-Type", "application/json")
	if err != nil {
		return nil, err
	}

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return nil, err
	}

	defer func() {
		err := resp.Body.Close()
		if err != nil {
			log.Println(err)
		}
	}()
	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return nil, err
	}

	type response struct {
		Result types.LightClientBlockViewJSON `json:"result"`
	}

	var r response
	err = json.Unmarshal(body, &r)
	if err != nil {
		return nil, err
	}
	return r.Result.IntoLightClientBlockView()
}
