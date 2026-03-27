import React, { useCallback, useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import StatePanel from '../components/common/StatePanel';
import { useMonitoringView } from '../hooks/useMonitoringView';
import { useUiCopy } from '../hooks/useUiCopy';
import { addAlertRule, listAlertEvents, listAlertRules, removeAlertRule } from '../services/tauri';
import type { AlertEvent, AlertRule } from '../types/alert';

const METRIC_KEYS = [
  'cpu_usage_percent',
  'memory_used_percent',
  'disk_used_bytes',
  'network_receive_bytes_total_rate',
  'network_transmit_bytes_total_rate',
] as const;

function conditionSymbol(condition: AlertRule['condition']): string {
  switch (condition) {
    case 'gt': return '>';
    case 'lt': return '<';
    case 'eq': return '=';
  }
}

function formatTimestamp(epoch: number): string {
  return new Date(epoch * 1000).toLocaleString();
}

function DeleteIcon() {
  return (
    <svg viewBox="0 0 24 24" aria-hidden="true">
      <path
        d="M4.5 7h15M9.5 3.75h5M9 10.5v6M15 10.5v6M7.5 7l.7 10.1a2 2 0 0 0 2 1.9h3.6a2 2 0 0 0 2-1.9L16.5 7"
        fill="none"
        stroke="currentColor"
        strokeWidth="1.8"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>
  );
}

function PlusIcon() {
  return (
    <svg viewBox="0 0 24 24" aria-hidden="true">
      <path
        d="M12 5v14M5 12h14"
        fill="none"
        stroke="currentColor"
        strokeWidth="1.8"
        strokeLinecap="round"
      />
    </svg>
  );
}

const AlertPage: React.FC = () => {
  const { activeServer, serverError, servers, state } = useMonitoringView();
  const { t } = useUiCopy();

  const [rules, setRules] = useState<AlertRule[]>([]);
  const [events, setEvents] = useState<AlertEvent[]>([]);
  const [rulesLoading, setRulesLoading] = useState(true);
  const [eventsLoading, setEventsLoading] = useState(true);
  const [formError, setFormError] = useState<string | null>(null);

  const hasSelectionContext =
    state !== 'hydrating' &&
    state !== 'server-error' &&
    state !== 'no-servers' &&
    state !== 'selection-required';

  const loadRules = useCallback(async () => {
    setRulesLoading(true);
    try {
      const data = await listAlertRules();
      setRules(data);
    } catch {
      // silently fail — empty state will show
    } finally {
      setRulesLoading(false);
    }
  }, []);

  const loadEvents = useCallback(async () => {
    setEventsLoading(true);
    try {
      const data = await listAlertEvents(undefined, 50);
      setEvents(data);
    } catch {
      // silently fail
    } finally {
      setEventsLoading(false);
    }
  }, []);

  useEffect(() => {
    if (hasSelectionContext) {
      loadRules();
      loadEvents();
    }
  }, [hasSelectionContext, loadRules, loadEvents]);

  async function handleAddRule(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setFormError(null);
    const form = event.currentTarget;
    const formData = new FormData(form);

    const serverId = String(formData.get('server_id') ?? '').trim();
    const name = String(formData.get('name') ?? '').trim();
    const metricKey = String(formData.get('metric_key') ?? '');
    const condition = String(formData.get('condition') ?? 'gt');
    const threshold = Number(formData.get('threshold'));
    const durationSec = Number(formData.get('duration_sec'));

    if (!serverId || !name || !metricKey) {
      setFormError('All fields are required');
      return;
    }

    try {
      await addAlertRule({
        server_id: serverId,
        name,
        metric_key: metricKey,
        condition,
        threshold,
        duration_sec: durationSec,
      });
      form.reset();
      await loadRules();
      await loadEvents();
    } catch (err) {
      setFormError(String(err));
    }
  }

  async function handleDeleteRule(id: string) {
    try {
      await removeAlertRule(id);
      await loadRules();
      await loadEvents();
    } catch {
      // silently fail
    }
  }

  function serverNameById(id: string): string {
    return servers.find((s) => s.id === id)?.name ?? id;
  }

  return (
    <div className="h-full overflow-y-auto">
      <div className="mx-auto max-w-[1680px] p-3 space-y-3">
        <section className="glass-panel hero-panel rounded-[24px] border p-6">
          <div className="hero-layout">
            <div className="min-w-0">
              <p className="shell-kicker">{t('nav_alerts')}</p>
              <h1 className="hero-title mt-4">{t('alert_title')}</h1>
              <p className="hero-description mt-4">{t('alert_desc')}</p>
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
            {/* Left panel: Rules */}
            <section className="glass-panel col-span-12 rounded-[28px] border p-6 lg:col-span-7">
              <div className="summary-panel-header">
                <div className="min-w-0">
                  <p className="panel-label">{t('alert_rules_title')}</p>
                  <h2 className="panel-title mt-2">{t('alert_add_rule')}</h2>
                </div>
                <span className="shell-count-chip">{rules.length}</span>
              </div>

              {/* Add Rule Form */}
              <form className="form-stack mt-4" onSubmit={handleAddRule}>
                <div className="form-grid form-grid-2">
                  <label className="form-field">
                    <span className="field-label">{t('alert_rule_name')}</span>
                    <input
                      className="app-input"
                      name="name"
                      placeholder={t('alert_rule_name_placeholder')}
                      required
                    />
                  </label>
                  <label className="form-field">
                    <span className="field-label">{t('alert_server')}</span>
                    <select className="app-select" name="server_id" defaultValue={activeServer?.id ?? ''} required>
                      <option value="" disabled>--</option>
                      {servers.map((server) => (
                        <option key={server.id} value={server.id}>
                          {server.name}
                        </option>
                      ))}
                    </select>
                  </label>
                </div>

                <div className="form-grid form-grid-2">
                  <label className="form-field">
                    <span className="field-label">{t('alert_metric_key')}</span>
                    <select className="app-select" name="metric_key" defaultValue="cpu_usage_percent" required>
                      {METRIC_KEYS.map((key) => (
                        <option key={key} value={key}>{key}</option>
                      ))}
                    </select>
                  </label>
                  <label className="form-field">
                    <span className="field-label">{t('alert_condition')}</span>
                    <select className="app-select" name="condition" defaultValue="gt">
                      <option value="gt">{t('alert_condition_gt')}</option>
                      <option value="lt">{t('alert_condition_lt')}</option>
                      <option value="eq">{t('alert_condition_eq')}</option>
                    </select>
                  </label>
                </div>

                <div className="form-grid form-grid-2">
                  <label className="form-field">
                    <span className="field-label">{t('alert_threshold')}</span>
                    <input
                      className="app-input"
                      name="threshold"
                      type="number"
                      step="any"
                      defaultValue="80"
                      required
                    />
                  </label>
                  <label className="form-field">
                    <span className="field-label">{t('alert_duration')} ({t('alert_duration_unit')})</span>
                    <input
                      className="app-input"
                      name="duration_sec"
                      type="number"
                      min="0"
                      defaultValue="60"
                      required
                    />
                  </label>
                </div>

                {formError ? (
                  <p className="text-sm" style={{ color: 'var(--tone-danger, #ef4444)' }}>{formError}</p>
                ) : null}

                <button className="app-button w-full" type="submit">
                  <PlusIcon />
                  {t('alert_submit_rule')}
                </button>
              </form>

              {/* Rules list */}
              <div className="summary-list mt-6">
                {rulesLoading ? (
                  <StatePanel eyebrow={t('status_loading')} title={t('loading_title')} description={t('loading_desc')} />
                ) : rules.length === 0 ? (
                  <StatePanel
                    eyebrow={t('status_empty')}
                    title={t('alert_no_rules_title')}
                    description={t('alert_no_rules_desc')}
                  />
                ) : (
                  rules.map((rule) => (
                    <div key={rule.id} className="summary-row">
                      <div className="min-w-0" style={{ flex: 1 }}>
                        <p className="summary-label truncate">{rule.name}</p>
                        <p className="summary-subvalue truncate">
                          {rule.metric_key} {conditionSymbol(rule.condition)} {rule.threshold}
                          {' · '}{rule.duration_sec}{t('alert_duration_unit')}
                          {' · '}{serverNameById(rule.server_id)}
                        </p>
                      </div>
                      <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
                        <span className={`shell-live-pill ${rule.enabled ? 'is-online' : ''}`}>
                          <span className="shell-live-pill-dot" aria-hidden="true" />
                          <span>{rule.enabled ? t('live') : t('idle')}</span>
                        </span>
                        <button
                          type="button"
                          className="app-button app-button-danger app-button-sm"
                          onClick={() => handleDeleteRule(rule.id)}
                          title={t('alert_delete_rule')}
                        >
                          <DeleteIcon />
                        </button>
                      </div>
                    </div>
                  ))
                )}
              </div>
            </section>

            {/* Right panel: Events */}
            <section className="glass-panel col-span-12 rounded-[28px] border p-6 lg:col-span-5">
              <div className="summary-panel-header">
                <div className="min-w-0">
                  <p className="panel-label">{t('alert_events_title')}</p>
                  <h2 className="panel-title mt-2">{t('alert_signal_history')}</h2>
                </div>
                <span className="shell-count-chip">{events.length}</span>
              </div>

              <div className="summary-list mt-6">
                {eventsLoading ? (
                  <StatePanel eyebrow={t('status_loading')} title={t('loading_title')} description={t('loading_desc')} />
                ) : events.length === 0 ? (
                  <StatePanel
                    eyebrow={t('status_empty')}
                    title={t('alert_no_events_title')}
                    description={t('alert_no_events_desc')}
                  />
                ) : (
                  events.map((event) => (
                    <div key={event.id} className="summary-row">
                      <div className="min-w-0" style={{ flex: 1 }}>
                        <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
                          <span
                            className={`shell-live-pill ${event.status === 'firing' ? 'is-online' : ''}`}
                            style={event.status === 'firing' ? { '--pill-color': 'var(--tone-danger, #ef4444)' } as React.CSSProperties : undefined}
                          >
                            <span className="shell-live-pill-dot" aria-hidden="true" />
                            <span>
                              {event.status === 'firing' ? t('alert_status_firing') : t('alert_status_resolved')}
                            </span>
                          </span>
                          <span className="summary-subvalue">{serverNameById(event.server_id)}</span>
                        </div>
                        <p className="summary-label mt-1 truncate">{event.message}</p>
                        <p className="summary-subvalue">
                          {t('alert_event_time')}: {formatTimestamp(event.fired_at)}
                          {event.resolved_at ? ` · ${t('alert_status_resolved')}: ${formatTimestamp(event.resolved_at)}` : ''}
                        </p>
                      </div>
                    </div>
                  ))
                )}
              </div>
            </section>
          </div>
        ) : null}
      </div>
    </div>
  );
};

export default AlertPage;
