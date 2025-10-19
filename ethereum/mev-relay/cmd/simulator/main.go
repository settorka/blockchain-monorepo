package main

import (
	"log"
	"net"

	"google.golang.org/grpc"
	"mev-relay/internal/simulator"
	"mev-relay/internal/pb"
	"mev-relay/internal/config"
)

func main() {
	cfg := config.Load()

	lis, err := net.Listen("tcp", ":"+cfg.SimulatorPort)
	if err != nil {
		log.Fatal("Failed to listen:", err)
	}

	server := grpc.NewServer()
	pb.RegisterSimulationServiceServer(server, &simulator.Service{})

	log.Println("Simulation service running on port", cfg.SimulatorPort)
	if err := server.Serve(lis); err != nil {
		log.Fatal("Simulator failed:", err)
	}
}
