import React from 'react';
import { Link } from 'react-router-dom';
import MetricChartCard from '../components/charts/MetricChartCard';
import MetricCard from '../components/common/MetricCard';
import StatePanel from '../components/common/StatePanel';
import { useMetricHistory } from '../hooks/useMetricHistory';
import { useMonitoringView } from '../hooks/useMonitoringView';
import { useUiCopy } from '../hooks/useUiCopy';
import { latestMetricTimestamp, mergeHistorySeries } from '../lib/chartData';
import { formatBytesPerSec } from '../lib/formatters';
import { getNetworkRates } from '../lib/metricSelectors';

const NetworkPage: React.FC = () => {
  const { activeServer, activeServerId, metrics, serverError, staleMetricsError, state } =
    useMonitoringView();
  const { t } = useUiCopy();
  const network = getNetworkRates(metrics);
  const busiestRx = [...network].sort((left, right) => (right.rx ?? 0) - (left.rx ?? 0))[0];
  const busiestTx = [...network].sort((left, right) => (right.tx ?? 0) - (left.tx ?? 0))[0];
  const leadInterface = busiestRx ?? busiestTx ?? network[0];
  const refreshToken = latestMetricTimestamp(metrics);
  const rxHistory = useMetricHistory(
    activeServerId,
    leadInterface ? 'network_receive_bytes_total_rate' : null,
    refreshToken,
    leadInterface ? { interface: leadInterface.label } : undefined,
  );
  const txHistory = useMetricHistory(
    activeServerId,
    leadInterface ? 'network_transmit_bytes_total_rate' : null,
    refreshToken,
    leadInterface ? { interface: leadInterface.label } : undefined,
  );
  const chartData = mergeHistorySeries([
    { key: 'rx', points: rxHistory.points },
    { key: 'tx', points: txHistory.points },
  ]);

  return (
    <div className="space-y-6">
      <section className="glass-panel hero-panel rounded-[30px] border p-8">
        <div className="hero-layout">
          <div className="min-w-0">
            <p className="shell-kicker">{t('nav_network')}</p>
            <h1 className="hero-title mt-4">{t('network_page_title')}</h1>
            <p className="hero-description mt-4">{t('network_page_desc')}</p>
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
      ) : state === 'no-metrics' || network.length === 0 ? (
        <StatePanel eyebrow={t('status_idle')} title={t('no_metrics_title')} description={t('no_metrics_desc')} />
      ) : (
        <>
          <div className="dashboard-grid status-grid">
            <MetricCard title={t('network_interfaces')} value={network.length} />
            <MetricCard title={t('network_busiest_rx')} value={busiestRx?.label ?? '--'} unit={busiestRx?.rx ? formatBytesPerSec(busiestRx.rx) : undefined} />
            <MetricCard title={t('network_busiest_tx')} value={busiestTx?.label ?? '--'} unit={busiestTx?.tx ? formatBytesPerSec(busiestTx.tx) : undefined} />
          </div>

          <MetricChartCard
            title={t('network_history')}
            subtitle={leadInterface?.label ?? t('nav_network')}
            data={chartData}
            series={[
              { dataKey: 'rx', label: 'RX', color: '#2bd6a4' },
              { dataKey: 'tx', label: 'TX', color: '#6ab8ff' },
            ]}
            emptyLabel={t('chart_empty')}
            valueFormatter={(value) => formatBytesPerSec(value)}
          />

          <section className="glass-panel rounded-[28px] border p-6">
            <div className="summary-panel-header">
              <div className="min-w-0">
                <p className="panel-label">{t('network_matrix')}</p>
                <h2 className="panel-title mt-2">{t('network_history')}</h2>
              </div>
            </div>
            <div className="summary-list mt-6">
              {network.map((item) => (
                <div key={item.label} className="summary-row summary-row-2col">
                  <div className="min-w-0">
                    <p className="summary-label truncate">{item.label}</p>
                  </div>
                  <div className="summary-metrics">
                    <span>{item.rx ? formatBytesPerSec(item.rx) : '--'}</span>
                    <span>{item.tx ? formatBytesPerSec(item.tx) : '--'}</span>
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

export default NetworkPage;
