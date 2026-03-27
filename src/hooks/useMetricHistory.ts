import { startTransition, useEffect, useState } from 'react';
import { useMetricsStore } from '../stores/metricsStore';
import type { MetricHistoryResponse } from '../types/metric';

export interface HistoryPoint {
  timestamp: number;
  value: number;
}

function normalizeHistory(response: MetricHistoryResponse): HistoryPoint[] {
  if (response.resolution === 'raw') {
    return response.points.map((point) => ({
      timestamp: point.timestamp,
      value: point.value,
    }));
  }

  return response.buckets.map((bucket) => ({
    timestamp: bucket.bucket,
    value: bucket.avg_val,
  }));
}

export function useMetricHistory(
  serverId: string | null,
  key: string | null,
  refreshToken: number,
  labels?: Record<string, string>,
) {
  const fetchHistory = useMetricsStore((state) => state.fetchHistory);
  const [points, setPoints] = useState<HistoryPoint[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const labelsKey = JSON.stringify(labels ?? {});

  useEffect(() => {
    let cancelled = false;

    async function load() {
      if (!serverId || !key) {
        setPoints([]);
        setLoading(false);
        setError(null);
        return;
      }

      startTransition(() => setLoading(true));
      setError(null);

      const to = Math.max(
        refreshToken || Math.floor(Date.now() / 1000),
        Math.floor(Date.now() / 1000),
      );
      const from = to - 1800;
      const queryLabels = labelsKey === '{}' ? undefined : (JSON.parse(labelsKey) as Record<string, string>);

      try {
        const response = await fetchHistory(serverId, key, from, to, {
          labels: queryLabels,
          resolution: 'raw',
        });

        if (!cancelled) {
          setPoints(normalizeHistory(response));
          setLoading(false);
        }
      } catch (historyError) {
        if (!cancelled) {
          setPoints([]);
          setLoading(false);
          setError(
            historyError instanceof Error
              ? historyError.message
              : 'Failed to load history',
          );
        }
      }
    }

    void load();

    return () => {
      cancelled = true;
    };
  }, [fetchHistory, key, labelsKey, refreshToken, serverId]);

  return { points, loading, error };
}
