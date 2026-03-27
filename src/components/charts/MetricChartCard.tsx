import {
  CartesianGrid,
  Line,
  LineChart,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from 'recharts';
import { formatTimestamp } from '../../lib/formatters';

type ChartDatum = {
  timestamp: number;
};

interface ChartSeries<T extends ChartDatum> {
  dataKey: Extract<keyof T, string>;
  label: string;
  color: string;
}

interface MetricChartCardProps<T extends ChartDatum> {
  title: string;
  subtitle: string;
  data: T[];
  series: Array<ChartSeries<T>>;
  emptyLabel: string;
  valueFormatter?: (value: number) => string;
}

export default function MetricChartCard<T extends ChartDatum>({
  title,
  subtitle,
  data,
  series,
  emptyLabel,
  valueFormatter,
}: MetricChartCardProps<T>) {
  return (
    <section className="glass-panel chart-card rounded-[28px] border p-6">
      <div className="chart-card-header">
        <div className="min-w-0">
          <p className="chart-kicker">{subtitle}</p>
          <h3 className="chart-title">{title}</h3>
        </div>
        <div className="chart-legend">
          {series.map((item) => (
            <span key={item.dataKey} className="chart-legend-item">
              <span
                className="chart-legend-dot"
                style={{ backgroundColor: item.color }}
              />
              {item.label}
            </span>
          ))}
        </div>
      </div>

      <div className="mt-6 h-64">
        {data.length > 0 ? (
          <ResponsiveContainer width="100%" height="100%">
            <LineChart data={data}>
              <CartesianGrid stroke="var(--chart-grid)" vertical={false} />
              <XAxis
                dataKey="timestamp"
                minTickGap={28}
                tickFormatter={(value) => formatTimestamp(Number(value))}
                tick={{ fill: 'var(--text-muted)', fontSize: 11 }}
                stroke="var(--chart-grid)"
              />
              <YAxis
                tickFormatter={(value) =>
                  valueFormatter ? valueFormatter(Number(value)) : String(value)
                }
                tick={{ fill: 'var(--text-muted)', fontSize: 11 }}
                stroke="var(--chart-grid)"
                width={68}
              />
              <Tooltip
                contentStyle={{
                  background: 'var(--surface-elevated)',
                  border: '1px solid var(--surface-border)',
                  borderRadius: '16px',
                  color: 'var(--text-primary)',
                }}
                formatter={(value: number) =>
                  valueFormatter ? valueFormatter(value) : value.toFixed(2)
                }
                labelFormatter={(value) => formatTimestamp(Number(value))}
              />
              {series.map((item) => (
                <Line
                  key={item.dataKey}
                  type="monotone"
                  dataKey={item.dataKey}
                  stroke={item.color}
                  strokeWidth={2.4}
                  dot={false}
                  activeDot={{ r: 4 }}
                />
              ))}
            </LineChart>
          </ResponsiveContainer>
        ) : (
          <div className="chart-empty">{emptyLabel}</div>
        )}
      </div>
    </section>
  );
}
