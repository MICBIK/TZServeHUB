package config

import (
	"os"
	"path/filepath"
	"testing"
)

func TestLoadValidConfig(t *testing.T) {
	content := `port: 8080
token: "test-token"
hostname: "test-host"
interval: 10
`
	tmp := filepath.Join(t.TempDir(), "config.yaml")
	if err := os.WriteFile(tmp, []byte(content), 0644); err != nil {
		t.Fatal(err)
	}

	cfg, err := Load(tmp)
	if err != nil {
		t.Fatalf("Load() error: %v", err)
	}

	if cfg.Port != 8080 {
		t.Errorf("Port = %d, want 8080", cfg.Port)
	}
	if cfg.Token != "test-token" {
		t.Errorf("Token = %q, want %q", cfg.Token, "test-token")
	}
	if cfg.Hostname != "test-host" {
		t.Errorf("Hostname = %q, want %q", cfg.Hostname, "test-host")
	}
	if cfg.Interval != 10 {
		t.Errorf("Interval = %d, want 10", cfg.Interval)
	}
}

func TestLoadDefaults(t *testing.T) {
	// Non-existent file should return defaults without error
	cfg, err := Load("/tmp/does-not-exist-serverhub.yaml")
	if err != nil {
		t.Fatalf("Load() error for missing file: %v", err)
	}

	if cfg.Port != 9100 {
		t.Errorf("default Port = %d, want 9100", cfg.Port)
	}
	if cfg.Interval != 5 {
		t.Errorf("default Interval = %d, want 5", cfg.Interval)
	}
	if cfg.Hostname == "" {
		t.Error("default Hostname should be set from os.Hostname()")
	}
}

func TestLoadPartialConfig(t *testing.T) {
	// Config with only token set — port and interval should keep defaults
	content := `token: "partial-token"
`
	tmp := filepath.Join(t.TempDir(), "partial.yaml")
	if err := os.WriteFile(tmp, []byte(content), 0644); err != nil {
		t.Fatal(err)
	}

	cfg, err := Load(tmp)
	if err != nil {
		t.Fatalf("Load() error: %v", err)
	}

	if cfg.Port != 9100 {
		t.Errorf("Port = %d, want default 9100", cfg.Port)
	}
	if cfg.Interval != 5 {
		t.Errorf("Interval = %d, want default 5", cfg.Interval)
	}
	if cfg.Token != "partial-token" {
		t.Errorf("Token = %q, want %q", cfg.Token, "partial-token")
	}
}

func TestLoadInvalidYAML(t *testing.T) {
	content := `port: [invalid yaml structure`
	tmp := filepath.Join(t.TempDir(), "bad.yaml")
	if err := os.WriteFile(tmp, []byte(content), 0644); err != nil {
		t.Fatal(err)
	}

	_, err := Load(tmp)
	if err == nil {
		t.Error("Load() should return error for invalid YAML")
	}
}

func TestLoadHostnameAutoDetect(t *testing.T) {
	// Empty hostname in config should trigger os.Hostname()
	content := `port: 9100
hostname: ""
`
	tmp := filepath.Join(t.TempDir(), "nohost.yaml")
	if err := os.WriteFile(tmp, []byte(content), 0644); err != nil {
		t.Fatal(err)
	}

	cfg, err := Load(tmp)
	if err != nil {
		t.Fatalf("Load() error: %v", err)
	}

	expected, _ := os.Hostname()
	if cfg.Hostname != expected {
		t.Errorf("Hostname = %q, want %q (from os.Hostname)", cfg.Hostname, expected)
	}
}
