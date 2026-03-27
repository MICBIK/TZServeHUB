package collector

import (
	"time"

	"github.com/shirou/gopsutil/v4/cpu"
	"github.com/shirou/gopsutil/v4/disk"
	"github.com/shirou/gopsutil/v4/mem"
	"github.com/shirou/gopsutil/v4/net"
)

type HostMetrics struct {
	Timestamp   int64          `json:"timestamp"`
	CPU         CPUMetrics     `json:"cpu"`
	Memory      MemoryMetrics  `json:"memory"`
	Disks       []DiskMetrics  `json:"disks"`
	DiskIO      []DiskIO       `json:"disk_io"`
	Network     []NetMetrics   `json:"network"`
}

type CPUMetrics struct {
	TotalPercent float64   `json:"total_percent"`
	PerCore      []float64 `json:"per_core"`
}

type MemoryMetrics struct {
	Total       uint64  `json:"total"`
	Used        uint64  `json:"used"`
	Free        uint64  `json:"free"`
	Cached      uint64  `json:"cached"`
	Available   uint64  `json:"available"`
	UsedPercent float64 `json:"used_percent"`
}

type DiskMetrics struct {
	Mount       string  `json:"mount"`
	Device      string  `json:"device"`
	Total       uint64  `json:"total"`
	Used        uint64  `json:"used"`
	Free        uint64  `json:"free"`
	UsedPercent float64 `json:"used_percent"`
}

type DiskIO struct {
	Device     string `json:"device"`
	ReadBytes  uint64 `json:"read_bytes"`
	WriteBytes uint64 `json:"write_bytes"`
}

type NetMetrics struct {
	Interface string `json:"interface"`
	RxBytes   uint64 `json:"rx_bytes"`
	TxBytes   uint64 `json:"tx_bytes"`
}

func Collect() (*HostMetrics, error) {
	m := &HostMetrics{
		Timestamp: time.Now().Unix(),
	}

	// CPU
	totalPct, err := cpu.Percent(0, false)
	if err == nil && len(totalPct) > 0 {
		m.CPU.TotalPercent = totalPct[0]
	}
	perCore, err := cpu.Percent(0, true)
	if err == nil {
		m.CPU.PerCore = perCore
	}

	// Memory
	vm, err := mem.VirtualMemory()
	if err == nil {
		m.Memory = MemoryMetrics{
			Total:       vm.Total,
			Used:        vm.Used,
			Free:        vm.Free,
			Cached:      vm.Cached,
			Available:   vm.Available,
			UsedPercent: vm.UsedPercent,
		}
	}

	// Disk usage
	partitions, err := disk.Partitions(false)
	if err == nil {
		for _, p := range partitions {
			usage, err := disk.Usage(p.Mountpoint)
			if err != nil {
				continue
			}
			m.Disks = append(m.Disks, DiskMetrics{
				Mount:       p.Mountpoint,
				Device:      p.Device,
				Total:       usage.Total,
				Used:        usage.Used,
				Free:        usage.Free,
				UsedPercent: usage.UsedPercent,
			})
		}
	}

	// Disk I/O (counters)
	ioCounters, err := disk.IOCounters()
	if err == nil {
		for name, io := range ioCounters {
			m.DiskIO = append(m.DiskIO, DiskIO{
				Device:     name,
				ReadBytes:  io.ReadBytes,
				WriteBytes: io.WriteBytes,
			})
		}
	}

	// Network (counters)
	netIO, err := net.IOCounters(true)
	if err == nil {
		for _, n := range netIO {
			if n.Name == "lo" || n.Name == "lo0" {
				continue
			}
			m.Network = append(m.Network, NetMetrics{
				Interface: n.Name,
				RxBytes:   n.BytesRecv,
				TxBytes:   n.BytesSent,
			})
		}
	}

	return m, nil
}
