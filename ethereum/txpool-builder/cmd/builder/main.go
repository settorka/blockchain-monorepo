package main

import (
    "context"
    "log"
    "txpool-builder/internal/builder"
    "txpool-builder/internal/client"
)

func main() {
    ctx := context.Background()

    // Connect to local Geth node
    rpcClient, ethClient := client.Connect("http://127.0.0.1:8545")
    defer rpcClient.Close()

    // Fetch pending txs
    txs := builder.FetchPendingTxs(ctx, rpcClient)
    if len(txs) == 0 {
        log.Println("No pending transactions found in txpool.")
        return
    }

    // Rank and build pseudo block
    ranked := builder.RankByGasPrice(txs)
    builder.BuildBlock(ranked, 50)

    log.Println("pseudo_block.json generated successfully")
}
