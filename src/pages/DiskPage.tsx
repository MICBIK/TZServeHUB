import React from 'react';
import { Link } from 'react-router-dom';
import MetricChartCard from '../components/charts/MetricChartCard';
import MetricCard from '../components/common/MetricCard';
import StatePanel from '../components/common/StatePanel';
import { useMetricHistory } from '../hooks/useMetricHistory';
import { useMonitoringView } from '../hooks/useMonitoringView';
import { useUiCopy } from '../hooks/useUiCopy';
import { latestMetricTimestamp, mergeHistorySeries } from '../lib/chartData';
import { formatBytes, formatBytesPerSec, formatPercent } from '../lib/formatters';
import { getDiskRates, getDiskUsage } from '../lib/metricSelectors';

const DiskPage: React.FC = () => {
  const { activeServer, activeServerId, metrics, serverError, staleMetricsError, state } =
    useMonitoringView();
  const { t } = useUiCopy();
  const usage = getDiskUsage(metrics);
  const rates = getDiskRates(metrics);
  const hottestDisk = [...usage].sort((left, right) => right.percent - left.percent)[0];
  const busiestDevice = [...rates].sort(
    (left, right) =>
      Math.max(right.read ?? 0, right.write ?? 0) - Math.max(left.read ?? 0, left.write ?? 0),
  )[0];
  const refreshToken = latestMetricTimestamp(metrics);
  const readHistory = useMetricHistory(
    activeServerId,
    busiestDevice ? 'disk_read_bytes_total_rate' : null,
    refreshToken,
    busiestDevice ? { device: busiestDevice.label } : undefined,
  );
  const writeHistory = useMetricHistory(
    activeServerId,
    busiestDevice ? 'disk_write_bytes_total_rate' : null,
    refreshToken,
    busiestDevice ? { device: busiestDevice.label } : undefined,
  );
  const chartData = mergeHistorySeries([
    { key: 'read', points: readHistory.points },
    { key: 'write', points: writeHistory.points },
  ]);

  return (
    <div className="space-y-6">
      <section className="glass-panel hero-panel rounded-[30px] border p-8">
        <div className="hero-layout">
          <div className="min-w-0">
            <p className="shell-kicker">{t('nav_disk')}</p>
            <h1 className="hero-title mt-4">{t('disk_page_title')}</h1>
            <p className="hero-description mt-4">{t('disk_page_desc')}</p>
          </div>
          <div className="hero-server-card">
            <p className="panel-label">{t('active_target')}</p>
            <h2 className="hero-server-name mt-3 truncate">
              {activeServer?.name ?? t('unknown')}
            </h2>
          </div>
        </div>
      </section>

      {staleMetricsError ? <div className="shell-banner-warning">{t('stale_prefix')}{staleMetricsError}</div> : null}

      {state === 'hydrating' ? (
        <StatePanel eyebrow={t('status_loading')} title={t('loading_title')} description={t('loading_desc')} />
      ) : state === 'server-error' ? (
        <StatePanel eyebrow={t('status_error')} title={t('metrics_error_title')} description={serverError ?? 'Unknown error'} />
      ) : state === 'no-servers' ? (
        <StatePanel
          eyebrow={t('status_empty')}
          title={t('no_servers_title')}
          description={t('no_servers_desc')}
          action={<Link className="app-button inline-flex items-center justify-center" to="/settings">{t('add_server')}</Link>}
        />
      ) : state === 'selection-required' ? (
        <StatePanel eyebrow={t('status_selection')} title={t('selection_title')} description={t('selection_desc')} />
      ) : state === 'metrics-loading' ? (
        <StatePanel eyebrow={t('status_loading')} title={t('metrics_loading_title')} description={t('metrics_loading_desc')} />
      ) : state === 'metrics-error' ? (
        <StatePanel eyebrow={t('status_error')} title={t('metrics_error_title')} description={t('metrics_error_desc')} />
      ) : state === 'no-metrics' || (usage.length === 0 && rates.length === 0) ? (
        <StatePanel eyebrow={t('status_idle')} title={t('no_metrics_title')} description={t('no_metrics_desc')} />
      ) : (
        <>
          <div className="dashboard-grid status-grid">
            <MetricCard title={t('disk_mount_count')} value={usage.length} />
            <MetricCard title={t('disk_hottest')} value={hottestDisk?.label ?? '--'} unit={hottestDisk ? formatPercent(hottestDisk.percent) : undefined} />
            <MetricCard title={t('disk_busiest_write')} value={busiestDevice?.label ?? '--'} unit={busiestDevice?.write ? formatBytesPerSec(busiestDevice.write) : undefined} />
          </div>

          <MetricChartCard
            title={t('disk_throughput')}
            subtitle={busiestDevice?.label ?? t('nav_disk')}
            data={chartData}
            series={[
              { dataKey: 'read', label: 'Read', color: '#f8a640' },
              { dataKey: 'write', label: 'Write', color: '#ff6f91' },
            ]}
            emptyLabel={t('chart_empty')}
            valueFormatter={(value) => formatBytesPerSec(value)}
          />

          <section className="glass-panel rounded-[28px] border p-6">
            <div className="summary-panel-header">
              <div className="min-w-0">
                <p className="panel-label">{t('disk_usage')}</p>
                <h2 className="panel-title mt-2">{t('disk_usage')}</h2>
              </div>
            </div>
            <div className="summary-list mt-6">
              {usage.map((disk) => (
                <div key={disk.label} className="summary-row summary-row-2col">
                  <div className="min-w-0">
                    <p className="summary-label truncate">{disk.label}</p>
                    <p className="summary-subvalue truncate">
                      {formatBytes(disk.used)} / {formatBytes(disk.total)}
                    </p>
                  </div>
                  <span className="summary-value">{formatPercent(disk.percent)}</span>
                </div>
              ))}
            </div>
          </section>
        </>
      )}
    </div>
  );
};

export default DiskPage;
