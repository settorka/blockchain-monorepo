package main

import (
	"log"
	"net"

	"google.golang.org/grpc"
	"mev-relay/internal/builder"
	"mev-relay/internal/pb"
	"mev-relay/internal/config"
)

func main() {
	cfg := config.Load()

	lis, err := net.Listen("tcp", ":"+cfg.BuilderPort)
	if err != nil {
		log.Fatal("Failed to listen:", err)
	}

	server := grpc.NewServer()
	pb.RegisterBuilderServiceServer(server, &builder.Service{})

	log.Println("Builder service running on port", cfg.BuilderPort)
	if err := server.Serve(lis); err != nil {
		log.Fatal("Builder failed:", err)
	}
}
