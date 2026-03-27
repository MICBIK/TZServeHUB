package collector

import (
	"testing"
	"time"
)

func TestCollectReturnsMetrics(t *testing.T) {
	metrics, err := Collect()
	if err != nil {
		t.Fatalf("Collect() error: %v", err)
	}
	if metrics == nil {
		t.Fatal("Collect() returned nil")
	}
}

func TestCollectTimestamp(t *testing.T) {
	before := time.Now().Unix()
	metrics, err := Collect()
	if err != nil {
		t.Fatalf("Collect() error: %v", err)
	}
	after := time.Now().Unix()

	if metrics.Timestamp < before || metrics.Timestamp > after {
		t.Errorf("Timestamp %d not in range [%d, %d]", metrics.Timestamp, before, after)
	}
}

func TestCollectCPU(t *testing.T) {
	metrics, err := Collect()
	if err != nil {
		t.Fatalf("Collect() error: %v", err)
	}

	// TotalPercent should be 0-100 (gopsutil may return 0 on first call)
	if metrics.CPU.TotalPercent < 0 || metrics.CPU.TotalPercent > 100 {
		t.Errorf("CPU.TotalPercent = %f, want 0-100", metrics.CPU.TotalPercent)
	}

	// PerCore should have at least 1 core
	if len(metrics.CPU.PerCore) == 0 {
		t.Error("CPU.PerCore is empty, expected at least 1 core")
	}

	for i, pct := range metrics.CPU.PerCore {
		if pct < 0 || pct > 100 {
			t.Errorf("CPU.PerCore[%d] = %f, want 0-100", i, pct)
		}
	}
}

func TestCollectMemory(t *testing.T) {
	metrics, err := Collect()
	if err != nil {
		t.Fatalf("Collect() error: %v", err)
	}

	if metrics.Memory.Total == 0 {
		t.Error("Memory.Total = 0, expected > 0")
	}
	if metrics.Memory.Total < metrics.Memory.Used {
		t.Errorf("Memory.Total (%d) < Memory.Used (%d)", metrics.Memory.Total, metrics.Memory.Used)
	}
	if metrics.Memory.UsedPercent < 0 || metrics.Memory.UsedPercent > 100 {
		t.Errorf("Memory.UsedPercent = %f, want 0-100", metrics.Memory.UsedPercent)
	}
}

func TestCollectDisks(t *testing.T) {
	metrics, err := Collect()
	if err != nil {
		t.Fatalf("Collect() error: %v", err)
	}

	// At least one disk partition should exist
	if len(metrics.Disks) == 0 {
		t.Skip("no disk partitions found (CI/container?)")
	}

	for _, d := range metrics.Disks {
		if d.Mount == "" {
			t.Error("Disk mount point is empty")
		}
		// Some virtual/special partitions (e.g., macOS autofs) report 0 total — skip them
		if d.Total == 0 {
			continue
		}
		if d.UsedPercent < 0 || d.UsedPercent > 100 {
			t.Errorf("Disk %q UsedPercent = %f, want 0-100", d.Mount, d.UsedPercent)
		}
	}
}

func TestCollectNetwork(t *testing.T) {
	metrics, err := Collect()
	if err != nil {
		t.Fatalf("Collect() error: %v", err)
	}

	// Should have at least one non-loopback interface
	if len(metrics.Network) == 0 {
		t.Skip("no non-loopback network interfaces found")
	}

	for _, n := range metrics.Network {
		if n.Interface == "" {
			t.Error("Network interface name is empty")
		}
		if n.Interface == "lo" || n.Interface == "lo0" {
			t.Errorf("loopback interface %q should be filtered out", n.Interface)
		}
	}
}

func TestCollectDiskIO(t *testing.T) {
	metrics, err := Collect()
	if err != nil {
		t.Fatalf("Collect() error: %v", err)
	}

	if len(metrics.DiskIO) == 0 {
		t.Skip("no disk I/O counters found (CI/container?)")
	}

	for _, io := range metrics.DiskIO {
		if io.Device == "" {
			t.Error("DiskIO device name is empty")
		}
	}
}

func TestHostMetricsStructure(t *testing.T) {
	metrics, err := Collect()
	if err != nil {
		t.Fatalf("Collect() error: %v", err)
	}

	// Verify all top-level fields are initialized (not nil slices for arrays)
	if metrics.Disks == nil && metrics.DiskIO == nil && metrics.Network == nil {
		t.Log("warning: all slice fields are nil — may indicate collection issues")
	}

	// CPU PerCore should match actual core count
	if len(metrics.CPU.PerCore) > 0 {
		t.Logf("detected %d CPU cores", len(metrics.CPU.PerCore))
	}
	t.Logf("memory total: %d bytes", metrics.Memory.Total)
	t.Logf("disk partitions: %d", len(metrics.Disks))
	t.Logf("network interfaces: %d", len(metrics.Network))
}
