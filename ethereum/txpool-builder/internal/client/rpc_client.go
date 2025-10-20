package client

import (
	"log"

	"github.com/ethereum/go-ethereum/ethclient"
	"github.com/ethereum/go-ethereum/rpc"
)

// Connect establishes both RPC and Ethereum client connections
// to a specified Geth endpoint. Returns the raw RPC client and
// a higher-level ethclient wrapper for convenience.
func Connect(url string) (*rpc.Client, *ethclient.Client) {
	rpcClient, err := rpc.Dial(url)
	if err != nil {
		log.Fatalf("failed to connect to RPC endpoint: %v", err)
	}

	ethClient := ethclient.NewClient(rpcClient)
	log.Println("connected to Geth node")
	return rpcClient, ethClient
}
