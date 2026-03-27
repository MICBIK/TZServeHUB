import { formatBytes, formatBytesPerSec } from '../../lib/formatters';

interface NetworkMetrics {
  rx_rate: number;
  tx_rate: number;
  rx_total: number;
  tx_total: number;
  disk_read_rate: number;
  disk_write_rate: number;
}

interface NetworkPanelProps {
  metrics: NetworkMetrics;
}

export default function NetworkPanel({ metrics }: NetworkPanelProps) {
  return (
    <div className="glass-panel rounded-[28px] border p-6">
      <div className="space-y-6">
        {/* Network Traffic Section */}
        <div>
          <p className="panel-label mb-4">Network Traffic</p>
          <div className="grid grid-cols-2 gap-4">
            {/* Ingress (Download) */}
            <div className="space-y-2">
              <div className="flex items-baseline gap-2">
                <span className="text-3xl font-semibold text-green-500">↓</span>
                <div className="min-w-0 flex-1">
                  <div className="text-2xl font-bold tabular-nums">
                    {formatBytesPerSec(metrics.rx_rate)}
                  </div>
                  <div className="mt-1 text-xs opacity-60">
                    Total: {formatBytes(metrics.rx_total)}
                  </div>
                </div>
              </div>
            </div>

            {/* Egress (Upload) */}
            <div className="space-y-2">
              <div className="flex items-baseline gap-2">
                <span className="text-3xl font-semibold text-blue-500">↑</span>
                <div className="min-w-0 flex-1">
                  <div className="text-2xl font-bold tabular-nums">
                    {formatBytesPerSec(metrics.tx_rate)}
                  </div>
                  <div className="mt-1 text-xs opacity-60">
                    Total: {formatBytes(metrics.tx_total)}
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        {/* Disk I/O Section */}
        <div className="border-t pt-6">
          <p className="panel-label mb-4">Disk I/O</p>
          <div className="grid grid-cols-2 gap-4">
            {/* Read */}
            <div className="space-y-2">
              <div className="flex items-baseline gap-2">
                <span className="text-2xl font-semibold text-purple-500">R</span>
                <div className="min-w-0 flex-1">
                  <div className="text-xl font-bold tabular-nums">
                    {formatBytesPerSec(metrics.disk_read_rate)}
                  </div>
                  <div className="mt-1 text-xs opacity-60">Read Speed</div>
                </div>
              </div>
            </div>

            {/* Write */}
            <div className="space-y-2">
              <div className="flex items-baseline gap-2">
                <span className="text-2xl font-semibold text-orange-500">W</span>
                <div className="min-w-0 flex-1">
                  <div className="text-xl font-bold tabular-nums">
                    {formatBytesPerSec(metrics.disk_write_rate)}
                  </div>
                  <div className="mt-1 text-xs opacity-60">Write Speed</div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
