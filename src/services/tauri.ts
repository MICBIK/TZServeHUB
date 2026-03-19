import { invoke } from '@tauri-apps/api/core';
import type { ServerConfig, ServerFormData } from '../types/server';
import type { MetricPoint } from '../types/metric';

// Server commands
export async function listServers(): Promise<ServerConfig[]> {
  return invoke('list_servers');
}

export async function addServer(name: string, host: string, port: number): Promise<ServerConfig> {
  return invoke('add_server', { name, host, port });
}

export async function removeServer(id: string): Promise<void> {
  return invoke('remove_server', { id });
}

// Metric commands
export async function getMetrics(serverId: string): Promise<MetricPoint[]> {
  return invoke('get_metrics', { serverId });
}

export async function getMetricHistory(
  serverId: string,
  key: string,
  from: number,
  to: number,
): Promise<MetricPoint[]> {
  return invoke('get_metric_history', { serverId, key, from, to });
}

// Settings commands
export interface AppSettings {
  default_polling_interval_sec: number;
  data_retention_days: number;
  theme: string;
  language: string;
}

export async function getSettings(): Promise<AppSettings> {
  return invoke('get_settings');
}

export async function updateSettings(settings: AppSettings): Promise<AppSettings> {
  return invoke('update_settings', { settings });
}
