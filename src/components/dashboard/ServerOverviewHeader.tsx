import { useServerStore } from '../../stores/serverStore';
import { useUiCopy } from '../../hooks/useUiCopy';

export default function ServerOverviewHeader() {
  const { t } = useUiCopy();
  const servers = useServerStore((state) => state.servers);
  const activeServerId = useServerStore((state) => state.activeServerId);

  const totalServers = servers.length;
  const onlineServers = servers.filter((s) => s.enabled).length;
  const offlineServers = totalServers - onlineServers;

  const activeServer = servers.find((s) => s.id === activeServerId);

  return (
    <div className="server-overview-header">
      <div className="server-counts">
        <div className="count-item">
          <span className="count-label">{t('server_count_total')}</span>
          <span className="count-value">{totalServers}</span>
        </div>
        <div className="count-item">
          <span className="count-label">{t('server_count_online')}</span>
          <span className="count-value count-online">{onlineServers}</span>
        </div>
        <div className="count-item">
          <span className="count-label">{t('server_count_offline')}</span>
          <span className="count-value count-offline">{offlineServers}</span>
        </div>
      </div>

      {activeServer && (
        <div className="active-server-info">
          <span className="active-label">{t('active_server_label')}</span>
          <div className="active-server-details">
            <span className="server-name">{activeServer.name}</span>
            <span className="server-endpoint">
              {activeServer.host}:{activeServer.port}
            </span>
          </div>
        </div>
      )}
    </div>
  );
}
