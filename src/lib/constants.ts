export const POLLING_INTERVALS = [5, 10, 15, 30, 60] as const;

export const METRIC_COLORS = {
  cpu: '#1890ff',
  memory: '#52c41a',
  disk: '#faad14',
  network_rx: '#13c2c2',
  network_tx: '#722ed1',
  alert: '#ff4d4f',
} as const;

export const DEFAULT_CHART_HEIGHT = 300;
export const SPARKLINE_HEIGHT = 40;
