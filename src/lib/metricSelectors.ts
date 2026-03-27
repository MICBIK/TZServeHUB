import type { MetricPoint } from '../types/metric';

interface DiskUsageSummary {
  label: string;
  device: string;
  used: number;
  total: number;
  percent: number;
}

interface RateSummary {
  label: string;
  rx?: number;
  tx?: number;
  read?: number;
  write?: number;
}

interface MemorySummary {
  total: number;
  used: number;
  free: number;
  cached: number;
  available: number;
  percent: number;
}

function matchesLabels(metric: MetricPoint, labels?: Record<string, string>) {
  if (!labels) {
    return true;
  }

  return Object.entries(labels).every(([key, value]) => metric.labels[key] === value);
}

export function pickMetric(metrics: MetricPoint[], key: string, labels?: Record<string, string>) {
  return metrics.find((metric) => metric.key === key && matchesLabels(metric, labels));
}

export function pickMetrics(metrics: MetricPoint[], key: string) {
  return metrics.filter((metric) => metric.key === key);
}

export function getCpuMetrics(metrics: MetricPoint[]) {
  return {
    total: pickMetric(metrics, 'cpu_usage_percent'),
    perCore: [...pickMetrics(metrics, 'cpu_core_usage_percent')].sort(
      (left, right) => Number(left.labels.core ?? 0) - Number(right.labels.core ?? 0),
    ),
  };
}

export function getMemoryMetric(metrics: MetricPoint[]) {
  return pickMetric(metrics, 'memory_used_percent');
}

export function getMemorySummary(metrics: MetricPoint[]): MemorySummary | null {
  const total = pickMetric(metrics, 'memory_total_bytes');
  const used = pickMetric(metrics, 'memory_used_bytes');
  const free = pickMetric(metrics, 'memory_free_bytes');
  const cached = pickMetric(metrics, 'memory_cached_bytes');
  const available = pickMetric(metrics, 'memory_available_bytes');
  const percent = pickMetric(metrics, 'memory_used_percent');

  if (!total || !used || !percent) {
    return null;
  }

  const cachedValue = cached?.value ?? Math.max((available?.value ?? 0) - (free?.value ?? 0), 0);
  const freeValue = free?.value ?? Math.max((available?.value ?? 0) - cachedValue, 0);
  const availableValue = available?.value ?? freeValue + cachedValue;

  return {
    total: total.value,
    used: used.value,
    free: freeValue,
    cached: cachedValue,
    available: availableValue,
    percent: percent.value,
  };
}

export function getDiskUsage(metrics: MetricPoint[]): DiskUsageSummary[] {
  const totals = pickMetrics(metrics, 'disk_total_bytes');
  const used = pickMetrics(metrics, 'disk_used_bytes');

  return totals
    .map((totalMetric) => {
      const label = totalMetric.labels.mount ?? totalMetric.labels.device ?? 'disk';
      const usedMetric = used.find((metric) => matchesLabels(metric, totalMetric.labels));

      if (!usedMetric || totalMetric.value <= 0) {
        return null;
      }

      return {
        label,
        device: totalMetric.labels.device ?? label,
        total: totalMetric.value,
        used: usedMetric.value,
        percent: (usedMetric.value / totalMetric.value) * 100,
      };
    })
    .filter((value): value is DiskUsageSummary => value !== null);
}

export function getNetworkRates(metrics: MetricPoint[]): RateSummary[] {
  const rx = pickMetrics(metrics, 'network_receive_bytes_total_rate');
  const tx = pickMetrics(metrics, 'network_transmit_bytes_total_rate');

  return rx.map((metric) => ({
    label: metric.labels.interface ?? 'network',
    rx: metric.value,
    tx: tx.find((candidate) => matchesLabels(candidate, metric.labels))?.value,
  }));
}

export function getDiskRates(metrics: MetricPoint[]): RateSummary[] {
  const read = pickMetrics(metrics, 'disk_read_bytes_total_rate');
  const write = pickMetrics(metrics, 'disk_write_bytes_total_rate');

  return read.map((metric) => ({
    label: metric.labels.device ?? 'disk',
    read: metric.value,
    write: write.find((candidate) => matchesLabels(candidate, metric.labels))?.value,
  }));
}
