import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { MemoryRouter } from 'react-router-dom';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import SettingsPage from './SettingsPage';
import { useServerStore } from '../stores/serverStore';
import { useSettingsStore } from '../stores/settingsStore';
import { useMetricsStore } from '../stores/metricsStore';
import type { ServerConfig } from '../types/server';

const knownHosts = [
  {
    host: 'vps.example.test',
    port: 22,
    fingerprint: '0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef',
    algorithm: 'ssh-ed25519',
    first_seen: 1_700_000_000,
    last_seen: 1_700_000_100,
  },
];

const { listKnownHostsMock, removeKnownHostMock } = vi.hoisted(() => ({
  listKnownHostsMock: vi.fn(),
  removeKnownHostMock: vi.fn(),
}));

vi.mock('../services/tauri', async () => {
  const actual = await vi.importActual<typeof import('../services/tauri')>('../services/tauri');
  return {
    ...actual,
    listKnownHosts: listKnownHostsMock,
    removeKnownHost: removeKnownHostMock,
  };
});

function makeServer(overrides: Partial<ServerConfig> = {}): ServerConfig {
  const now = Math.floor(Date.now() / 1000);
  return {
    id: 'srv-1',
    name: 'Test Server',
    host: '192.0.2.10',
    port: 9100,
    adapter_type: 'go_agent',
    access_method: 'private',
    polling_interval_sec: 10,
    enabled: true,
    auth_token: null,
    auth_type: 'token',
    ssh_key_path: null,
    ssh_passphrase: null,
    password: null,
    status: 'online',
    last_seen_at: now,
    last_error: null,
    created_at: now,
    updated_at: now,
    ...overrides,
  };
}

function resetStores() {
  useServerStore.setState({
    servers: [makeServer()],
    activeServerId: 'srv-1',
    loading: false,
    hydrated: true,
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
    hydrated: true,
    error: null,
  });
  useMetricsStore.setState({ current: {}, loading: false, error: null });
}

function renderPage() {
  return render(
    <MemoryRouter>
      <SettingsPage />
    </MemoryRouter>,
  );
}

describe('SettingsPage known hosts', () => {
  beforeEach(() => {
    listKnownHostsMock.mockResolvedValue([...knownHosts]);
    removeKnownHostMock.mockResolvedValue(undefined);
    resetStores();
  });

  it('should_render_known_hosts_section (HOST-009)', async () => {
    renderPage();

    expect(await screen.findByTestId('settings-known-hosts')).toBeInTheDocument();
    expect(screen.getByText('vps.example.test:22')).toBeInTheDocument();
    expect(screen.getByText(/0123456789abcdef/)).toBeInTheDocument();
  });

  it('should_remove_known_host_via_button (HOST-009)', async () => {
    renderPage();

    const button = await screen.findByTestId('known-host-remove-vps.example.test-22');
    fireEvent.click(button);

    await waitFor(() => {
      expect(removeKnownHostMock).toHaveBeenCalledWith('vps.example.test', 22);
    });
    await waitFor(() => {
      expect(listKnownHostsMock).toHaveBeenCalledTimes(2);
    });
  });
});
