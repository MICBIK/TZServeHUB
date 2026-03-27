import type { MetricHistoryResponse, MetricPoint } from '../types/metric';
import type { ServerConfig, ServerFormData } from '../types/server';

export type BrowserThemeMode = 'dark' | 'light';
export type BrowserLanguageMode = 'zh-CN' | 'en-US';

export interface BrowserAppSettings {
  default_polling_interval_sec: number;
  data_retention_days: number;
  theme: BrowserThemeMode;
  language: BrowserLanguageMode;
}

export interface BrowserMetricHistoryOptions {
  labels?: Record<string, string>;
  resolution?: 'raw' | '1m' | '15m';
}

const STORAGE_PREFIX = 'serverhub-browser-demo-v2';
const HISTORY_POINTS = 36;
const HISTORY_STEP_SECONDS = 60;

const defaultSettings: BrowserAppSettings = {
  default_polling_interval_sec: 10,
  data_retention_days: 7,
  theme: 'dark',
  language: 'zh-CN',
};

const baseServers: ServerConfig[] = [
  createBaseServer('black-lodge', '10.10.0.12', 9100),
  createBaseServer('double-r', '10.10.0.18', 9100),
  createBaseServer('great-northern', '10.10.0.21', 9100),
  createBaseServer('localhost', '127.0.0.1', 9100),
  createBaseServer('log-lady', '10.10.0.29', 9100),
  createBaseServer('one-eyed-jacks', '10.10.0.33', 9100),
];

type BrowserState = {
  servers: ServerConfig[];
  settings: BrowserAppSettings;
};

let memoryState: BrowserState = {
  servers: baseServers.map(withTimestamps),
  settings: defaultSettings,
};

function createBaseServer(name: string, host: string, port: number): ServerConfig {
  return {
    id: `browser-demo-${name}`,
    name,
    host,
    port,
    adapter_type: 'go_agent',
    access_method: 'private',
    polling_interval_sec: 10,
    enabled: true,
    auth_token: null,
    auth_type: 'none',
    created_at: 0,
    updated_at: 0,
  };
}

function storageAvailable() {
  return typeof window !== 'undefined' && typeof window.localStorage !== 'undefined';
}

function readState(): BrowserState {
  if (!storageAvailable()) {
    return memoryState;
  }

  const raw = window.localStorage.getItem(STORAGE_PREFIX);
  if (!raw) {
    return memoryState;
  }

  try {
    const parsed = JSON.parse(raw) as BrowserState;
    return {
      servers: parsed.servers?.length ? parsed.servers : memoryState.servers,
      settings: parsed.settings ?? memoryState.settings,
    };
  } catch {
    return memoryState;
  }
}

function writeState(next: BrowserState) {
  memoryState = next;
  if (storageAvailable()) {
    window.localStorage.setItem(STORAGE_PREFIX, JSON.stringify(next));
  }
}

function withTimestamps(server: ServerConfig): ServerConfig {
  const now = Math.floor(Date.now() / 1000);
  return {
    ...server,
    created_at: server.created_at || now,
    updated_at: now,
  };
}

function clamp(value: number, min: number, max: number) {
  return Math.min(Math.max(value, min), max);
}

function seedOf(input: string) {
  return [...input].reduce((sum, char, index) => sum + char.charCodeAt(0) * (index + 1), 0);
}

function wave(index: number, seed: number, base: number, amplitude: number, spread: number, drift = 0) {
  const phase = (seed % 11) / 3.5;
  const scale = 1 + ((seed % 9) - 4) * 0.04;
  return clamp(
    base * scale
      + Math.sin(index / spread + phase) * amplitude
      + Math.cos(index / (spread + 1.8) + phase / 2) * amplitude * 0.45
      + drift * index,
    0,
    Number.MAX_SAFE_INTEGER,
  );
}

function createHistory(serverId: string): MetricPoint[] {
  const now = Math.floor(Date.now() / 1000);
  const start = now - (HISTORY_POINTS - 1) * HISTORY_STEP_SECONDS;
  const seed = seedOf(serverId);
  const totalMemory = ((seed % 5) + 4) * 1024 * 1024 * 1024;
  const diskRootTotal = ((seed % 4) + 48) * 1024 * 1024 * 1024;
  const diskDataTotal = ((seed % 6) + 96) * 1024 * 1024 * 1024;
  const points: MetricPoint[] = [];

  for (let index = 0; index < HISTORY_POINTS; index += 1) {
    const timestamp = start + index * HISTORY_STEP_SECONDS;
    const cpu = clamp(wave(index, seed, 24 + (seed % 20), 10, 3), 5, 92);
    const memoryPercent = clamp(wave(index, seed + 7, 42 + (seed % 18), 7, 4.4), 18, 88);
    const memoryUsed = totalMemory * (memoryPercent / 100);
    const cachedTarget = totalMemory * clamp(wave(index, seed + 9, 0.16, 0.05, 5.2), 0.06, 0.32);
    const memoryFree = Math.max(totalMemory - memoryUsed - cachedTarget, totalMemory * 0.04);
    const memoryCached = Math.max(totalMemory - memoryUsed - memoryFree, 0);
    const memoryAvailable = memoryFree + memoryCached;
    const rx = wave(index, seed + 17, 1_200_000 + (seed % 7) * 380_000, 420_000, 2.7);
    const tx = wave(index, seed + 23, 880_000 + (seed % 5) * 310_000, 360_000, 3.4);
    const overlayRx = wave(index, seed + 31, 420_000 + (seed % 6) * 120_000, 160_000, 2.9);
    const overlayTx = wave(index, seed + 41, 260_000 + (seed % 4) * 90_000, 120_000, 3.3);
    const diskRead = wave(index, seed + 47, 120_000 + (seed % 8) * 30_000, 44_000, 2.2);
    const diskWrite = wave(index, seed + 53, 180_000 + (seed % 8) * 36_000, 66_000, 2.5);
    const rootUsed = diskRootTotal * clamp(wave(index, seed + 59, 0.34, 0.08, 6), 0.12, 0.94);
    const dataUsed = diskDataTotal * clamp(wave(index, seed + 67, 0.52, 0.1, 5.5), 0.16, 0.96);

    points.push(
      metric(serverId, 'cpu_usage_percent', cpu, timestamp),
      metric(serverId, 'memory_used_percent', memoryPercent, timestamp),
      metric(serverId, 'memory_total_bytes', totalMemory, timestamp),
      metric(serverId, 'memory_used_bytes', memoryUsed, timestamp),
      metric(serverId, 'memory_free_bytes', memoryFree, timestamp),
      metric(serverId, 'memory_cached_bytes', memoryCached, timestamp),
      metric(serverId, 'memory_available_bytes', memoryAvailable, timestamp),
      metric(serverId, 'network_receive_bytes_total_rate', rx, timestamp, { interface: 'eth0' }),
      metric(serverId, 'network_transmit_bytes_total_rate', tx, timestamp, { interface: 'eth0' }),
      metric(serverId, 'network_receive_bytes_total_rate', overlayRx, timestamp, { interface: 'tailscale0' }),
      metric(serverId, 'network_transmit_bytes_total_rate', overlayTx, timestamp, { interface: 'tailscale0' }),
      metric(serverId, 'disk_total_bytes', diskRootTotal, timestamp, { mount: '/', device: 'vda' }),
      metric(serverId, 'disk_used_bytes', rootUsed, timestamp, { mount: '/', device: 'vda' }),
      metric(serverId, 'disk_total_bytes', diskDataTotal, timestamp, { mount: '/data', device: 'vdb' }),
      metric(serverId, 'disk_used_bytes', dataUsed, timestamp, { mount: '/data', device: 'vdb' }),
      metric(serverId, 'disk_read_bytes_total_rate', diskRead, timestamp, { device: 'vda' }),
      metric(serverId, 'disk_write_bytes_total_rate', diskWrite, timestamp, { device: 'vda' }),
    );
  }

  return points;
}

function metric(
  serverId: string,
  key: string,
  value: number,
  timestamp: number,
  labels: Record<string, string> = {},
): MetricPoint {
  return {
    server_id: serverId,
    key,
    value,
    metric_type: 'gauge',
    vantage_point: 'browser_demo',
    labels,
    timestamp,
  };
}

function matchesLabels(metricPoint: MetricPoint, labels?: Record<string, string>) {
  if (!labels) {
    return true;
  }

  return Object.entries(labels).every(([key, value]) => metricPoint.labels[key] === value);
}

function getLatestHistoryValue(history: MetricPoint[], key: string, labels?: Record<string, string>) {
  const series = history.filter((item) => item.key === key && matchesLabels(item, labels));
  return series[series.length - 1]?.value ?? 0;
}

function getCurrentMetrics(serverId: string): MetricPoint[] {
  const history = createHistory(serverId);
  const now = Math.floor(Date.now() / 1000);
  const seed = seedOf(serverId);
  const cpuBase = getLatestHistoryValue(history, 'cpu_usage_percent');

  return [
    metric(serverId, 'cpu_usage_percent', cpuBase, now),
    metric(serverId, 'cpu_core_usage_percent', clamp(cpuBase * 0.62, 0, 100), now, { core: '0' }),
    metric(serverId, 'cpu_core_usage_percent', clamp(cpuBase * 0.78, 0, 100), now, { core: '1' }),
    metric(serverId, 'cpu_core_usage_percent', clamp(cpuBase * 0.84, 0, 100), now, { core: '2' }),
    metric(serverId, 'cpu_core_usage_percent', clamp(cpuBase * 0.97, 0, 100), now, { core: '3' }),
    metric(serverId, 'cpu_core_usage_percent', clamp(cpuBase * 0.52, 0, 100), now, { core: '4' }),
    metric(serverId, 'cpu_core_usage_percent', clamp(cpuBase * 0.66, 0, 100), now, { core: '5' }),
    metric(serverId, 'memory_used_percent', getLatestHistoryValue(history, 'memory_used_percent'), now),
    metric(serverId, 'memory_total_bytes', getLatestHistoryValue(history, 'memory_total_bytes'), now),
    metric(serverId, 'memory_used_bytes', getLatestHistoryValue(history, 'memory_used_bytes'), now),
    metric(serverId, 'memory_free_bytes', getLatestHistoryValue(history, 'memory_free_bytes'), now),
    metric(serverId, 'memory_cached_bytes', getLatestHistoryValue(history, 'memory_cached_bytes'), now),
    metric(serverId, 'memory_available_bytes', getLatestHistoryValue(history, 'memory_available_bytes'), now),
    metric(serverId, 'disk_total_bytes', getLatestHistoryValue(history, 'disk_total_bytes', { mount: '/', device: 'vda' }), now, { mount: '/', device: 'vda' }),
    metric(serverId, 'disk_used_bytes', getLatestHistoryValue(history, 'disk_used_bytes', { mount: '/', device: 'vda' }), now, { mount: '/', device: 'vda' }),
    metric(serverId, 'disk_total_bytes', getLatestHistoryValue(history, 'disk_total_bytes', { mount: '/data', device: 'vdb' }), now, { mount: '/data', device: 'vdb' }),
    metric(serverId, 'disk_used_bytes', getLatestHistoryValue(history, 'disk_used_bytes', { mount: '/data', device: 'vdb' }), now, { mount: '/data', device: 'vdb' }),
    metric(serverId, 'network_receive_bytes_total_rate', getLatestHistoryValue(history, 'network_receive_bytes_total_rate', { interface: 'eth0' }), now, { interface: 'eth0' }),
    metric(serverId, 'network_transmit_bytes_total_rate', getLatestHistoryValue(history, 'network_transmit_bytes_total_rate', { interface: 'eth0' }), now, { interface: 'eth0' }),
    metric(serverId, 'network_receive_bytes_total_rate', getLatestHistoryValue(history, 'network_receive_bytes_total_rate', { interface: 'tailscale0' }), now, { interface: 'tailscale0' }),
    metric(serverId, 'network_transmit_bytes_total_rate', getLatestHistoryValue(history, 'network_transmit_bytes_total_rate', { interface: 'tailscale0' }), now, { interface: 'tailscale0' }),
    metric(serverId, 'disk_read_bytes_total_rate', getLatestHistoryValue(history, 'disk_read_bytes_total_rate', { device: 'vda' }) + (seed % 3) * 12_000, now, { device: 'vda' }),
    metric(serverId, 'disk_write_bytes_total_rate', getLatestHistoryValue(history, 'disk_write_bytes_total_rate', { device: 'vda' }) + (seed % 5) * 17_000, now, { device: 'vda' }),
  ];
}

export async function demoListServers() {
  return readState().servers;
}

export async function demoAddServer(input: ServerFormData) {
  const state = readState();
  const server = withTimestamps({
    id: `browser-demo-${Date.now()}`,
    name: input.name,
    host: input.host,
    port: input.port,
    adapter_type: input.adapter_type,
    access_method: input.access_method,
    polling_interval_sec: input.polling_interval_sec,
    enabled: true,
    auth_token: input.auth_token ?? null,
    auth_type: input.auth_type ?? 'none',
    ssh_key_path: input.ssh_key ?? null,
    ssh_passphrase: input.ssh_passphrase ?? null,
    password: input.password ?? null,
    created_at: 0,
    updated_at: 0,
  });
  writeState({ ...state, servers: [...state.servers, server] });
  return server;
}

export async function demoRemoveServer(id: string) {
  const state = readState();
  writeState({ ...state, servers: state.servers.filter((server) => server.id !== id) });
}

export async function demoGetMetrics(serverId: string) {
  return getCurrentMetrics(serverId);
}

export async function demoGetMetricHistory(
  serverId: string,
  key: string,
  from: number,
  to: number,
  options?: BrowserMetricHistoryOptions,
): Promise<MetricHistoryResponse> {
  const history = createHistory(serverId)
    .filter((item) => item.key === key)
    .filter((item) => item.timestamp >= from && item.timestamp <= to)
    .filter((item) => matchesLabels(item, options?.labels));

  return {
    kind: 'raw',
    resolution: 'raw',
    points: history,
  };
}

export async function demoGetSettings() {
  return readState().settings;
}

export async function demoUpdateSettings(settings: BrowserAppSettings) {
  const state = readState();
  writeState({ ...state, settings });
  return settings;
}
