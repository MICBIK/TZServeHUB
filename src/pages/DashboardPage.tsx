import { useMemo } from 'react';
import { Link } from 'react-router-dom';
import { Line, LineChart, ResponsiveContainer } from 'recharts';
import MetricCard from '../components/common/MetricCard';
import StatePanel from '../components/common/StatePanel';
import { useMetricHistory } from '../hooks/useMetricHistory';
import { useAllServersMetrics } from '../hooks/useAllServersMetrics';
import type { ServerMetrics } from '../hooks/useAllServersMetrics';
import { useUiCopy } from '../hooks/useUiCopy';
import { mergeHistorySeries, latestMetricTimestamp } from '../lib/chartData';
import { formatBytes, formatBytesPerSec, formatPercent } from '../lib/formatters';
import {
  getCpuMetrics,
  getDiskRates,
  getDiskUsage,
  getMemoryMetric,
  getMemorySummary,
  getNetworkRates,
} from '../lib/metricSelectors';
import { getAccessMethodLabelKey, getAdapterLabelKey } from '../lib/serverLabels';
import { useServerStore } from '../stores/serverStore';

type TrendDatum = { timestamp: number } & Record<string, number>;

function toneClass(value: number) {
  if (value >= 80) {
    return 'is-danger';
  }
  if (value >= 60) {
    return 'is-warn';
  }
  return 'is-ok';
}

function MetricMeter({
  value,
  label,
  detail,
}: {
  value: number;
  label: string;
  detail?: string;
}) {
  return (
    <div className="ops-meter">
      <div className="ops-meter-head">
        <span>{label}</span>
        <strong>{formatPercent(value)}</strong>
      </div>
      <div className="ops-meter-track">
        <span className={`ops-meter-fill ${toneClass(value)}`} style={{ width: `${Math.max(value, 3)}%` }} />
      </div>
      {detail ? <p className="ops-meter-detail">{detail}</p> : null}
    </div>
  );
}

function MiniTrend({
  data,
  series,
  height = 72,
}: {
  data: TrendDatum[];
  series: Array<{ key: string; color: string }>;
  height?: number;
}) {
  if (data.length === 0) {
    return <div className="ops-mini-empty" style={{ height }}>--</div>;
  }

  return (
    <ResponsiveContainer width="100%" height={height}>
      <LineChart data={data}>
        {series.map((item) => (
          <Line
            key={item.key}
            type="monotone"
            dataKey={item.key}
            stroke={item.color}
            strokeWidth={2}
            dot={false}
            isAnimationActive={false}
          />
        ))}
      </LineChart>
    </ResponsiveContainer>
  );
}

function MemoryRing({ value, compact = false }: { value: number; compact?: boolean }) {
  const radius = compact ? 24 : 30;
  const size = compact ? 72 : 84;
  const center = size / 2;
  const circumference = 2 * Math.PI * radius;
  const offset = circumference - (value / 100) * circumference;

  return (
    <div className={`ops-memory-ring ${compact ? 'is-compact' : ''}`}>
      <svg viewBox={`0 0 ${size} ${size}`} aria-hidden="true">
        <circle cx={center} cy={center} r={radius} className="ops-memory-ring-track" />
        <circle
          cx={center}
          cy={center}
          r={radius}
          className={`ops-memory-ring-fill ${toneClass(value)}`}
          style={{
            strokeDasharray: circumference,
            strokeDashoffset: offset,
          }}
        />
      </svg>
      <div>
        <strong>{formatPercent(value)}</strong>
        <span>RAM</span>
      </div>
    </div>
  );
}

function useFleetHealth(serversMetrics: ServerMetrics[]) {
  return useMemo(() => {
    let cpuCritical = 0;
    let memCritical = 0;
    let diskCritical = 0;
    const perServer: Array<{ serverId: string; cpu: number; mem: number; maxDisk: number }> = [];

    for (const { server, metrics } of serversMetrics) {
      if (metrics.length === 0) continue;
      const cpu = getCpuMetrics(metrics).total?.value ?? 0;
      const mem = getMemoryMetric(metrics)?.value ?? 0;
      const disks = getDiskUsage(metrics);
      const maxDisk = disks.reduce((max, d) => Math.max(max, d.percent), 0);

      if (cpu > 90) cpuCritical++;
      if (mem > 85) memCritical++;
      if (maxDisk > 90) diskCritical++;
      perServer.push({ serverId: server.id, cpu, mem, maxDisk });
    }

    return { cpuCritical, memCritical, diskCritical, perServer };
  }, [serversMetrics]);
}

function cpuToneClass(value: number) {
  if (value > 90) return 'is-danger';
  if (value > 75) return 'is-warn';
  return '';
}

function memToneClass(value: number) {
  if (value > 85) return 'is-danger';
  if (value > 70) return 'is-warn';
  return '';
}

function diskAlertToneClass(value: number) {
  if (value > 90) return 'is-danger';
  return '';
}

export default function DashboardPage() {
  const servers = useServerStore((state) => state.servers);
  const activeServerId = useServerStore((state) => state.activeServerId);
  const hydrated = useServerStore((state) => state.hydrated);
  const serverError = useServerStore((state) => state.error);
  const setActiveServer = useServerStore((state) => state.setActiveServer);
  const serversMetrics = useAllServersMetrics();
  const { t } = useUiCopy();
  const fleet = useFleetHealth(serversMetrics);

  const onlineCount = serversMetrics.filter((item) => item.server.enabled && item.metrics.length > 0).length;
  const offlineCount = Math.max(servers.length - onlineCount, 0);
  const selectedEntry = serversMetrics.find((item) => item.server.id === activeServerId) ?? serversMetrics[0] ?? null;
  const selectedServer = selectedEntry?.server ?? null;
  const selectedMetrics = selectedEntry?.metrics ?? [];
  const selectedCpu = getCpuMetrics(selectedMetrics);
  const selectedMemorySummary = getMemorySummary(selectedMetrics);
  const selectedMemoryPercent = selectedMemorySummary?.percent ?? getMemoryMetric(selectedMetrics)?.value ?? 0;
  const selectedDisks = getDiskUsage(selectedMetrics);
  const selectedNetwork = [...getNetworkRates(selectedMetrics)].sort(
    (left, right) => (right.rx ?? 0) + (right.tx ?? 0) - ((left.rx ?? 0) + (left.tx ?? 0)),
  );
  const selectedDiskRates = getDiskRates(selectedMetrics);
  const primaryNetwork = selectedNetwork[0];
  const primaryDisk = selectedDiskRates[0];
  const refreshToken = latestMetricTimestamp(selectedMetrics);

  const cpuHistory = useMetricHistory(selectedServer?.id ?? null, 'cpu_usage_percent', refreshToken);
  const memoryHistory = useMetricHistory(selectedServer?.id ?? null, 'memory_used_percent', refreshToken);
  const networkRxHistory = useMetricHistory(
    selectedServer?.id ?? null,
    primaryNetwork ? 'network_receive_bytes_total_rate' : null,
    refreshToken,
    primaryNetwork ? { interface: primaryNetwork.label } : undefined,
  );
  const networkTxHistory = useMetricHistory(
    selectedServer?.id ?? null,
    primaryNetwork ? 'network_transmit_bytes_total_rate' : null,
    refreshToken,
    primaryNetwork ? { interface: primaryNetwork.label } : undefined,
  );
  const diskReadHistory = useMetricHistory(
    selectedServer?.id ?? null,
    primaryDisk ? 'disk_read_bytes_total_rate' : null,
    refreshToken,
    primaryDisk ? { device: primaryDisk.label } : undefined,
  );
  const diskWriteHistory = useMetricHistory(
    selectedServer?.id ?? null,
    primaryDisk ? 'disk_write_bytes_total_rate' : null,
    refreshToken,
    primaryDisk ? { device: primaryDisk.label } : undefined,
  );

  const cpuTrend = mergeHistorySeries([{ key: 'cpu', points: cpuHistory.points }]);
  const memoryTrend = mergeHistorySeries([{ key: 'memory', points: memoryHistory.points }]);
  const networkTrend = mergeHistorySeries([
    { key: 'rx', points: networkRxHistory.points },
    { key: 'tx', points: networkTxHistory.points },
  ]);
  const diskTrend = mergeHistorySeries([
    { key: 'read', points: diskReadHistory.points },
    { key: 'write', points: diskWriteHistory.points },
  ]);
  const topDisks = selectedDisks.slice(0, 3);
  const activeStatusLabel = selectedMetrics.length > 0 ? t('live') : t('idle');

  return (
    <div className="h-full overflow-y-auto">
      <div className="mx-auto max-w-[1680px] p-3 space-y-3">
        <section className="glass-panel ops-hero rounded-[24px] border p-4">
          <div>
            <p className="shell-kicker">{t('nav_dashboard')}</p>
            <h1 className="hero-title mt-2">{t('dashboard_title')}</h1>
            <p className="hero-description mt-2">{t('dashboard_desc')}</p>
          </div>
          <div className="ops-hero-summary-card">
            <div className="ops-hero-summary-item">
              <span>{t('server_count_total')}</span>
              <strong>{servers.length}</strong>
            </div>
            <div className="ops-hero-summary-item">
              <span>{t('server_count_online')}</span>
              <strong>{onlineCount}</strong>
            </div>
            <div className="ops-hero-summary-item">
              <span>{t('server_count_offline')}</span>
              <strong>{offlineCount}</strong>
            </div>
          </div>
        </section>

        {servers.length > 0 && (
          <section className="glass-panel rounded-[24px] border p-4">
            <div className="summary-panel-header">
              <div>
                <p className="panel-label">{t('fleet_health')}</p>
                <p className="shell-muted mt-1 text-sm">{t('fleet_health_desc')}</p>
              </div>
            </div>
            <div className="fleet-health-cards mt-3" style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(180px, 1fr))', gap: '0.75rem' }}>
              <MetricCard title={t('server_count_total')} value={servers.length} unit={t('server_count_online') + ': ' + onlineCount} />
              <MetricCard
                title={t('fleet_cpu_critical')}
                value={fleet.cpuCritical}
                trend={fleet.cpuCritical > 0 ? 'up' : 'neutral'}
              />
              <MetricCard
                title={t('fleet_mem_critical')}
                value={fleet.memCritical}
                trend={fleet.memCritical > 0 ? 'up' : 'neutral'}
              />
              <MetricCard
                title={t('fleet_disk_critical')}
                value={fleet.diskCritical}
                trend={fleet.diskCritical > 0 ? 'up' : 'neutral'}
              />
            </div>
            {fleet.cpuCritical === 0 && fleet.memCritical === 0 && fleet.diskCritical === 0 && onlineCount > 0 && (
              <p className="shell-muted mt-2 text-sm" style={{ color: 'var(--success, #4ade80)' }}>
                {t('fleet_all_healthy')}
              </p>
            )}
          </section>
        )}

        {!hydrated ? (
          <StatePanel eyebrow={t('status_loading')} title={t('loading_title')} description={t('loading_desc')} />
        ) : serverError ? (
          <StatePanel eyebrow={t('status_error')} title={t('metrics_error_title')} description={serverError} />
        ) : servers.length === 0 ? (
          <StatePanel
            eyebrow={t('status_empty')}
            title={t('no_servers_title')}
            description={t('no_servers_desc')}
            action={
              <Link className="app-button inline-flex items-center justify-center" to="/settings">
                {t('add_server')}
              </Link>
            }
          />
        ) : (
          <div className="ops-console-grid">
            <section className="glass-panel ops-panel rounded-[24px] border p-4">
              <div className="summary-panel-header">
                <div>
                  <p className="panel-label">{t('overview')}</p>
                  <h2 className="panel-title mt-2">{t('settings_configured_targets')}</h2>
                </div>
                <span className="shell-count-chip">{servers.length}</span>
              </div>
              <div className="ops-status-table mt-4">
                <div className="ops-status-header">
                  <span>{t('server_target_label')}</span>
                  <span>{t('cpu_total')}</span>
                  <span>{t('memory_usage')}</span>
                  <span>{t('disk_usage')}</span>
                  <span>{t('network_ingress')}</span>
                </div>
                <div className="ops-status-body">
                  {serversMetrics.map(({ server, metrics }) => {
                    const cpu = getCpuMetrics(metrics).total?.value ?? 0;
                    const memory = getMemoryMetric(metrics)?.value ?? 0;
                    const disks = getDiskUsage(metrics);
                    const disk = disks[0]?.percent ?? 0;
                    const maxDisk = disks.reduce((max, d) => Math.max(max, d.percent), 0);
                    const network = [...getNetworkRates(metrics)].sort(
                      (left, right) => (right.rx ?? 0) + (right.tx ?? 0) - ((left.rx ?? 0) + (left.tx ?? 0)),
                    )[0];

                    return (
                      <button
                        type="button"
                        key={server.id}
                        className={`ops-status-row ${server.id === selectedServer?.id ? 'is-active' : ''}`}
                        onClick={() => setActiveServer(server.id)}
                      >
                        <span className="ops-system-cell">
                          <span className={`server-card-dot ${metrics.length > 0 ? 'is-online' : ''}`} />
                          <span className="ops-system-copy">
                            <strong>{server.name}</strong>
                            <small>{server.host}:{server.port}</small>
                          </span>
                        </span>
                        <div className={`ops-fleet-cell ${cpuToneClass(cpu)}`}>
                          <MetricMeter value={cpu} label={t(getAdapterLabelKey(server.adapter_type))} />
                        </div>
                        <div className={`ops-fleet-cell ${memToneClass(memory)}`}>
                          <MetricMeter value={memory} label={t(getAccessMethodLabelKey(server.access_method))} />
                        </div>
                        <div className={`ops-fleet-cell ${diskAlertToneClass(maxDisk)}`}>
                          <MetricMeter value={disk} label={disks[0]?.label ?? '--'} />
                        </div>
                        <div className="ops-network-cell">
                          <strong>{network?.rx ? formatBytesPerSec(network.rx) : '--'}</strong>
                          <span>{network?.tx ? formatBytesPerSec(network.tx) : '--'}</span>
                        </div>
                      </button>
                    );
                  })}
                </div>
              </div>
            </section>

            <section className="glass-panel ops-panel rounded-[24px] border p-4">
              <div className="summary-panel-header">
                <div>
                  <p className="panel-label">{t('active_server_label')}</p>
                  <h2 className="panel-title mt-2">{selectedServer?.name ?? t('unknown')}</h2>
                  <p className="shell-muted mt-2 text-sm">
                    {selectedServer
                      ? `${selectedServer.host}:${selectedServer.port} · ${t(getAdapterLabelKey(selectedServer.adapter_type))}`
                      : '--'}
                  </p>
                </div>
                <div className="ops-detail-meta">
                  <span className={`shell-live-pill ${selectedMetrics.length > 0 ? 'is-online' : ''}`}>
                    <span className="shell-live-pill-dot" aria-hidden="true" />
                    <span>{activeStatusLabel}</span>
                  </span>
                  <span className="shell-count-chip">{selectedServer?.polling_interval_sec ?? '--'}s</span>
                </div>
              </div>

              {selectedServer && selectedMetrics.length > 0 ? (
                <div className="ops-detail-surface mt-4">
                  <div className="ops-detail-summary-strip">
                    <div className="ops-detail-summary-item">
                      <span>{t('settings_adapter')}</span>
                      <strong>{t(getAdapterLabelKey(selectedServer.adapter_type))}</strong>
                    </div>
                    <div className="ops-detail-summary-item">
                      <span>{t('network_history')}</span>
                      <strong title={primaryNetwork?.label ?? '--'}>{primaryNetwork?.label ?? '--'}</strong>
                    </div>
                    <div className="ops-detail-summary-item">
                      <span>{t('disk_usage')}</span>
                      <strong>{selectedDisks.length}</strong>
                    </div>
                  </div>

                  <div className="ops-detail-grid mt-4">
                    <section className="ops-detail-card is-primary">
                      <div className="ops-detail-card-head">
                        <div>
                          <p className="panel-label">{t('cpu_total')}</p>
                          <h3>{formatPercent(selectedCpu.total?.value ?? 0)}</h3>
                        </div>
                        <span className={`ops-tone-pill ${toneClass(selectedCpu.total?.value ?? 0)}`}>{activeStatusLabel}</span>
                      </div>
                      <MiniTrend data={cpuTrend} series={[{ key: 'cpu', color: '#4ade80' }]} height={60} />
                      <div className="ops-core-grid">
                        {selectedCpu.perCore.slice(0, 6).map((metric) => (
                          <div key={metric.labels.core} className="ops-core-chip">
                            <div className="ops-core-chip-head">
                              <span>{metric.labels.core}</span>
                              <strong>{formatPercent(metric.value)}</strong>
                            </div>
                            <div className="ops-core-track">
                              <span className={`ops-core-fill ${toneClass(metric.value)}`} style={{ width: `${Math.max(metric.value, 3)}%` }} />
                            </div>
                          </div>
                        ))}
                      </div>
                    </section>

                    <section className="ops-detail-card">
                      <div className="ops-detail-card-head">
                        <div>
                          <p className="panel-label">{t('memory_usage')}</p>
                          <h3>{formatPercent(selectedMemoryPercent)}</h3>
                        </div>
                        <span className="shell-count-chip">{selectedMemorySummary ? formatBytes(selectedMemorySummary.total) : '--'}</span>
                      </div>
                      <div className="ops-memory-layout is-compact">
                        <MemoryRing value={selectedMemoryPercent} compact />
                        <div className="ops-module-trend">
                          <MiniTrend data={memoryTrend} series={[{ key: 'memory', color: '#60a5fa' }]} height={56} />
                          <div className="ops-compact-stats is-triple">
                            <div className="ops-compact-stat">
                              <span>{t('memory_usage')}</span>
                              <strong>{selectedMemorySummary ? formatBytes(selectedMemorySummary.used) : '--'}</strong>
                            </div>
                            <div className="ops-compact-stat">
                              <span>{t('memory_cached')}</span>
                              <strong>{selectedMemorySummary ? formatBytes(selectedMemorySummary.cached) : '--'}</strong>
                            </div>
                            <div className="ops-compact-stat">
                              <span>{t('memory_free')}</span>
                              <strong>{selectedMemorySummary ? formatBytes(selectedMemorySummary.free) : '--'}</strong>
                            </div>
                          </div>
                        </div>
                      </div>
                    </section>

                    <section className="ops-detail-card">
                      <div className="ops-detail-card-head">
                        <div>
                          <p className="panel-label">{t('overview')}</p>
                          <h3 title={primaryNetwork?.label ?? selectedDisks[0]?.label ?? '--'}>
                            {primaryNetwork?.label ?? selectedDisks[0]?.label ?? '--'}
                          </h3>
                        </div>
                        <span className="shell-count-chip">{selectedNetwork.length + selectedDisks.length}</span>
                      </div>
                      <div className="ops-io-grid">
                        <div className="ops-io-module">
                          <div className="ops-io-head">
                            <span>{t('network_history')}</span>
                            <strong title={primaryNetwork?.label ?? '--'}>{primaryNetwork?.label ?? '--'}</strong>
                          </div>
                          <MiniTrend
                            data={networkTrend}
                            series={[
                              { key: 'rx', color: '#38bdf8' },
                              { key: 'tx', color: '#22c55e' },
                            ]}
                            height={54}
                          />
                          <div className="ops-compact-stats">
                            <div className="ops-compact-stat">
                              <span>{t('network_ingress')}</span>
                              <strong>{primaryNetwork?.rx ? formatBytesPerSec(primaryNetwork.rx) : '--'}</strong>
                            </div>
                            <div className="ops-compact-stat">
                              <span>{t('network_egress')}</span>
                              <strong>{primaryNetwork?.tx ? formatBytesPerSec(primaryNetwork.tx) : '--'}</strong>
                            </div>
                          </div>
                        </div>

                        <div className="ops-io-module">
                          <div className="ops-io-head">
                            <span>{t('disk_usage')}</span>
                            <strong title={selectedDisks[0]?.label ?? '--'}>{selectedDisks[0]?.label ?? '--'}</strong>
                          </div>
                          <MiniTrend
                            data={diskTrend}
                            series={[
                              { key: 'read', color: '#f59e0b' },
                              { key: 'write', color: '#fb7185' },
                            ]}
                            height={54}
                          />
                          <div className="ops-disk-list compact">
                            {topDisks.map((disk) => {
                              const rate = selectedDiskRates.find((item) => item.label === disk.device);

                              return (
                                <div key={disk.label} className="ops-disk-row">
                                  <div className="ops-disk-row-head">
                                    <strong title={disk.label}>{disk.label}</strong>
                                    <span>{formatPercent(disk.percent)}</span>
                                  </div>
                                  <div className="ops-meter-track">
                                    <span className={`ops-meter-fill ${toneClass(disk.percent)}`} style={{ width: `${Math.max(disk.percent, 3)}%` }} />
                                  </div>
                                  <div className="ops-disk-row-foot">
                                    <span>{rate?.read ? formatBytesPerSec(rate.read) : '--'}</span>
                                    <span>{rate?.write ? formatBytesPerSec(rate.write) : '--'}</span>
                                  </div>
                                </div>
                              );
                            })}
                          </div>
                        </div>
                      </div>
                    </section>
                  </div>
                </div>
              ) : (
                <div className="mt-4">
                  <StatePanel
                    eyebrow={t('status_loading')}
                    title={t('metrics_loading_title')}
                    description={t('metrics_loading_desc')}
                  />
                </div>
              )}
            </section>
          </div>
        )}
      </div>
    </div>
  );
}
