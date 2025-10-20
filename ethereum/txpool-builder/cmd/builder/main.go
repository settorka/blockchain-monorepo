package main

import (
	"context"
	"log"
	"os"
	"txpool-builder/internal/builder"
	"txpool-builder/internal/client"
)

// main initializes the connection to a Geth node using the
// GETH_RPC_URL environment variable. It retrieves pending
// transactions, ranks them by gas price, and assembles a pseudo block.
func main() {
	ctx := context.Background()

	rpcURL := os.Getenv("GETH_RPC_URL")
	if rpcURL == "" {
		log.Fatal("missing GETH_RPC_URL environment variable")
	}

	rpcClient, ethClient := client.Connect(rpcURL)
	defer rpcClient.Close()
	_ = ethClient

	txs := builder.FetchPendingTxs(ctx, rpcClient)
	if len(txs) == 0 {
		log.Println("no pending transactions found in txpool")
		return
	}

	ranked := builder.RankByGasPrice(txs)
	builder.BuildBlock(ranked, 50)

	log.Println("pseudo_block.json generated successfully")
}
