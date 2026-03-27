interface ActivityRingsProps {
  cpu: number;
  memory: number;
  disk: number;
  size?: number;
}

export default function ActivityRings({ cpu, memory, disk, size = 200 }: ActivityRingsProps) {
  const getColor = (ring: 'cpu' | 'memory' | 'disk') => {
    // ServerCat official colors
    if (ring === 'cpu') return '#007AFF'; // iOS blue
    if (ring === 'memory') return '#34C759'; // iOS green
    return '#FF9500'; // iOS orange (disk)
  };

  const createRing = (percent: number, radius: number, ring: 'cpu' | 'memory' | 'disk') => {
    const circumference = 2 * Math.PI * radius;
    const offset = circumference - (percent / 100) * circumference;
    return { circumference, offset, color: getColor(ring) };
  };

  const center = size / 2;
  const cpuRing = createRing(cpu, center - 20, 'cpu');
  const memRing = createRing(memory, center - 42, 'memory');
  const diskRing = createRing(disk, center - 64, 'disk');

  return (
    <svg width={size} height={size} viewBox={`0 0 ${size} ${size}`}>
      <defs>
        {[cpuRing, memRing, diskRing].map((ring, i) => (
          <linearGradient key={i} id={`grad${i}`} x1="0%" y1="0%" x2="100%" y2="100%">
            <stop offset="0%" stopColor={ring.color} stopOpacity="0.8" />
            <stop offset="100%" stopColor={ring.color} stopOpacity="1" />
          </linearGradient>
        ))}
      </defs>

      {[
        { ring: diskRing, radius: center - 64, label: 'Disk', value: disk },
        { ring: memRing, radius: center - 42, label: 'Mem', value: memory },
        { ring: cpuRing, radius: center - 20, label: 'CPU', value: cpu },
      ].map(({ ring, radius }, i) => (
        <g key={i}>
          <circle
            cx={center}
            cy={center}
            r={radius}
            fill="none"
            stroke="#374151"
            strokeWidth="16"
          />
          <circle
            cx={center}
            cy={center}
            r={radius}
            fill="none"
            stroke={`url(#grad${2 - i})`}
            strokeWidth="16"
            strokeLinecap="round"
            strokeDasharray={ring.circumference}
            strokeDashoffset={ring.offset}
            transform={`rotate(-90 ${center} ${center})`}
          />
        </g>
      ))}

      <text x={center} y={center - 10} textAnchor="middle" fill="#9ca3af" fontSize="14">
        System
      </text>
      <text x={center} y={center + 10} textAnchor="middle" fill="#e5e7eb" fontSize="20" fontWeight="600">
        {Math.round((cpu + memory + disk) / 3)}%
      </text>
    </svg>
  );
}
