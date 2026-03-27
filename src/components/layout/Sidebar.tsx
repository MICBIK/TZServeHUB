import React, { useEffect } from 'react';
import { NavLink } from 'react-router-dom';
import ServerCard from '../server/ServerCard';
import { useServerStore } from '../../stores/serverStore';
import { useUiCopy } from '../../hooks/useUiCopy';
import type { UiCopyKey } from '../../lib/uiCopy';

type NavIconKind = 'dashboard' | 'settings';

const navItems: Array<{ path: string; label: UiCopyKey; icon: NavIconKind }> = [
  { path: '/', label: 'nav_dashboard', icon: 'dashboard' },
  { path: '/settings', label: 'nav_settings', icon: 'settings' },
];

interface SidebarProps {
  bootstrapped: boolean;
  mode?: 'desktop' | 'drawer';
  onClose?: () => void;
}

function NavIcon({ kind }: { kind: NavIconKind }) {
  const commonProps = {
    fill: 'none',
    stroke: 'currentColor',
    strokeWidth: 1.8,
    strokeLinecap: 'round' as const,
    strokeLinejoin: 'round' as const,
  };

  if (kind === 'settings') {
    return (
      <svg viewBox="0 0 24 24" aria-hidden="true">
        <path d="M12 8.5A3.5 3.5 0 1 0 12 15.5A3.5 3.5 0 1 0 12 8.5z" {...commonProps} />
        <path d="M19 12a7 7 0 0 0-.1-1l2-1.5-2-3.5-2.4 1a7.8 7.8 0 0 0-1.7-1l-.3-2.6h-4l-.3 2.6a7.8 7.8 0 0 0-1.7 1l-2.4-1-2 3.5 2 1.5a7 7 0 0 0 0 2l-2 1.5 2 3.5 2.4-1a7.8 7.8 0 0 0 1.7 1l.3 2.6h4l.3-2.6a7.8 7.8 0 0 0 1.7-1l2.4 1 2-3.5-2-1.5c.1-.3.1-.7.1-1Z" {...commonProps} />
      </svg>
    );
  }

  return (
    <svg viewBox="0 0 24 24" aria-hidden="true">
      <rect x="4" y="4" width="16" height="5" rx="1.5" {...commonProps} />
      <rect x="4" y="11.5" width="16" height="8.5" rx="1.5" {...commonProps} />
      <path d="M8 15.5h8M8 18h5" {...commonProps} />
    </svg>
  );
}

const Sidebar: React.FC<SidebarProps> = ({ bootstrapped, mode = 'desktop', onClose }) => {
  const servers = useServerStore((state) => state.servers);
  const activeServerId = useServerStore((state) => state.activeServerId);
  const setActiveServer = useServerStore((state) => state.setActiveServer);
  const refreshServerStatus = useServerStore((state) => state.refreshServerStatus);
  const { t } = useUiCopy();
  const onlineTargets = servers.filter((server) => server.status === 'online').length;
  const isDrawer = mode === 'drawer';

  useEffect(() => {
    if (!bootstrapped) return;
    const handle = setInterval(() => {
      refreshServerStatus();
    }, 10_000);
    return () => clearInterval(handle);
  }, [bootstrapped, refreshServerStatus]);

  return (
    <aside
      className={
        isDrawer
          ? 'shell-sidebar-drawer flex h-full w-[min(320px,88vw)] shrink-0 flex-col'
          : 'mr-2 hidden w-[196px] shrink-0 flex-col min-[920px]:flex min-[1180px]:w-[210px] min-[1420px]:w-[228px]'
      }
    >
      <div className="glass-panel shell-sidebar relative flex h-full flex-col overflow-hidden rounded-[24px] border px-3.5 py-3.5 xl:px-3.5 xl:py-4">
        <div className="sidebar-topline pointer-events-none absolute inset-x-6 top-0 h-px" />
        <div className="flex items-start justify-between gap-4">
          <div>
            <p className="shell-kicker">ServerHUB</p>
            <h1 className="shell-sidebar-title mt-2">{t('shell_title')}</h1>
            <p className="shell-muted mt-2 text-sm leading-6">{t('shell_subtitle')}</p>
          </div>
          {isDrawer ? (
            <button
              type="button"
              className="shell-close-button"
              aria-label="Close navigation"
              onClick={onClose}
            >
              ×
            </button>
          ) : (
            <div className={`status-beacon ${bootstrapped ? 'is-online' : ''}`}>
              <span />
            </div>
          )}
        </div>

        <nav className="mt-4 space-y-1.5">
          {navItems.map((item) => (
            <NavLink
              key={item.path}
              to={item.path}
              className={({ isActive }) => `nav-chip ${isActive ? 'nav-chip-active' : 'nav-chip-idle'}`}
              title={t(item.label)}
              onClick={onClose}
            >
              <span className="nav-chip-icon">
                <NavIcon kind={item.icon} />
              </span>
              <span className="nav-chip-copy">
                <span className="truncate">{t(item.label)}</span>
              </span>
            </NavLink>
          ))}
        </nav>

        <div className="ops-sidebar-summary mt-4">
          <div className="ops-sidebar-stat">
            <span className="ops-sidebar-stat-label">{t('targets')}</span>
            <strong>{servers.length}</strong>
          </div>
          <div className="ops-sidebar-stat">
            <span className="ops-sidebar-stat-label">{t('live')}</span>
            <strong>{onlineTargets}</strong>
          </div>
        </div>

        <div className="sidebar-targets-panel mt-3.5">
          <div className="sidebar-targets-head">
            <p className="panel-label">{t('active_target')}</p>
            <span className="shell-count-chip">{servers.length}</span>
          </div>

          <div className="sidebar-targets-list mt-3">
            {servers.length > 0 ? (
              servers.map((server) => (
                <ServerCard
                  key={server.id}
                  server={server}
                  isActive={server.id === activeServerId}
                  compact
                  onClick={() => {
                    setActiveServer(server.id);
                    onClose?.();
                  }}
                />
              ))
            ) : (
              <div className="rounded-[18px] border border-dashed border-[var(--surface-border)] bg-[var(--surface-muted)] p-4 text-sm leading-6 text-[var(--text-secondary)]">
                {t('add_server')}
              </div>
            )}
          </div>
        </div>
      </div>
    </aside>
  );
};

export default Sidebar;
