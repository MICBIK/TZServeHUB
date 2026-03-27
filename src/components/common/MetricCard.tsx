interface MetricCardProps {
  title: string;
  value: string | number;
  unit?: string;
  trend?: 'up' | 'down' | 'neutral';
}

export default function MetricCard({ title, value, unit, trend }: MetricCardProps) {
  const trendColors = {
    up: 'metric-trend-up',
    down: 'metric-trend-down',
    neutral: 'metric-trend-neutral',
  };

  return (
    <div className="metric-card rounded-[26px] border p-5 shadow-xl">
      <h4 className="metric-card-title">{title}</h4>
      <div className="metric-card-body mt-4">
        <span className="metric-card-value" title={String(value)}>
          {value}
        </span>
        {unit ? (
          <span className="metric-card-unit" title={unit}>
            {unit}
          </span>
        ) : null}
      </div>
      {trend && (
        <span className={`mt-3 block text-xs ${trendColors[trend]}`}>
          {trend === 'up' ? '↑' : trend === 'down' ? '↓' : '→'}
        </span>
      )}
    </div>
  );
}
