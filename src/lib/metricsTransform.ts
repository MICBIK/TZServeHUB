import type { MetricPoint } from '../types/metric';

/**
 * Transform raw MetricPoint[] into structured metrics for dashboard components
 */

export interface TransformedMetrics {
  cpu: {
    total_percent: number;
    per_core: number[];
  };
  memory: {
    used_percent: number;
  };
  disk: {
    avg_used_percent: number;
  };
  network: {
    rx_rate: number;
    tx_rate: number;
    rx_total: number;
    tx_total: number;
  };
  disk_io: {
    read_rate: number;
    write_rate: number;
  };
}

interface CounterState {
  value: number;
  timestamp: number;
}

const counterCache = new Map<string, CounterState>();

function calculateRate(key: string, currentValue: number, currentTimestamp: number): number {
  const cached = counterCache.get(key);

  if (!cached) {
    counterCache.set(key, { value: currentValue, timestamp: currentTimestamp });
    return 0;
  }

  const timeDelta = (currentTimestamp - cached.timestamp) / 1000; // Convert to seconds
  if (timeDelta <= 0) return 0;

  // Handle counter reset
  const valueDelta = currentValue < cached.value ? currentValue : currentValue - cached.value;
  const rate = valueDelta / timeDelta;

  counterCache.set(key, { value: currentValue, timestamp: currentTimestamp });
  return rate;
}

export function transformMetrics(metrics: MetricPoint[]): TransformedMetrics {
  const result: TransformedMetrics = {
    cpu: { total_percent: 0, per_core: [] },
    memory: { used_percent: 0 },
    disk: { avg_used_percent: 0 },
    network: { rx_rate: 0, tx_rate: 0, rx_total: 0, tx_total: 0 },
    disk_io: { read_rate: 0, write_rate: 0 },
  };

  const diskUsages: number[] = [];
  const networkInterfaces = new Map<string, { rx: number; tx: number; timestamp: number }>();
  const diskDevices = new Map<string, { read: number; write: number; timestamp: number }>();

  for (const metric of metrics) {
    const { key, value, labels, timestamp, metric_type } = metric;

    // CPU metrics
    if (key === 'cpu_usage_percent' && !labels.core) {
      result.cpu.total_percent = value;
    } else if (key === 'cpu_usage_percent' && labels.core) {
      const coreIndex = parseInt(labels.core, 10);
      result.cpu.per_core[coreIndex] = value;
    }

    // Memory metrics
    if (key === 'memory_used_percent') {
      result.memory.used_percent = value;
    }

    // Disk usage metrics
    if (key === 'disk_used_percent') {
      diskUsages.push(value);
    }

    // Network metrics (counters need rate calculation)
    if (key === 'network_receive_bytes_total' && labels.interface) {
      const iface = labels.interface;
      const cacheKey = `net_rx_${iface}`;
      const rate = metric_type === 'counter' ? calculateRate(cacheKey, value, timestamp) : value;

      if (!networkInterfaces.has(iface)) {
        networkInterfaces.set(iface, { rx: 0, tx: 0, timestamp });
      }
      const ifaceData = networkInterfaces.get(iface)!;
      ifaceData.rx = rate;
      result.network.rx_total += value;
    }

    if (key === 'network_transmit_bytes_total' && labels.interface) {
      const iface = labels.interface;
      const cacheKey = `net_tx_${iface}`;
      const rate = metric_type === 'counter' ? calculateRate(cacheKey, value, timestamp) : value;

      if (!networkInterfaces.has(iface)) {
        networkInterfaces.set(iface, { rx: 0, tx: 0, timestamp });
      }
      const ifaceData = networkInterfaces.get(iface)!;
      ifaceData.tx = rate;
      result.network.tx_total += value;
    }

    // Disk I/O metrics (counters need rate calculation)
    if (key === 'disk_read_bytes_total' && labels.device) {
      const device = labels.device;
      const cacheKey = `disk_read_${device}`;
      const rate = metric_type === 'counter' ? calculateRate(cacheKey, value, timestamp) : value;

      if (!diskDevices.has(device)) {
        diskDevices.set(device, { read: 0, write: 0, timestamp });
      }
      diskDevices.get(device)!.read = rate;
    }

    if (key === 'disk_write_bytes_total' && labels.device) {
      const device = labels.device;
      const cacheKey = `disk_write_${device}`;
      const rate = metric_type === 'counter' ? calculateRate(cacheKey, value, timestamp) : value;

      if (!diskDevices.has(device)) {
        diskDevices.set(device, { read: 0, write: 0, timestamp });
      }
      diskDevices.get(device)!.write = rate;
    }
  }

  // Aggregate disk usage (average across all mounts)
  if (diskUsages.length > 0) {
    result.disk.avg_used_percent = diskUsages.reduce((a, b) => a + b, 0) / diskUsages.length;
  }

  // Aggregate network rates (sum across all interfaces)
  for (const ifaceData of networkInterfaces.values()) {
    result.network.rx_rate += ifaceData.rx;
    result.network.tx_rate += ifaceData.tx;
  }

  // Aggregate disk I/O rates (sum across all devices)
  for (const deviceData of diskDevices.values()) {
    result.disk_io.read_rate += deviceData.read;
    result.disk_io.write_rate += deviceData.write;
  }

  return result;
}

export function clearCounterCache() {
  counterCache.clear();
}
