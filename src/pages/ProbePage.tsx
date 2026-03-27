import React from 'react';
import { Link } from 'react-router-dom';
import StatePanel from '../components/common/StatePanel';
import { useMonitoringView } from '../hooks/useMonitoringView';
import { useUiCopy } from '../hooks/useUiCopy';

const ProbePage: React.FC = () => {
  const { activeServer, serverError, servers, state } = useMonitoringView();
  const { t } = useUiCopy();
  const hasSelectionContext =
    state !== 'hydrating' &&
    state !== 'server-error' &&
    state !== 'no-servers' &&
    state !== 'selection-required';

  return (
    <div className="space-y-6">
      <section className="glass-panel hero-panel rounded-[30px] border p-8">
        <div className="hero-layout">
          <div className="min-w-0">
            <p className="shell-kicker">{t('nav_probes')}</p>
            <h1 className="hero-title mt-4">{t('probe_title')}</h1>
            <p className="hero-description mt-4">{t('probe_desc')}</p>
          </div>
          <div className="hero-server-card">
            <p className="panel-label">{t('active_target')}</p>
            <h2 className="hero-server-name mt-3 truncate">
              {activeServer?.name ?? t('unknown')}
            </h2>
          </div>
        </div>
      </section>

      {state === 'hydrating' ? (
        <StatePanel eyebrow={t('status_loading')} title={t('loading_title')} description={t('loading_desc')} />
      ) : state === 'server-error' ? (
        <StatePanel eyebrow={t('status_error')} title={t('metrics_error_title')} description={serverError ?? 'Unknown error'} />
      ) : state === 'no-servers' ? (
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
      ) : state === 'selection-required' ? (
        <StatePanel eyebrow={t('status_selection')} title={t('selection_title')} description={t('selection_desc')} />
      ) : hasSelectionContext ? (
        <div className="dashboard-grid">
          <section className="glass-panel col-span-12 rounded-[28px] border p-6 lg:col-span-7">
            <div className="summary-panel-header">
              <div className="min-w-0">
                <p className="panel-label">{t('overview')}</p>
                <h2 className="panel-title mt-2">{t('probe_scope_title')}</h2>
              </div>
            </div>
            <p className="hero-description mt-4">{t('probe_scope_desc')}</p>
            <div className="summary-row mt-6">
              <div className="min-w-0">
                <p className="summary-label truncate">{activeServer?.name ?? t('unknown')}</p>
                <p className="summary-subvalue truncate">
                  {t('probe_active_target_desc', { count: servers.length })}
                </p>
              </div>
              <span className="summary-value">{t('live')}</span>
            </div>
          </section>

          <section className="glass-panel col-span-12 rounded-[28px] border p-6 lg:col-span-5">
            <div className="summary-panel-header">
              <div className="min-w-0">
                <p className="panel-label">{t('targets')}</p>
                <h2 className="panel-title mt-2">{t('probe_signals_title')}</h2>
              </div>
            </div>
            <div className="summary-list mt-6">
              {[t('probe_signal_icmp'), t('probe_signal_tcp'), t('probe_signal_dns')].map((item) => (
                <div key={item} className="summary-row">
                  <div className="min-w-0">
                    <p className="summary-label truncate">{item}</p>
                    <p className="summary-subvalue">{t('deferred_notice')}</p>
                  </div>
                </div>
              ))}
            </div>
          </section>
        </div>
      ) : null}
    </div>
  );
};

export default ProbePage;
