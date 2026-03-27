export interface PingProbeResult {
  avg_rtt_ms: number;
  loss_rate: number;
  packets_sent: number;
  packets_lost: number;
}

export interface TcpProbeResult {
  reachable: boolean;
  latency_ms: number;
}

export interface DnsProbeResult {
  resolved: boolean;
  latency_ms: number;
}
