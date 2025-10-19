package simulator

import (
	"context"
	"log"
	"time"

	"mev-relay/internal/config"
	"mev-relay/internal/pb"
)

// Service implements pb.SimulationServiceServer
type Service struct {
	cfg *config.Config
}

// mustEmbedUnimplementedSimulationServiceServer implements pb.SimulationServiceServer.
func (s *Service) mustEmbedUnimplementedSimulationServiceServer() {
	panic("unimplemented")
}

func NewService(cfg *config.Config) *Service {
	return &Service{cfg: cfg}
}

// SimulateBundle runs bundle simulation against a local EVM node (Anvil/Geth)
func (s *Service) SimulateBundle(ctx context.Context, req *pb.BundleRequest) (*pb.BundleResponse, error) {
	start := time.Now()

	log.Printf("[Simulator] Simulating bundle %s targeting block %s (%d txs)",
		req.BundleId, req.TargetBlock, len(req.Txs))

	success, profit, err := RunSimulation(s.cfg, req.Txs)
	latency := time.Since(start).Milliseconds()

	if err != nil {
		log.Println("[Simulator] Simulation failed:", err)
		return &pb.BundleResponse{
			BundleId:  req.BundleId,
			ProfitEth: 0,
			LatencyMs: latency,
			Success:   false,
			Reason:    err.Error(),
		}, nil
	}

	log.Printf("[Simulator] Bundle %s profit=%.6f ETH (success=%v)", req.BundleId, profit, success)

	return &pb.BundleResponse{
		BundleId:  req.BundleId,
		ProfitEth: profit,
		LatencyMs: latency,
		Success:   success,
		Reason:    "",
	}, nil
}
