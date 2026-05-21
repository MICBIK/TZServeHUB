import { useId, useState } from 'react';
import ServerCard from '../components/server/ServerCard';
import KnownHostsList from '../components/settings/KnownHostsList';
import StatePanel from '../components/common/StatePanel';
import { useUiCopy } from '../hooks/useUiCopy';
import { POLLING_INTERVALS } from '../lib/constants';
import { getAccessMethodLabelKey, getAdapterLabelKey } from '../lib/serverLabels';
import type { LanguageMode, ThemeMode } from '../services/tauri';
import { useServerStore } from '../stores/serverStore';
import { useSettingsStore } from '../stores/settingsStore';
import type { ServerFormData } from '../types/server';

function readNumber(value: FormDataEntryValue | null, fallback: number) {
  const parsed = Number(value);
  return Number.isFinite(parsed) ? parsed : fallback;
}

function ThemeChoiceIcon({ theme }: { theme: ThemeMode }) {
  if (theme === 'dark') {
    return (
      <svg viewBox="0 0 24 24" aria-hidden="true">
        <path
          d="M20.5 14.5A8.5 8.5 0 1 1 10 4a7 7 0 0 0 10.5 10.5Z"
          fill="none"
          stroke="currentColor"
          strokeWidth="1.8"
          strokeLinecap="round"
          strokeLinejoin="round"
        />
      </svg>
    );
  }

  return (
    <svg viewBox="0 0 24 24" aria-hidden="true">
      <circle cx="12" cy="12" r="4" fill="none" stroke="currentColor" strokeWidth="1.8" />
      <path
        d="M12 2.5v2.4M12 19.1v2.4M4.9 4.9l1.7 1.7M17.4 17.4l1.7 1.7M2.5 12h2.4M19.1 12h2.4M4.9 19.1l1.7-1.7M17.4 6.6l1.7-1.7"
        fill="none"
        stroke="currentColor"
        strokeWidth="1.8"
        strokeLinecap="round"
      />
    </svg>
  );
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

function SaveIcon() {
  return (
    <svg viewBox="0 0 24 24" aria-hidden="true">
      <path
        d="M6 4.75h9.5l3.75 3.75V19a1.75 1.75 0 0 1-1.75 1.75H6.75A1.75 1.75 0 0 1 5 19V6.5A1.75 1.75 0 0 1 6.75 4.75Z"
        fill="none"
        stroke="currentColor"
        strokeWidth="1.8"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
      <path
        d="M8.5 4.75v5h6v-5M9.25 15.25h5.5"
        fill="none"
        stroke="currentColor"
        strokeWidth="1.8"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>
  );
}

export default function SettingsPage() {
  const addFormKey = useId();
  const settingsFormKey = useId();
  const [authType, setAuthType] = useState<'token' | 'ssh_key' | 'password'>('token');
  const servers = useServerStore((state) => state.servers);
  const activeServerId = useServerStore((state) => state.activeServerId);
  const hydrated = useServerStore((state) => state.hydrated);
  const setActiveServer = useServerStore((state) => state.setActiveServer);
  const addServer = useServerStore((state) => state.addServer);
  const removeServer = useServerStore((state) => state.removeServer);
  const serverError = useServerStore((state) => state.error);

  const settings = useSettingsStore((state) => state.settings);
  const settingsHydrated = useSettingsStore((state) => state.hydrated);
  const updateSettings = useSettingsStore((state) => state.updateSettings);
  const settingsError = useSettingsStore((state) => state.error);
  const { t } = useUiCopy();

  async function applyTheme(theme: ThemeMode) {
    if (settings.theme === theme) {
      return;
    }

    await updateSettings({ ...settings, theme });
  }

  async function applyLanguage(language: LanguageMode) {
    if (settings.language === language) {
      return;
    }

    await updateSettings({ ...settings, language });
  }

  async function handleAddServer(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const form = event.currentTarget;
    const formData = new FormData(form);
    const selectedAuthType = String(formData.get('auth_type') ?? 'token') as 'token' | 'ssh_key' | 'password';

    const input: ServerFormData = {
      name: String(formData.get('name') ?? '').trim(),
      host: String(formData.get('host') ?? '').trim(),
      port: readNumber(formData.get('port'), 9100),
      adapter_type: String(formData.get('adapter_type') ?? 'go_agent') as ServerFormData['adapter_type'],
      access_method: String(formData.get('access_method') ?? 'private') as ServerFormData['access_method'],
      polling_interval_sec: readNumber(
        formData.get('polling_interval_sec'),
        settings.default_polling_interval_sec,
      ),
      auth_type: selectedAuthType,
    };

    // Add auth fields based on type
    if (selectedAuthType === 'token') {
      const authToken = String(formData.get('auth_token') ?? '').trim();
      input.auth_token = authToken || undefined;
    } else if (selectedAuthType === 'ssh_key') {
      const sshKey = String(formData.get('ssh_key') ?? '').trim();
      const passphrase = String(formData.get('ssh_passphrase') ?? '').trim();
      input.ssh_key = sshKey || undefined;
      input.ssh_passphrase = passphrase || undefined;
    } else if (selectedAuthType === 'password') {
      const password = String(formData.get('password') ?? '').trim();
      input.password = password || undefined;
    }

    await addServer(input);
    form.reset();
    setAuthType('token');
  }

  async function handleSaveSettings(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const formData = new FormData(event.currentTarget);

    await updateSettings({
      default_polling_interval_sec: readNumber(
        formData.get('default_polling_interval_sec'),
        settings.default_polling_interval_sec,
      ),
      data_retention_days: readNumber(
        formData.get('data_retention_days'),
        settings.data_retention_days,
      ),
      theme: settings.theme,
      language: settings.language,
    });
  }

  return (
    <div className="h-full overflow-y-auto">
      <div className="settings-page-shell mx-auto max-w-[1480px] p-2.5 space-y-2.5">
        <section className="glass-panel hero-panel settings-hero-panel rounded-[22px] border">
          <div className="settings-hero-stack">
            <div className="min-w-0 settings-hero-copy">
              <p className="shell-kicker">{t('nav_settings')}</p>
              <h1 className="hero-title settings-hero-title mt-2">{t('settings_title')}</h1>
              <p className="hero-description settings-hero-description mt-2">{t('settings_desc')}</p>
            </div>
            <div className="hero-server-card settings-preferences-bar">
              <div className="settings-preferences-head">
                <div>
                  <p className="panel-label">{t('settings_app_settings')}</p>
                  <h2 className="settings-preferences-title mt-2">{t('settings_display_title')}</h2>
                </div>
                <p className="shell-muted settings-preferences-desc">{t('settings_display_desc')}</p>
              </div>

              <div className="settings-preferences-inline mt-3">
                <div className="settings-preference-inline-block">
                  <span className="settings-preference-label">{t('settings_theme_label')}</span>
                  <div className="settings-option-group is-picker" role="group" aria-label={t('settings_theme_label')}>
                    <button
                      type="button"
                      className={`settings-option-button is-picker ${settings.theme === 'dark' ? 'is-active' : ''}`}
                      onClick={() => applyTheme('dark')}
                      aria-label={t('theme_dark')}
                    >
                      <span className="settings-option-icon-shell" aria-hidden="true">
                        <ThemeChoiceIcon theme="dark" />
                      </span>
                      <span>{t('theme_dark')}</span>
                    </button>
                    <button
                      type="button"
                      className={`settings-option-button is-picker ${settings.theme === 'light' ? 'is-active' : ''}`}
                      onClick={() => applyTheme('light')}
                      aria-label={t('theme_light')}
                    >
                      <span className="settings-option-icon-shell" aria-hidden="true">
                        <ThemeChoiceIcon theme="light" />
                      </span>
                      <span>{t('theme_light')}</span>
                    </button>
                  </div>
                </div>

                <div className="settings-preference-inline-block">
                  <span className="settings-preference-label">{t('settings_language_label')}</span>
                  <div className="settings-option-group is-picker" role="group" aria-label={t('settings_language_label')}>
                    <button
                      type="button"
                      className={`settings-option-button is-picker ${settings.language === 'zh-CN' ? 'is-active' : ''}`}
                      onClick={() => applyLanguage('zh-CN')}
                    >
                      <span className="settings-option-locale" aria-hidden="true">中</span>
                      <span>{t('language_zh')}</span>
                    </button>
                    <button
                      type="button"
                      className={`settings-option-button is-picker ${settings.language === 'en-US' ? 'is-active' : ''}`}
                      onClick={() => applyLanguage('en-US')}
                    >
                      <span className="settings-option-locale" aria-hidden="true">EN</span>
                      <span>{t('language_en')}</span>
                    </button>
                  </div>
                </div>

                <form
                  key={`${settingsFormKey}-${settings.default_polling_interval_sec}-${settings.data_retention_days}`}
                  className="settings-preferences-inline-form"
                  onSubmit={handleSaveSettings}
                >
                  <label className="settings-inline-field">
                    <span className="field-label">{t('settings_default_polling')}</span>
                    <input
                      className="app-input"
                      name="default_polling_interval_sec"
                      type="number"
                      min="1"
                      defaultValue={settings.default_polling_interval_sec}
                    />
                  </label>
                  <label className="settings-inline-field">
                    <span className="field-label">{t('settings_retention_days')}</span>
                    <input
                      className="app-input"
                      name="data_retention_days"
                      type="number"
                      min="1"
                      defaultValue={settings.data_retention_days}
                    />
                  </label>
                  <div className="settings-mini-runtime-actions">
                    <button className="app-button app-button-quiet app-button-sm settings-save-button" type="submit">
                      <SaveIcon />
                      {t('settings_save_settings')}
                    </button>
                  </div>
                </form>
              </div>
            </div>
          </div>
        </section>

        {!hydrated || !settingsHydrated ? (
          <StatePanel
            eyebrow={t('status_loading')}
            title={t('loading_title')}
            description={t('loading_desc')}
          />
        ) : null}
        {serverError ? (
          <StatePanel eyebrow={t('status_error')} title={t('metrics_error_title')} description={serverError} />
        ) : null}
        {settingsError ? (
          <StatePanel eyebrow={t('status_error')} title={t('metrics_error_title')} description={settingsError} />
        ) : null}

        <div className="dashboard-grid settings-workspace-grid">
          <section className="glass-panel settings-section settings-workspace-pane col-span-12 rounded-[22px] border p-3.5 xl:col-span-7">
            <div className="summary-panel-header">
              <div className="min-w-0">
                <p className="panel-label">{t('settings_registry')}</p>
                <h2 className="panel-title mt-2">{t('settings_configured_targets')}</h2>
              </div>
              <span className="shell-count-chip">{servers.length}</span>
            </div>

            <div className="settings-section-scroll mt-4">
              <div className="settings-server-grid">
                {servers.length > 0 ? (
                  servers.map((server) => (
                    <div key={server.id} className="target-shell-card">
                      <ServerCard
                        server={server}
                        isActive={server.id === activeServerId}
                        onClick={() => setActiveServer(server.id)}
                      />
                      <div className="target-shell-card-actions">
                        <button
                          type="button"
                          className="app-button app-button-danger"
                          onClick={() => removeServer(server.id)}
                        >
                          <DeleteIcon />
                          {t('settings_delete_target')}
                        </button>
                      </div>
                    </div>
                  ))
                ) : (
                  <div className="xl:col-span-2">
                    <StatePanel
                      eyebrow={t('status_empty')}
                      title={t('settings_no_server_title')}
                      description={t('settings_no_server_desc')}
                    />
                  </div>
                )}
              </div>
            </div>
          </section>

          <section className="glass-panel settings-section settings-workspace-pane col-span-12 rounded-[22px] border p-3.5 xl:col-span-5">
            <p className="panel-label">{t('settings_add_target')}</p>
            <h2 className="panel-title mt-2">{t('settings_create_target')}</h2>
            <div className="settings-section-scroll mt-4">
              <form
                key={`${addFormKey}-${settings.default_polling_interval_sec}`}
                className="form-stack"
                onSubmit={handleAddServer}
              >
                <label className="form-field">
                  <span className="field-label">{t('settings_name_label')}</span>
                  <input
                    className="app-input"
                    name="name"
                    placeholder={t('settings_name_placeholder')}
                    required
                  />
                </label>
                <div className="form-grid form-grid-2">
                  <label className="form-field">
                    <span className="field-label">{t('settings_host_label')}</span>
                    <input
                      className="app-input"
                      name="host"
                      placeholder={t('settings_host_placeholder')}
                      required
                    />
                  </label>
                  <label className="form-field">
                    <span className="field-label">{t('settings_port_label')}</span>
                    <input
                      className="app-input"
                      name="port"
                      type="number"
                      min="1"
                      max="65535"
                      defaultValue="9100"
                      required
                    />
                  </label>
                </div>
                <label className="form-field">
                  <span className="field-label">{t('settings_polling_label')}</span>
                  <select
                    className="app-select"
                    name="polling_interval_sec"
                    defaultValue={String(settings.default_polling_interval_sec)}
                  >
                    {POLLING_INTERVALS.map((value) => (
                      <option key={value} value={value}>
                        {value}s
                      </option>
                    ))}
                  </select>
                </label>
                <div className="form-grid form-grid-2">
                  <label className="form-field">
                    <span className="field-label">{t('settings_adapter')}</span>
                    <select className="app-select" name="adapter_type" defaultValue="go_agent">
                      <option value="go_agent">{t(getAdapterLabelKey('go_agent'))}</option>
                    </select>
                  </label>
                  <label className="form-field">
                    <span className="field-label">{t('settings_access_method')}</span>
                    <select className="app-select" name="access_method" defaultValue="private">
                      <option value="private">{t(getAccessMethodLabelKey('private'))}</option>
                      <option value="tunnel">{t(getAccessMethodLabelKey('tunnel'))}</option>
                      <option value="gateway">{t(getAccessMethodLabelKey('gateway'))}</option>
                    </select>
                  </label>
                </div>
                <div className="form-field">
                  <span className="field-label">{t('auth_type_label')}</span>
                  <div className="settings-auth-grid mt-2">
                    <label className="settings-auth-option">
                      <input
                        type="radio"
                        name="auth_type"
                        value="token"
                        checked={authType === 'token'}
                        onChange={(e) => setAuthType(e.target.value as 'token' | 'ssh_key' | 'password')}
                      />
                      <span>{t('auth_type_token')}</span>
                    </label>
                    <label className="settings-auth-option">
                      <input
                        type="radio"
                        name="auth_type"
                        value="ssh_key"
                        checked={authType === 'ssh_key'}
                        onChange={(e) => setAuthType(e.target.value as 'token' | 'ssh_key' | 'password')}
                      />
                      <span>{t('auth_type_ssh_key')}</span>
                    </label>
                    <label className="settings-auth-option">
                      <input
                        type="radio"
                        name="auth_type"
                        value="password"
                        checked={authType === 'password'}
                        onChange={(e) => setAuthType(e.target.value as 'token' | 'ssh_key' | 'password')}
                      />
                      <span>{t('auth_type_password')}</span>
                    </label>
                  </div>
                </div>

                {authType === 'token' && (
                  <label className="form-field">
                    <span className="field-label">{t('auth_type_token')}</span>
                    <input
                      className="app-input"
                      name="auth_token"
                      placeholder={t('settings_token_placeholder')}
                    />
                  </label>
                )}

                {authType === 'ssh_key' && (
                  <>
                    <label className="form-field">
                      <span className="field-label">{t('ssh_key_path_label')}</span>
                      <input
                        className="app-input"
                        name="ssh_key"
                        placeholder={t('settings_ssh_key_placeholder')}
                      />
                    </label>
                    <label className="form-field">
                      <span className="field-label">{t('ssh_passphrase_label')}</span>
                      <input
                        className="app-input"
                        name="ssh_passphrase"
                        type="password"
                        placeholder={t('settings_ssh_passphrase_placeholder')}
                      />
                    </label>
                  </>
                )}

                {authType === 'password' && (
                  <label className="form-field">
                    <span className="field-label">{t('password_label')}</span>
                    <input
                      className="app-input"
                      name="password"
                      type="password"
                      placeholder={t('password_label')}
                    />
                  </label>
                )}

                <button className="app-button w-full" type="submit">
                  {t('settings_save_target')}
                </button>
              </form>
            </div>
          </section>
        </div>

        <KnownHostsList />
      </div>
    </div>
  );
}
