package config

import (
	"os"

	"gopkg.in/yaml.v3"
)

type Config struct {
	Port      int    `yaml:"port"`
	Token     string `yaml:"token"`
	Hostname  string `yaml:"hostname"`
	Interval  int    `yaml:"interval"` // collection interval in seconds
}

func Load(path string) (*Config, error) {
	cfg := &Config{
		Port:     9200,
		Interval: 5,
	}

	data, err := os.ReadFile(path)
	if err != nil {
		// Use defaults if no config file
		if os.IsNotExist(err) {
			return cfg, nil
		}
		return nil, err
	}

	if err := yaml.Unmarshal(data, cfg); err != nil {
		return nil, err
	}

	if cfg.Hostname == "" {
		cfg.Hostname, _ = os.Hostname()
	}

	return cfg, nil
}
