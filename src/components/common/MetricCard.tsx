interface MetricCardProps {
  title: string;
  value: string | number;
  unit?: string;
  trend?: 'up' | 'down' | 'neutral';
}

export default function MetricCard({ title, value, unit, trend }: MetricCardProps) {
  const trendColors = {
    up: 'text-red-400',
    down: 'text-green-400',
    neutral: 'text-gray-400',
  };

  return (
    <div className="bg-gray-800 rounded-lg p-4 border border-gray-700">
      <h4 className="text-sm text-gray-400 mb-2">{title}</h4>
      <div className="flex items-baseline gap-2">
        <span className="text-2xl font-semibold text-white">{value}</span>
        {unit && <span className="text-sm text-gray-500">{unit}</span>}
      </div>
      {trend && (
        <span className={`text-xs ${trendColors[trend]} mt-1 block`}>
          {trend === 'up' ? '↑' : trend === 'down' ? '↓' : '→'}
        </span>
      )}
    </div>
  );
}
