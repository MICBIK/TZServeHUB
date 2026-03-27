import { useMetricsStore } from '../stores/metricsStore';
import { useServerStore } from '../stores/serverStore';
import type { MetricPoint } from '../types/metric';

const EMPTY_METRICS: MetricPoint[] = [];

export type MonitoringViewState =
  | 'hydrating'
  | 'server-error'
  | 'no-servers'
  | 'selection-required'
  | 'metrics-loading'
  | 'metrics-error'
  | 'no-metrics'
  | 'ready';

export function useMonitoringView() {
  const servers = useServerStore((state) => state.servers);
  const hydrated = useServerStore((state) => state.hydrated);
  const serverError = useServerStore((state) => state.error);
  const activeServerId = useServerStore((state) => state.activeServerId);
  const metrics = useMetricsStore((state) =>
    activeServerId ? state.current[activeServerId] ?? EMPTY_METRICS : EMPTY_METRICS,
  );
  const metricsLoading = useMetricsStore((state) => state.loading);
  const metricsError = useMetricsStore((state) => state.error);
  const activeServer =
    servers.find((server) => server.id === activeServerId) ?? null;

  let state: MonitoringViewState = 'ready';

  if (!hydrated) {
    state = 'hydrating';
  } else if (serverError) {
    state = 'server-error';
  } else if (servers.length === 0) {
    state = 'no-servers';
  } else if (!activeServerId) {
    state = 'selection-required';
  } else if (metricsLoading && metrics.length === 0) {
    state = 'metrics-loading';
  } else if (metricsError && metrics.length === 0) {
    state = 'metrics-error';
  } else if (metrics.length === 0) {
    state = 'no-metrics';
  }

  return {
    activeServer,
    activeServerId,
    hydrated,
    metrics,
    metricsError,
    metricsLoading,
    serverError,
    servers,
    staleMetricsError: metricsError && metrics.length > 0 ? metricsError : null,
    state,
  };
}
