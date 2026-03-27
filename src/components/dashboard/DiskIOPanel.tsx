import { formatBytesPerSec } from '../../lib/formatters';

interface DiskIOMetrics {
  disk_read_rate: number;
  disk_write_rate: number;
}

interface DiskIOPanelProps {
  metrics: DiskIOMetrics;
}

export default function DiskIOPanel({ metrics }: DiskIOPanelProps) {
  return (
    <div className="glass-panel rounded-[28px] border p-6">
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
  );
}
