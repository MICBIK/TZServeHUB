interface CoreHeatmapProps {
  coreMetrics: number[];
}

export default function CoreHeatmap({ coreMetrics }: CoreHeatmapProps) {
  const getColor = (value: number): string => {
    const clampedValue = Math.max(0, Math.min(100, value));
    const intensity = clampedValue / 100;
    return `rgba(156, 214, 255, ${0.1 + intensity * 0.9})`;
  };

  const columns = Math.ceil(Math.sqrt(coreMetrics.length));

  return (
    <div className="rounded-[26px] border p-5 shadow-xl bg-gray-900">
      <h4 className="text-sm font-medium text-gray-400 mb-4">Per-Core CPU Usage</h4>
      <div
        className="grid gap-2"
        style={{
          gridTemplateColumns: `repeat(${columns}, minmax(0, 1fr))`,
        }}
      >
        {coreMetrics.map((usage, index) => (
          <div
            key={index}
            className="aspect-square rounded flex items-center justify-center text-xs font-medium transition-colors duration-200"
            style={{
              backgroundColor: getColor(usage),
              color: usage > 50 ? '#1f2937' : '#9ca3af',
            }}
            title={`Core ${index}: ${usage.toFixed(1)}%`}
          >
            {usage.toFixed(0)}%
          </div>
        ))}
      </div>
    </div>
  );
}
