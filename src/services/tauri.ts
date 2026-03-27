import { invoke } from '@tauri-apps/api/core';
import {
  demoAddServer,
  demoGetMetricHistory,
  demoGetMetrics,
  demoGetSettings,
  demoListServers,
  demoRemoveServer,
  demoUpdateSettings,
} from '../lib/browserDemo';
import type { ServerConfig, ServerFormData } from '../types/server';
import type { MetricHistoryResponse, MetricPoint } from '../types/metric';

export type ThemeMode = 'dark' | 'light';
export type LanguageMode = 'zh-CN' | 'en-US';

export interface MetricHistoryOptions {
  labels?: Record<string, string>;
  resolution?: 'raw' | '1m' | '15m';
}

function hasTauriInvoke() {
  return typeof window !== 'undefined'
    && typeof (window as Window & { __TAURI_INTERNALS__?: { invoke?: unknown } }).__TAURI_INTERNALS__?.invoke === 'function';
}

// Server commands
export async function listServers(): Promise<ServerConfig[]> {
  if (!hasTauriInvoke()) {
    return demoListServers();
  }

  return invoke('list_servers');
}

export async function addServer(input: ServerFormData): Promise<ServerConfig> {
  if (!hasTauriInvoke()) {
    return demoAddServer(input);
  }

  return invoke('add_server', {
    name: input.name,
    host: input.host,
    port: input.port,
    adapter_type: input.adapter_type,
    access_method: input.access_method,
    polling_interval_sec: input.polling_interval_sec,
    auth_token: input.auth_token ?? null,
    auth_type: input.auth_type ?? null,
    ssh_key_path: input.ssh_key ?? null,
    ssh_passphrase: input.ssh_passphrase ?? null,
    password: input.password ?? null,
  });
}

export async function removeServer(id: string): Promise<void> {
  if (!hasTauriInvoke()) {
    return demoRemoveServer(id);
  }

  return invoke('remove_server', { id });
}

// Metric commands
export async function getMetrics(serverId: string): Promise<MetricPoint[]> {
  if (!hasTauriInvoke()) {
    return demoGetMetrics(serverId);
  }

  return invoke('get_metrics', { serverId });
}

export async function getMetricHistory(
  serverId: string,
  key: string,
  from: number,
  to: number,
  options?: MetricHistoryOptions,
): Promise<MetricHistoryResponse> {
  if (!hasTauriInvoke()) {
    return demoGetMetricHistory(serverId, key, from, to, options);
  }

  return invoke('get_metric_history', {
    serverId,
    key,
    from,
    to,
    labels: options?.labels,
    resolution: options?.resolution,
  });
}

// Settings commands
export interface AppSettings {
  default_polling_interval_sec: number;
  data_retention_days: number;
  theme: ThemeMode;
  language: LanguageMode;
}

export async function getSettings(): Promise<AppSettings> {
  if (!hasTauriInvoke()) {
    return demoGetSettings();
  }

  return invoke('get_settings');
}

export async function updateSettings(settings: AppSettings): Promise<AppSettings> {
  if (!hasTauriInvoke()) {
    return demoUpdateSettings(settings);
  }

  return invoke('update_settings', { settings });
}
