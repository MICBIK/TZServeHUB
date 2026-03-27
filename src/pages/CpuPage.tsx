import React from 'react';
import { Link } from 'react-router-dom';
import MetricChartCard from '../components/charts/MetricChartCard';
import MetricCard from '../components/common/MetricCard';
import StatePanel from '../components/common/StatePanel';
import { useMetricHistory } from '../hooks/useMetricHistory';
import { useMonitoringView } from '../hooks/useMonitoringView';
import { useUiCopy } from '../hooks/useUiCopy';
import { latestMetricTimestamp } from '../lib/chartData';
import { formatPercent } from '../lib/formatters';
import { getCpuMetrics } from '../lib/metricSelectors';

const CpuPage: React.FC = () => {
  const { activeServer, activeServerId, metrics, serverError, staleMetricsError, state } =
    useMonitoringView();
  const { t } = useUiCopy();
  const cpu = getCpuMetrics(metrics);
  const hottestCore = [...cpu.perCore].sort((left, right) => right.value - left.value)[0];
  const refreshToken = latestMetricTimestamp(metrics);
  const cpuHistory = useMetricHistory(activeServerId, 'cpu_usage_percent', refreshToken);

  return (
    <div className="space-y-6">
      <section className="glass-panel hero-panel rounded-[30px] border p-8">
        <div className="hero-layout">
          <div className="min-w-0">
            <p className="shell-kicker">{t('nav_cpu')}</p>
            <h1 className="hero-title mt-4">{t('cpu_page_title')}</h1>
            <p className="hero-description mt-4">{t('cpu_page_desc')}</p>
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
      ) : state === 'no-metrics' || (!cpu.total && cpu.perCore.length === 0) ? (
        <StatePanel eyebrow={t('status_idle')} title={t('no_metrics_title')} description={t('no_metrics_desc')} />
      ) : (
        <>
          <div className="dashboard-grid status-grid">
            <MetricCard title={t('cpu_total')} value={cpu.total ? formatPercent(cpu.total.value) : '--'} />
            <MetricCard title={t('cpu_core_count')} value={cpu.perCore.length} />
            <MetricCard
              title={t('cpu_hot_core')}
              value={hottestCore ? `Core ${hottestCore.labels.core}` : '--'}
              unit={hottestCore ? formatPercent(hottestCore.value) : undefined}
            />
          </div>

          <MetricChartCard
            title={t('cpu_history')}
            subtitle={t('nav_cpu')}
            data={cpuHistory.points}
            series={[{ dataKey: 'value', label: t('cpu_total'), color: '#22c7f8' }]}
            emptyLabel={t('chart_empty')}
            valueFormatter={(value) => formatPercent(value)}
          />

          <section className="glass-panel rounded-[28px] border p-6">
            <div className="summary-panel-header">
              <div className="min-w-0">
                <p className="panel-label">{t('cpu_matrix')}</p>
                <h2 className="panel-title mt-2">{t('per_core_snapshot')}</h2>
              </div>
            </div>
            <div className="core-grid mt-6">
              {cpu.perCore.map((metric) => (
                <div key={metric.labels.core} className="core-row">
                  <div className="core-row-head">
                    <span className="truncate">Core {metric.labels.core}</span>
                    <span>{formatPercent(metric.value)}</span>
                  </div>
                  <div className="core-bar">
                    <div className="core-bar-fill" style={{ width: `${Math.min(metric.value, 100)}%` }} />
                  </div>
                </div>
              ))}
            </div>
          </section>
        </>
      )}
    </div>
  );
};

export default CpuPage;
