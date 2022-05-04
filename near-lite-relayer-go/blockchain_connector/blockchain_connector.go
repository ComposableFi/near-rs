package blockchain_connector

import "fmt"

type NearNetwork string

const (
	Mainnet NearNetwork = "Mainnet"
	Testnet NearNetwork = "Testnet"
)

func (n NearNetwork) ToString() string {
	switch n {
	case Mainnet:
		return "mainnet"
	case Testnet:
		return "testnet"
	}
	panic("unreachable")
}

func (n NearNetwork) GetBaseUrl() string {
	return fmt.Sprintf("https://rpc.%s.near.org", n.ToString())
}

type BlockchainConector struct {
	network NearNetwork
}
