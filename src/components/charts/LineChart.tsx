import { LineChart as RechartsLine, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts';
import { formatTimestamp } from '../../lib/formatters';

interface LineChartProps {
  data: Array<{ timestamp: number; value: number }>;
  dataKey?: string;
  color?: string;
  height?: number;
}

export default function LineChart({ data, dataKey = 'value', color = '#3b82f6', height = 300 }: LineChartProps) {
  return (
    <ResponsiveContainer width="100%" height={height}>
      <RechartsLine data={data}>
        <CartesianGrid strokeDasharray="3 3" stroke="#374151" />
        <XAxis
          dataKey="timestamp"
          tickFormatter={formatTimestamp}
          stroke="#9ca3af"
        />
        <YAxis stroke="#9ca3af" />
        <Tooltip
          contentStyle={{ backgroundColor: '#1f2937', border: '1px solid #374151' }}
          labelFormatter={formatTimestamp}
        />
        <Line type="monotone" dataKey={dataKey} stroke={color} strokeWidth={2} dot={false} />
      </RechartsLine>
    </ResponsiveContainer>
  );
}
