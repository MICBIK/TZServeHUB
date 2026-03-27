import type { ServerConfig } from '../../types/server';
import type { MetricPoint } from '../../types/metric';
import { getCpuMetrics, getDiskUsage, getMemoryMetric, pickMetrics } from '../../lib/metricSelectors';

interface ServerCardProps {
  server: ServerConfig;
  metrics: MetricPoint[];
  isOnline: boolean;
}

export default function ServerCard({ server, metrics, isOnline }: ServerCardProps) {
  const formatBytes = (bytes: number): string => {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${(bytes / Math.pow(k, i)).toFixed(1)} ${sizes[i]}`;
  };

  const formatSpeed = (bytesPerSec: number): string => {
    return `${formatBytes(bytesPerSec)}/s`;
  };

  const getDynamicColor = (percent: number): string => {
    if (percent < 50) return '#34C759'; // green
    if (percent < 80) return '#FFD60A'; // yellow
    return '#FF453A'; // red
  };

  const cpu = getCpuMetrics(metrics);
  const memory = getMemoryMetric(metrics);
  const disks = getDiskUsage(metrics);

  const cpuPercent = cpu.total?.value ?? 0;
  const memoryPercent = memory?.value ?? 0;

  const totalRx = pickMetrics(metrics, 'network_receive_bytes_total_rate')
    .reduce((sum, m) => sum + m.value, 0);
  const totalTx = pickMetrics(metrics, 'network_transmit_bytes_total_rate')
    .reduce((sum, m) => sum + m.value, 0);

  const diskUsed = disks[0]?.used ?? 0;
  const diskTotal = disks[0]?.total ?? 0;

  const createRing = (percent: number) => {
    const radius = 45;
    const circumference = 2 * Math.PI * radius;
    const offset = circumference - (percent / 100) * circumference;
    const color = getDynamicColor(percent);
    return { circumference, offset, color, radius };
  };

  const cpuRing = createRing(cpuPercent);
  const memoryRing = createRing(memoryPercent);

  return (
    <div className="glass-panel rounded-2xl p-4 hover:shadow-lg transition-all duration-200 hover:-translate-y-0.5 overflow-hidden">
      {/* Header */}
      <div className="flex items-center justify-between mb-3">
        <div className="min-w-0 flex-1">
          <div className="flex items-center gap-2">
            <div
              className={`w-2 h-2 rounded-full ${
                isOnline ? 'bg-green-500' : 'bg-red-500'
              }`}
            />
            <h3 className="font-semibold text-base truncate">{server.name}</h3>
          </div>
          <p className="text-xs text-gray-500 dark:text-gray-400 truncate mt-0.5">
            {server.host}:{server.port}
          </p>
        </div>
      </div>

      {/* Rings */}
      <div className="flex justify-center items-center gap-6 mb-4">
        {/* CPU Ring */}
        <div className="flex flex-col items-center">
          <svg width="110" height="110" className="transform -rotate-90">
            <circle
              cx="65"
              cy="65"
              r={cpuRing.radius}
              fill="none"
              stroke={`${cpuRing.color}1A`}
              strokeWidth="10"
            />
            <circle
              cx="65"
              cy="65"
              r={cpuRing.radius}
              fill="none"
              stroke={cpuRing.color}
              strokeWidth="10"
              strokeDasharray={cpuRing.circumference}
              strokeDashoffset={cpuRing.offset}
              strokeLinecap="round"
              className="transition-all duration-500"
            />
            <text
              x="65"
              y="65"
              textAnchor="middle"
              dy="0.3em"
              className="text-2xl font-semibold fill-current transform rotate-90"
              style={{ transformOrigin: '65px 65px' }}
            >
              {cpuPercent.toFixed(0)}%
            </text>
          </svg>
          <span className="text-sm text-gray-600 dark:text-gray-400 mt-2 font-medium">CPU</span>
        </div>

        {/* Memory Ring */}
        <div className="flex flex-col items-center">
          <svg width="130" height="130" className="transform -rotate-90">
            <circle
              cx="65"
              cy="65"
              r={memoryRing.radius}
              fill="none"
              stroke={`${memoryRing.color}1A`}
              strokeWidth="10"
            />
            <circle
              cx="65"
              cy="65"
              r={memoryRing.radius}
              fill="none"
              stroke={memoryRing.color}
              strokeWidth="10"
              strokeDasharray={memoryRing.circumference}
              strokeDashoffset={memoryRing.offset}
              strokeLinecap="round"
              className="transition-all duration-500"
            />
            <text
              x="65"
              y="65"
              textAnchor="middle"
              dy="0.3em"
              className="text-2xl font-semibold fill-current transform rotate-90"
              style={{ transformOrigin: '65px 65px' }}
            >
              {memoryPercent.toFixed(0)}%
            </text>
          </svg>
          <span className="text-sm text-gray-600 dark:text-gray-400 mt-2 font-medium">Memory</span>
        </div>
      </div>

      {/* Footer Metrics */}
      <div className="flex justify-between text-xs text-gray-500 dark:text-gray-400 pt-3 border-t border-gray-200 dark:border-gray-700">
        <div className="flex items-center gap-1">
          <span>↓</span>
          <span>{formatSpeed(totalRx)}</span>
        </div>
        <div className="flex items-center gap-1">
          <span>↑</span>
          <span>{formatSpeed(totalTx)}</span>
        </div>
        {diskTotal > 0 && (
          <div>
            {formatBytes(diskUsed)} / {formatBytes(diskTotal)}
          </div>
        )}
      </div>
    </div>
  );
}
