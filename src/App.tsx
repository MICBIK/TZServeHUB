import { useEffect, useState } from 'react';
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import AppLayout from './components/layout/AppLayout';
import DashboardPage from './pages/DashboardPage';
import SettingsPage from './pages/SettingsPage';
import { useServerStore } from './stores/serverStore';
import { useSettingsStore } from './stores/settingsStore';
import { useUiCopy } from './hooks/useUiCopy';

function App() {
  const fetchServers = useServerStore((state) => state.fetchServers);
  const fetchSettings = useSettingsStore((state) => state.fetchSettings);
  const settings = useSettingsStore((state) => state.settings);
  const serversHydrated = useServerStore((state) => state.hydrated);
  const settingsHydrated = useSettingsStore((state) => state.hydrated);
  const [entered, setEntered] = useState(false);
  const [showBootOverlay, setShowBootOverlay] = useState(true);
  const { t } = useUiCopy();

  useEffect(() => {
    void Promise.all([fetchServers(), fetchSettings()]);
  }, [fetchServers, fetchSettings]);

  useEffect(() => {
    document.documentElement.dataset.theme = settings.theme === 'light' ? 'light' : 'dark';
    document.documentElement.lang = settings.language;
  }, [settings.language, settings.theme]);

  useEffect(() => {
    const frame = window.requestAnimationFrame(() => setEntered(true));
    return () => window.cancelAnimationFrame(frame);
  }, []);

  useEffect(() => {
    const timeout = window.setTimeout(() => setShowBootOverlay(false), 1850);
    return () => window.clearTimeout(timeout);
  }, []);

  const bootstrapped = serversHydrated && settingsHydrated;

  return (
    <div className={`app-shell-shell ${entered ? 'is-entered' : ''}`}>
      {showBootOverlay ? (
        <div className="boot-overlay">
          <div className="boot-overlay-panel">
            <svg
              className="boot-svg"
              viewBox="0 0 320 260"
              fill="none"
              aria-hidden="true"
            >
              <rect className="boot-frame" x="28" y="40" width="264" height="160" rx="28" />
              <path
                className="boot-grid"
                d="M72 82H248M72 118H248M72 154H248M104 66V178M160 66V178M216 66V178"
              />
              <path
                className="boot-signal"
                d="M58 154C96 154 104 108 136 108C168 108 176 168 214 168C240 168 250 130 262 120"
              />
              <circle className="boot-node boot-node-a" cx="136" cy="108" r="6" />
              <circle className="boot-node boot-node-b" cx="214" cy="168" r="6" />
              <path className="boot-scan" d="M72 94H248" />
              <path className="boot-caption" d="M104 224H216" />
            </svg>
            <div className="boot-copy">
              <p className="boot-kicker">SERVERHUB</p>
              <h1>{t('boot_title')}</h1>
            </div>
          </div>
        </div>
      ) : null}
      <BrowserRouter>
        <Routes>
          <Route element={<AppLayout bootstrapped={bootstrapped} />}>
            <Route path="/" element={<DashboardPage />} />
            <Route path="/settings" element={<SettingsPage />} />
          </Route>
        </Routes>
      </BrowserRouter>
    </div>
  );
}

export default App;
