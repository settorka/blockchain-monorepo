package builder

import (
	"context"
	"log"
	"sync"
	"time"

	"mev-relay/internal/pb"
)

// Service implements pb.BuilderServiceServer
type Service struct {
	mu       sync.Mutex
	pending  []*pb.BundleSubmission
	history  []pb.BuildResult
}

// SubmitBundle receives a profitable bundle from the relay
func (s *Service) SubmitBundle(ctx context.Context, req *pb.BundleSubmission) (*pb.BuildResult, error) {
	s.mu.Lock()
	defer s.mu.Unlock()

	log.Printf("[Builder] Received bundle: %s, profit=%.6f ETH", req.BundleId, req.ProfitEth)

	// Add to pending queue
	s.pending = append(s.pending, req)

	// Simulate block building
	block := BuildCandidateBlock(s.pending)

	// Rank bundles by profitability
	ranked := RankBundles(block.Bundles)

	// Simulate inclusion
	selected := ranked[0]
	log.Printf("[Builder] Selected bundle %s for inclusion", selected.BundleId)

	// Simulated latency
	time.Sleep(300 * time.Millisecond)

	result := &pb.BuildResult{
		BlockHash:          GenerateBlockHash(selected.BundleId),
		Included:           true,
		InclusionReason:    "Highest profit",
		InclusionLatencyMs: 300,
	}

	// Record block build result
	s.history = append(s.history, *result)
	s.pending = []*pb.BundleSubmission{} // reset pending

	return result, nil
}
