import React from 'react';
import { useServerStore } from '../../stores/serverStore';
import { usePolling } from '../../hooks/usePolling';
import { useMetricsStore } from '../../stores/metricsStore';

const DashboardPage: React.FC = () => {
  const servers = useServerStore((s) => s.servers);
  const activeServerId = useServerStore((s) => s.activeServerId);
  const metrics = useMetricsStore((s) => s.current);

  usePolling(activeServerId);

  return (
    <div className="p-6">
      <h1 className="text-2xl font-bold mb-6">Dashboard</h1>
      {servers.length === 0 ? (
        <div className="text-center text-gray-500 mt-20">
          <p className="text-lg">暂无服务器</p>
          <p className="mt-2">请在设置中添加服务器开始监控</p>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {/* TODO: server overview cards with sparklines */}
          <p className="text-gray-400">服务器概览卡片 - 待实现</p>
        </div>
      )}
    </div>
  );
};

export default DashboardPage;
