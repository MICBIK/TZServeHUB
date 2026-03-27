import { useEffect } from 'react';
import { useMetricsStore } from '../stores/metricsStore';
import type { ServerConfig } from '../types/server';

export function usePolling(servers: ServerConfig[]) {
  const fetchMetrics = useMetricsStore((state) => state.fetchMetrics);

  useEffect(() => {
    const enabledServers = servers.filter((server) => server.enabled);

    if (enabledServers.length === 0) {
      return undefined;
    }

    const timers = enabledServers.map((server) => {
      void fetchMetrics(server.id);

      return window.setInterval(() => {
        void fetchMetrics(server.id);
      }, Math.max(server.polling_interval_sec, 1) * 1000);
    });

    return () => {
      timers.forEach((timer) => window.clearInterval(timer));
    };
  }, [fetchMetrics, servers]);
}
