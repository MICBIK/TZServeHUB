import { useMetricsStore } from '../stores/metricsStore';
import { useServerStore } from '../stores/serverStore';
import type { MetricPoint } from '../types/metric';
import type { ServerConfig } from '../types/server';

export interface ServerMetrics {
  server: ServerConfig;
  metrics: MetricPoint[];
  loading: boolean;
}

export function useAllServersMetrics(): ServerMetrics[] {
  const servers = useServerStore((state) => state.servers);
  const allMetrics = useMetricsStore((state) => state.current);
  const loading = useMetricsStore((state) => state.loading);

  return servers.map((server) => ({
    server,
    metrics: allMetrics[server.id] ?? [],
    loading,
  }));
}
