import { useState } from 'react';
import { LineChart, Line, ResponsiveContainer } from 'recharts';
import { useMetricHistory } from '../../hooks/useMetricHistory';

interface HistoricalSparklineProps {
  serverId: string;
  metricKey: string;
  labels?: Record<string, string>;
  color?: string;
}

export default function HistoricalSparkline({
  serverId,
  metricKey,
  labels,
  color = '#3b82f6',
}: HistoricalSparklineProps) {
  const [refreshToken] = useState(() => Date.now());
  const { points, loading } = useMetricHistory(serverId, metricKey, refreshToken, labels);

  if (loading || points.length === 0) {
    return (
      <div className="h-[60px] flex items-center justify-center text-gray-500 text-xs">
        {loading ? '...' : '—'}
      </div>
    );
  }

  return (
    <ResponsiveContainer width="100%" height={60}>
      <LineChart data={points}>
        <defs>
          <linearGradient id={`gradient-${metricKey}`} x1="0" y1="0" x2="0" y2="1">
            <stop offset="0%" stopColor={color} stopOpacity={0.3} />
            <stop offset="100%" stopColor={color} stopOpacity={0.05} />
          </linearGradient>
        </defs>
        <Line
          type="monotone"
          dataKey="value"
          stroke={color}
          strokeWidth={1.5}
          dot={false}
          fill={`url(#gradient-${metricKey})`}
          isAnimationActive={false}
        />
      </LineChart>
    </ResponsiveContainer>
  );
}
