import React, { useState } from 'react';
import { Link } from 'react-router-dom';
import MetricCard from '../components/common/MetricCard';
import StatePanel from '../components/common/StatePanel';
import { useMonitoringView } from '../hooks/useMonitoringView';
import { useUiCopy } from '../hooks/useUiCopy';
import { runPingProbe, runTcpProbe, runDnsProbe } from '../services/tauri';
import type { PingProbeResult, TcpProbeResult, DnsProbeResult } from '../types/probe';

const ProbePage: React.FC = () => {
  const { activeServer, serverError, state } = useMonitoringView();
  const { t } = useUiCopy();
  const hasSelectionContext =
    state !== 'hydrating' &&
    state !== 'server-error' &&
    state !== 'no-servers' &&
    state !== 'selection-required';

  // Ping state
  const [pingHost, setPingHost] = useState('');
  const [pingCount, setPingCount] = useState('4');
  const [pingLoading, setPingLoading] = useState(false);
  const [pingResult, setPingResult] = useState<PingProbeResult | null>(null);
  const [pingError, setPingError] = useState<string | null>(null);

  // TCP state
  const [tcpHost, setTcpHost] = useState('');
  const [tcpPort, setTcpPort] = useState('80');
  const [tcpTimeout, setTcpTimeout] = useState('5000');
  const [tcpLoading, setTcpLoading] = useState(false);
  const [tcpResult, setTcpResult] = useState<TcpProbeResult | null>(null);
  const [tcpError, setTcpError] = useState<string | null>(null);

  // DNS state
  const [dnsDomain, setDnsDomain] = useState('');
  const [dnsServer, setDnsServer] = useState('8.8.8.8');
  const [dnsTimeout, setDnsTimeout] = useState('5000');
  const [dnsLoading, setDnsLoading] = useState(false);
  const [dnsResult, setDnsResult] = useState<DnsProbeResult | null>(null);
  const [dnsError, setDnsError] = useState<string | null>(null);

  // Pre-fill host from active server
  const defaultHost = activeServer?.host ?? '';

  async function handlePing() {
    setPingLoading(true);
    setPingError(null);
    setPingResult(null);
    try {
      const host = pingHost.trim() || defaultHost;
      const result = await runPingProbe(host, parseInt(pingCount, 10) || 4);
      setPingResult(result);
    } catch (err) {
      setPingError(String(err));
    } finally {
      setPingLoading(false);
    }
  }

  async function handleTcp() {
    setTcpLoading(true);
    setTcpError(null);
    setTcpResult(null);
    try {
      const host = tcpHost.trim() || defaultHost;
      const result = await runTcpProbe(host, parseInt(tcpPort, 10) || 80, parseInt(tcpTimeout, 10) || 5000);
      setTcpResult(result);
    } catch (err) {
      setTcpError(String(err));
    } finally {
      setTcpLoading(false);
    }
  }

  async function handleDns() {
    setDnsLoading(true);
    setDnsError(null);
    setDnsResult(null);
    try {
      const domain = dnsDomain.trim();
      if (!domain) {
        setDnsError('Domain cannot be empty');
        return;
      }
      const result = await runDnsProbe(domain, dnsServer.trim() || undefined, parseInt(dnsTimeout, 10) || 5000);
      setDnsResult(result);
    } catch (err) {
      setDnsError(String(err));
    } finally {
      setDnsLoading(false);
    }
  }

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
          {/* Ping Probe */}
          <section className="glass-panel col-span-12 rounded-[28px] border p-6 lg:col-span-4">
            <div className="summary-panel-header">
              <div className="min-w-0">
                <p className="panel-label">{t('probe_signal_icmp')}</p>
                <h2 className="panel-title mt-2">{t('probe_ping_title')}</h2>
              </div>
            </div>
            <div className="mt-4 space-y-3">
              <input
                type="text"
                className="app-input w-full"
                placeholder={`${t('probe_host_label')} (${defaultHost || '0.0.0.0'})`}
                value={pingHost}
                onChange={(e) => setPingHost(e.target.value)}
              />
              <input
                type="number"
                className="app-input w-full"
                placeholder={t('probe_count_label')}
                value={pingCount}
                onChange={(e) => setPingCount(e.target.value)}
                min={1}
                max={20}
              />
              <button
                className="app-button w-full"
                onClick={handlePing}
                disabled={pingLoading}
              >
                {pingLoading ? t('probe_running') : t('probe_run')}
              </button>
            </div>
            {pingError ? (
              <p className="mt-4 text-sm text-red-400">{t('probe_error')}: {pingError}</p>
            ) : null}
            {pingResult ? (
              <div className="mt-4 grid grid-cols-2 gap-3">
                <MetricCard title={t('probe_avg_rtt')} value={pingResult.avg_rtt_ms.toFixed(2)} unit="ms" />
                <MetricCard title={t('probe_loss_rate')} value={pingResult.loss_rate.toFixed(1)} unit="%" />
                <MetricCard title={t('probe_packets_sent')} value={pingResult.packets_sent} />
                <MetricCard title={t('probe_packets_lost')} value={pingResult.packets_lost} />
              </div>
            ) : null}
          </section>

          {/* TCP Probe */}
          <section className="glass-panel col-span-12 rounded-[28px] border p-6 lg:col-span-4">
            <div className="summary-panel-header">
              <div className="min-w-0">
                <p className="panel-label">{t('probe_signal_tcp')}</p>
                <h2 className="panel-title mt-2">{t('probe_tcp_title')}</h2>
              </div>
            </div>
            <div className="mt-4 space-y-3">
              <input
                type="text"
                className="app-input w-full"
                placeholder={`${t('probe_host_label')} (${defaultHost || '0.0.0.0'})`}
                value={tcpHost}
                onChange={(e) => setTcpHost(e.target.value)}
              />
              <input
                type="number"
                className="app-input w-full"
                placeholder={t('probe_port_label')}
                value={tcpPort}
                onChange={(e) => setTcpPort(e.target.value)}
                min={1}
                max={65535}
              />
              <input
                type="number"
                className="app-input w-full"
                placeholder={t('probe_timeout_label')}
                value={tcpTimeout}
                onChange={(e) => setTcpTimeout(e.target.value)}
              />
              <button
                className="app-button w-full"
                onClick={handleTcp}
                disabled={tcpLoading}
              >
                {tcpLoading ? t('probe_running') : t('probe_run')}
              </button>
            </div>
            {tcpError ? (
              <p className="mt-4 text-sm text-red-400">{t('probe_error')}: {tcpError}</p>
            ) : null}
            {tcpResult ? (
              <div className="mt-4 grid grid-cols-2 gap-3">
                <MetricCard
                  title={t('probe_tcp_title')}
                  value={tcpResult.reachable ? t('probe_reachable') : t('probe_unreachable')}
                  trend={tcpResult.reachable ? 'up' : 'down'}
                />
                <MetricCard title={t('probe_latency')} value={tcpResult.latency_ms.toFixed(2)} unit="ms" />
              </div>
            ) : null}
          </section>

          {/* DNS Probe */}
          <section className="glass-panel col-span-12 rounded-[28px] border p-6 lg:col-span-4">
            <div className="summary-panel-header">
              <div className="min-w-0">
                <p className="panel-label">{t('probe_signal_dns')}</p>
                <h2 className="panel-title mt-2">{t('probe_dns_title')}</h2>
              </div>
            </div>
            <div className="mt-4 space-y-3">
              <input
                type="text"
                className="app-input w-full"
                placeholder={t('probe_domain_label')}
                value={dnsDomain}
                onChange={(e) => setDnsDomain(e.target.value)}
              />
              <input
                type="text"
                className="app-input w-full"
                placeholder={`${t('probe_dns_server_label')} (8.8.8.8)`}
                value={dnsServer}
                onChange={(e) => setDnsServer(e.target.value)}
              />
              <input
                type="number"
                className="app-input w-full"
                placeholder={t('probe_timeout_label')}
                value={dnsTimeout}
                onChange={(e) => setDnsTimeout(e.target.value)}
              />
              <button
                className="app-button w-full"
                onClick={handleDns}
                disabled={dnsLoading}
              >
                {dnsLoading ? t('probe_running') : t('probe_run')}
              </button>
            </div>
            {dnsError ? (
              <p className="mt-4 text-sm text-red-400">{t('probe_error')}: {dnsError}</p>
            ) : null}
            {dnsResult ? (
              <div className="mt-4 grid grid-cols-2 gap-3">
                <MetricCard
                  title={t('probe_dns_title')}
                  value={dnsResult.resolved ? t('probe_resolved') : t('probe_not_resolved')}
                  trend={dnsResult.resolved ? 'up' : 'down'}
                />
                <MetricCard title={t('probe_latency')} value={dnsResult.latency_ms.toFixed(2)} unit="ms" />
              </div>
            ) : null}
          </section>
        </div>
      ) : null}
    </div>
  );
};

export default ProbePage;
