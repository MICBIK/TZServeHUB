import type { HistoryPoint } from '../hooks/useMetricHistory';

export type HistorySeriesRow = { timestamp: number } & Record<string, number>;

export function mergeHistorySeries(
  series: Array<{ key: string; points: HistoryPoint[] }>,
): HistorySeriesRow[] {
  const byTimestamp = new Map<number, HistorySeriesRow>();

  for (const item of series) {
    for (const point of item.points) {
      const row = byTimestamp.get(point.timestamp) ?? { timestamp: point.timestamp };
      row[item.key] = point.value;
      byTimestamp.set(point.timestamp, row);
    }
  }

  return [...byTimestamp.values()].sort(
    (left, right) => left.timestamp - right.timestamp,
  );
}

export function latestMetricTimestamp(
  metrics: Array<{ timestamp: number }>,
) {
  return metrics.reduce((latest, metric) => Math.max(latest, metric.timestamp), 0);
}
