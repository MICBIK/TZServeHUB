import type { ServerConfig, ServerStatus } from '../../types/server';
import type { UiCopyKey } from '../../lib/uiCopy';
import { useUiCopy } from '../../hooks/useUiCopy';
import { getAdapterLabelKey } from '../../lib/serverLabels';

interface ServerCardProps {
  server: ServerConfig;
  isActive: boolean;
  onClick: () => void;
  compact?: boolean;
}

function statusDotClass(status: ServerStatus): string {
  switch (status) {
    case 'online':
      return 'is-online';
    case 'error':
      return 'is-error';
    default:
      return '';
  }
}

function statusLabelKey(status: ServerStatus): UiCopyKey {
  switch (status) {
    case 'online':
      return 'status_online';
    case 'error':
      return 'status_error_label';
    default:
      return 'status_unknown';
  }
}

export default function ServerCard({ server, isActive, onClick, compact = false }: ServerCardProps) {
  const { t } = useUiCopy();

  return (
    <button
      type="button"
      onClick={onClick}
      className={`server-card group w-full rounded-[18px] border text-left transition duration-300 ${
        compact ? 'is-compact px-3 py-2.5' : 'px-3.5 py-3'
      } ${
        isActive ? 'is-active' : ''
      }`}
    >
      <div className={`${compact ? 'mb-2' : 'mb-3'} flex items-start justify-between gap-3`}>
        <div className="min-w-0">
          {compact ? null : <p className="panel-label">{t('server_target_label')}</p>}
          <h3 className={`server-card-title truncate ${compact ? '' : 'mt-1'}`}>{server.name}</h3>
        </div>
        <span
          className={`server-card-dot ${statusDotClass(server.status)}`}
          title={server.last_error ?? undefined}
        />
      </div>
      <p className={`server-card-endpoint truncate ${compact ? 'text-[0.76rem]' : 'text-sm'}`}>
        {server.host}:{server.port}
      </p>
      <div className={`server-card-meta ${compact ? 'mt-2' : 'mt-3'}`}>
        <span className="truncate">{t(getAdapterLabelKey(server.adapter_type))}</span>
        <span className={`server-card-status ${isActive ? 'is-active' : ''} ${server.status === 'error' ? 'is-error' : ''}`}>
          {isActive ? t('server_active') : t(statusLabelKey(server.status))}
        </span>
      </div>
    </button>
  );
}
