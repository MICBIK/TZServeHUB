import { useEffect, useRef } from 'react';
import { useMetricsStore } from '../stores/metricsStore';

export function usePolling(serverId: string | null, intervalMs: number = 10000) {
  const fetchMetrics = useMetricsStore((s) => s.fetchMetrics);
  const timerRef = useRef<ReturnType<typeof setInterval> | null>(null);

  useEffect(() => {
    if (!serverId) return;

    // Fetch immediately
    fetchMetrics(serverId);

    // Then poll
    timerRef.current = setInterval(() => {
      fetchMetrics(serverId);
    }, intervalMs);

    return () => {
      if (timerRef.current) {
        clearInterval(timerRef.current);
      }
    };
  }, [serverId, intervalMs, fetchMetrics]);
}
