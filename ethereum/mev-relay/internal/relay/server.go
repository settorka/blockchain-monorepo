package relay

import (
	"log"
	"net/http"

	"github.com/gin-gonic/gin"
	"mev-relay/internal/config"
)

// StartServer starts the JSON-RPC server for incoming searcher bundles
func StartServer(cfg *config.Config) error {
	router := gin.Default()

	// Define the single entrypoint for bundle submission
	router.POST("/relay/v1/bundle", func(c *gin.Context) {
		handleBundleRequest(c, cfg)
	})

	addr := ":" + cfg.RelayPort
	log.Println("[Relay] Listening on", addr)
	return router.Run(addr)
}
