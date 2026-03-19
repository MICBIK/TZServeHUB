export interface MetricPoint {
  server_id: string;
  key: string;
  value: number;
  metric_type: 'counter' | 'gauge' | 'state';
  vantage_point: string;
  timestamp: number;
}

export interface AggregatedMetric {
  server_id: string;
  key: string;
  min_val: number;
  max_val: number;
  avg_val: number;
  bucket: number;
}

export interface HostMetrics {
  timestamp: number;
  cpu: {
    total_percent: number;
    per_core: number[];
  };
  memory: {
    total: number;
    used: number;
    available: number;
    used_percent: number;
  };
  disks: {
    mount: string;
    device: string;
    total: number;
    used: number;
    free: number;
    used_percent: number;
  }[];
  disk_io: {
    device: string;
    read_bytes: number;
    write_bytes: number;
  }[];
  network: {
    interface: string;
    rx_bytes: number;
    tx_bytes: number;
  }[];
}
