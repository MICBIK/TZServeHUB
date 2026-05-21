import { invoke } from '@tauri-apps/api/core';
import {
  demoAddServer,
  demoGetMetricHistory,
  demoGetMetrics,
  demoGetServerHealthSummary,
  demoGetLatestProbeResults,
  demoGetProbeHistory,
  demoGetSettings,
  demoListServers,
  demoRemoveServer,
  demoUpdateSettings,
} from '../lib/browserDemo';
import type { AlertEvent, AlertRule } from '../types/alert';
import type { HealthSummary, ServerConfig, ServerFormData } from '../types/server';
import type { MetricHistoryResponse, MetricPoint } from '../types/metric';
import type { PingProbeResult, TcpProbeResult, DnsProbeResult } from '../types/probe';

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

export async function getServerHealthSummary(): Promise<HealthSummary> {
  if (!hasTauriInvoke()) {
    return demoGetServerHealthSummary();
  }

  return invoke('get_server_health_summary');
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

export interface KnownHost {
  host: string;
  port: number;
  fingerprint: string;
  algorithm: string;
  first_seen: number;
  last_seen: number;
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

export async function listKnownHosts(): Promise<KnownHost[]> {
  if (!hasTauriInvoke()) {
    return [];
  }

  return invoke('list_known_hosts');
}

export async function removeKnownHost(host: string, port: number): Promise<void> {
  if (!hasTauriInvoke()) {
    return;
  }

  return invoke('remove_known_host', { host, port });
}

// Probe commands (on-demand)
export async function runPingProbe(
  host: string,
  count?: number,
): Promise<PingProbeResult> {
  if (!hasTauriInvoke()) {
    return {
      avg_rtt_ms: 12.4 + Math.random() * 8,
      loss_rate: 0,
      packets_sent: count ?? 4,
      packets_lost: 0,
    };
  }

  return invoke('run_ping_probe', { host, count: count ?? null });
}

export async function runTcpProbe(
  host: string,
  port: number,
  timeoutMs?: number,
): Promise<TcpProbeResult> {
  if (!hasTauriInvoke()) {
    return {
      reachable: true,
      latency_ms: 3.2 + Math.random() * 5,
    };
  }

  return invoke('run_tcp_probe', { host, port, timeoutMs: timeoutMs ?? null });
}

export async function runDnsProbe(
  domain: string,
  dnsServer?: string,
  timeoutMs?: number,
): Promise<DnsProbeResult> {
  if (!hasTauriInvoke()) {
    return {
      resolved: true,
      latency_ms: 18.6 + Math.random() * 12,
    };
  }

  return invoke('run_dns_probe', {
    domain,
    dnsServer: dnsServer ?? null,
    timeoutMs: timeoutMs ?? null,
  });
}

// Probe history commands (from auto-scheduler)
export interface ProbeResultRow {
  id: number;
  server_id: string;
  probe_type: string;
  target: string;
  success: boolean;
  latency_ms: number | null;
  loss_rate: number | null;
  error_message: string | null;
  timestamp: number;
}

export async function getProbeHistory(
  serverId: string,
  probeType: string,
  from: number,
  to: number,
): Promise<ProbeResultRow[]> {
  if (!hasTauriInvoke()) {
    return demoGetProbeHistory(serverId, probeType, from, to);
  }

  return invoke('get_probe_history', {
    serverId,
    probeType,
    from,
    to,
  });
}

export async function getLatestProbeResults(
  serverId: string,
): Promise<ProbeResultRow[]> {
  if (!hasTauriInvoke()) {
    return demoGetLatestProbeResults(serverId);
  }

  return invoke('get_latest_probe_results', { serverId });
}

// Alert commands
export async function listAlertRules(): Promise<AlertRule[]> {
  if (!hasTauriInvoke()) {
    return [];
  }

  return invoke('list_alert_rules');
}

export async function addAlertRule(input: {
  server_id: string;
  name: string;
  metric_key: string;
  condition: string;
  threshold: number;
  duration_sec: number;
}): Promise<AlertRule> {
  if (!hasTauriInvoke()) {
    const now = Math.floor(Date.now() / 1000);
    return {
      id: `demo-rule-${now}`,
      server_id: input.server_id,
      name: input.name,
      metric_key: input.metric_key,
      condition: input.condition as AlertRule['condition'],
      threshold: input.threshold,
      duration_sec: input.duration_sec,
      enabled: true,
      created_at: now,
    };
  }

  return invoke('add_alert_rule', {
    serverId: input.server_id,
    name: input.name,
    metricKey: input.metric_key,
    condition: input.condition,
    threshold: input.threshold,
    durationSec: input.duration_sec,
  });
}

export async function removeAlertRule(id: string): Promise<void> {
  if (!hasTauriInvoke()) {
    return;
  }

  return invoke('remove_alert_rule', { id });
}

export async function listAlertEvents(
  serverId?: string,
  limit?: number,
): Promise<AlertEvent[]> {
  if (!hasTauriInvoke()) {
    return [];
  }

  return invoke('list_alert_events', {
    serverId: serverId ?? null,
    limit: limit ?? null,
  });
}
