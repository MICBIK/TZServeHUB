import React, { useState } from 'react';
import { Outlet, useLocation } from 'react-router-dom';
import Sidebar from './Sidebar';
import { usePolling } from '../../hooks/usePolling';
import { useServerStore } from '../../stores/serverStore';
import { useSettingsStore } from '../../stores/settingsStore';
import { useUiCopy } from '../../hooks/useUiCopy';
import type { UiCopyKey } from '../../lib/uiCopy';
import type { ThemeMode } from '../../services/tauri';

interface AppLayoutProps {
  bootstrapped: boolean;
}

function ThemeIcon({ theme }: { theme: ThemeMode }) {
  if (theme === 'dark') {
    return (
      <svg viewBox="0 0 24 24" aria-hidden="true">
        <path
          d="M20.5 14.5A8.5 8.5 0 1 1 10 4a7 7 0 0 0 10.5 10.5Z"
          fill="none"
          stroke="currentColor"
          strokeWidth="1.8"
          strokeLinecap="round"
          strokeLinejoin="round"
        />
      </svg>
    );
  }

  return (
    <svg viewBox="0 0 24 24" aria-hidden="true">
      <circle
        cx="12"
        cy="12"
        r="4"
        fill="none"
        stroke="currentColor"
        strokeWidth="1.8"
      />
      <path
        d="M12 2.5v2.4M12 19.1v2.4M4.9 4.9l1.7 1.7M17.4 17.4l1.7 1.7M2.5 12h2.4M19.1 12h2.4M4.9 19.1l1.7-1.7M17.4 6.6l1.7-1.7"
        fill="none"
        stroke="currentColor"
        strokeWidth="1.8"
        strokeLinecap="round"
      />
    </svg>
  );
}

const AppLayout: React.FC<AppLayoutProps> = ({ bootstrapped }) => {
  const location = useLocation();
  const servers = useServerStore((state) => state.servers);
  const settings = useSettingsStore((state) => state.settings);
  const updateSettings = useSettingsStore((state) => state.updateSettings);
  const { t } = useUiCopy();
  const [sidebarOpen, setSidebarOpen] = useState(false);
  const isSettingsPage = location.pathname === '/settings';
  const currentPageKey: UiCopyKey = isSettingsPage ? 'nav_settings' : 'nav_dashboard';

  usePolling(servers);

  async function applyTheme(theme: ThemeMode) {
    if (settings.theme === theme) {
      return;
    }

    await updateSettings({ ...settings, theme });
  }

  return (
    <div className="shell-root relative h-screen overflow-hidden">
      <div className="shell-grid" />
      <div className="pointer-events-none absolute inset-x-0 top-0 flex justify-center opacity-80">
        <svg className="h-56 w-[980px]" viewBox="0 0 980 320" fill="none">
          <path
            d="M24 181C170 117 260 63 432 70C630 79 699 241 956 134"
            stroke="url(#radialStroke)"
            strokeWidth="2"
            strokeLinecap="round"
            className="flow-line subtle"
          />
          <circle cx="118" cy="146" r="6" className="pulse-node" fill="var(--accent-strong)" />
          <circle cx="514" cy="83" r="5" className="pulse-node pulse-delay" fill="var(--accent-soft)" />
          <circle cx="850" cy="156" r="7" className="pulse-node pulse-late" fill="var(--accent-warm)" />
          <defs>
            <linearGradient id="radialStroke" x1="24" y1="90" x2="956" y2="180" gradientUnits="userSpaceOnUse">
              <stop stopColor="var(--accent-strong)" />
              <stop offset="0.5" stopColor="var(--accent-soft)" />
              <stop offset="1" stopColor="var(--accent-warm)" />
            </linearGradient>
          </defs>
        </svg>
      </div>
      <div className="shell-workspace relative flex h-screen min-h-0">
        <Sidebar bootstrapped={bootstrapped} mode="desktop" />
        {sidebarOpen ? (
          <div className="shell-drawer-backdrop shell-drawer-visible">
            <div className="shell-drawer-layer" onClick={() => setSidebarOpen(false)} />
            <Sidebar bootstrapped={bootstrapped} mode="drawer" onClose={() => setSidebarOpen(false)} />
          </div>
        ) : null}
        <main className="shell-main relative flex h-full min-h-0 flex-1 flex-col overflow-hidden rounded-[28px] border">
          <div className="shell-main-outline pointer-events-none absolute inset-0 rounded-[28px]" />
          <header className="shell-toolbar">
            <div className="shell-toolbar-context">
              <button
                type="button"
                className="shell-menu-button"
                aria-label="Open navigation"
                onClick={() => setSidebarOpen(true)}
              >
                <span />
                <span />
                <span />
              </button>
              <div className="min-w-0">
                <p className="shell-toolbar-label">{t('shell_title')}</p>
                <span className="shell-toolbar-inline-title">{t(currentPageKey)}</span>
              </div>
            </div>
            <div className={`shell-toolbar-actions ${isSettingsPage ? 'is-settings-page' : ''}`}>
              <div
                className={`shell-theme-toggle ${isSettingsPage ? 'is-compact' : ''}`}
                role="group"
                aria-label={t('settings_theme_label')}
              >
                <button
                  type="button"
                  className={`shell-theme-button ${settings.theme === 'dark' ? 'is-active' : ''}`}
                  onClick={() => applyTheme('dark')}
                  aria-label={t('theme_dark')}
                  aria-pressed={settings.theme === 'dark'}
                  title={t('theme_dark')}
                >
                  <ThemeIcon theme="dark" />
                </button>
                <button
                  type="button"
                  className={`shell-theme-button ${settings.theme === 'light' ? 'is-active' : ''}`}
                  onClick={() => applyTheme('light')}
                  aria-label={t('theme_light')}
                  aria-pressed={settings.theme === 'light'}
                  title={t('theme_light')}
                >
                  <ThemeIcon theme="light" />
                </button>
              </div>
            </div>
          </header>
          <div className="page-reveal relative mt-2 flex-1 min-h-0">
            <Outlet />
          </div>
        </main>
      </div>
    </div>
  );
};

export default AppLayout;
