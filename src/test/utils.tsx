import { render, type RenderResult } from '@testing-library/react';
import { MemoryRouter } from 'react-router-dom';
import type { ReactElement } from 'react';
import { useServerStore } from '../stores/serverStore';
import { useSettingsStore } from '../stores/settingsStore';
import { useMetricsStore } from '../stores/metricsStore';
import type { ServerConfig } from '../types/server';
import type { MetricPoint } from '../types/metric';

export function renderWithRouter(
  ui: ReactElement,
  initialPath = '/',
): RenderResult {
  return render(<MemoryRouter initialEntries={[initialPath]}>{ui}</MemoryRouter>);
}

export function resetStores() {
  useServerStore.setState({
    servers: [],
    activeServerId: null,
    loading: false,
    hydrated: false,
    error: null,
  });
  useSettingsStore.setState({
    settings: {
      default_polling_interval_sec: 10,
      data_retention_days: 7,
      theme: 'dark',
      language: 'zh-CN',
    },
    loading: false,
    hydrated: false,
    error: null,
  });
  useMetricsStore.setState({ current: {}, loading: false, error: null });
}

export function mockServerStore(partial: Partial<ReturnType<typeof useServerStore.getState>>) {
  useServerStore.setState({ hydrated: true, ...partial });
}

export function mockSettingsHydrated() {
  useSettingsStore.setState({ hydrated: true });
}

export function makeServer(overrides: Partial<ServerConfig> = {}): ServerConfig {
  return {
    id: 'srv-1',
    name: 'Test Server',
    host: '192.0.2.10',
    port: 9100,
    adapter_type: 'go_agent' as ServerConfig['adapter_type'],
    access_method: 'private' as ServerConfig['access_method'],
    polling_interval_sec: 10,
    enabled: true,
    auth_token: null,
    auth_type: 'token' as ServerConfig['auth_type'],
    ssh_key_path: null,
    ssh_passphrase: null,
    password: null,
    status: 'online',
    last_seen_at: Math.floor(Date.now() / 1000),
    last_error: null,
    created_at: Math.floor(Date.now() / 1000),
    updated_at: Math.floor(Date.now() / 1000),
    ...overrides,
  };
}

export function setMetricsFor(serverId: string, points: MetricPoint[]) {
  useMetricsStore.setState((state) => ({
    current: { ...state.current, [serverId]: points },
  }));
}
