export interface MetricPoint {
  server_id: string;
  key: string;
  value: number;
  metric_type: 'counter' | 'gauge' | 'state';
  vantage_point: string;
  labels: Record<string, string>;
  timestamp: number;
}

export interface AggregatedMetric {
  server_id: string;
  key: string;
  labels: Record<string, string>;
  vantage_point: string;
  resolution: '1m' | '15m';
  min_val: number;
  max_val: number;
  avg_val: number;
  bucket: number;
}

export type MetricHistoryResponse =
  | { kind: 'raw'; resolution: 'raw'; points: MetricPoint[] }
  | { kind: 'rollup'; resolution: '1m' | '15m'; buckets: AggregatedMetric[] };

export interface HostMetrics {
  timestamp: number;
  cpu: {
    total_percent: number;
    per_core: number[];
  };
  memory: {
    total: number;
    used: number;
    free: number;
    cached: number;
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
