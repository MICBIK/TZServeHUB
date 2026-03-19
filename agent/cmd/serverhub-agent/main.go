package main

import (
	"flag"
	"log"

	"github.com/ha1den/serverhub-agent/internal/config"
	"github.com/ha1den/serverhub-agent/internal/server"
)

func main() {
	cfgPath := flag.String("config", "config.yaml", "config file path")
	flag.Parse()

	cfg, err := config.Load(*cfgPath)
	if err != nil {
		log.Fatalf("failed to load config: %v", err)
	}

	log.Printf("ServerHUB Agent starting on :%d", cfg.Port)
	if err := server.Run(cfg); err != nil {
		log.Fatalf("server error: %v", err)
	}
}
