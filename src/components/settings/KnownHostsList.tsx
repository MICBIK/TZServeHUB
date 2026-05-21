import { useEffect, useState } from 'react';
import { listKnownHosts, removeKnownHost, type KnownHost } from '../../services/tauri';

function formatFingerprint(fingerprint: string) {
  return fingerprint.length > 16 ? `${fingerprint.slice(0, 16)}...` : fingerprint;
}

export default function KnownHostsList() {
  const [hosts, setHosts] = useState<KnownHost[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  async function refreshHosts() {
    setLoading(true);
    setError(null);
    try {
      setHosts(await listKnownHosts());
    } catch (caught) {
      setError(caught instanceof Error ? caught.message : String(caught));
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    let active = true;

    async function loadHosts() {
      setLoading(true);
      setError(null);
      try {
        const rows = await listKnownHosts();
        if (active) {
          setHosts(rows);
        }
      } catch (caught) {
        if (active) {
          setError(caught instanceof Error ? caught.message : String(caught));
        }
      } finally {
        if (active) {
          setLoading(false);
        }
      }
    }

    void loadHosts();

    return () => {
      active = false;
    };
  }, []);

  async function handleRemove(host: KnownHost) {
    await removeKnownHost(host.host, host.port);
    await refreshHosts();
  }

  return (
    <section className="sc-panel settings-section p-3.5" data-testid="settings-known-hosts">
      <div className="summary-panel-header">
        <div className="min-w-0">
          <p className="panel-label">Security</p>
          <h2 className="panel-title mt-2">SSH Known Hosts</h2>
        </div>
        <span className="shell-count-chip">{hosts.length}</span>
      </div>

      {loading ? <p className="shell-muted mt-3">Loading known hosts...</p> : null}
      {error ? <p className="shell-muted mt-3">{error}</p> : null}
      {!loading && !error && hosts.length === 0 ? (
        <p className="shell-muted mt-3">No SSH host keys recorded yet.</p>
      ) : null}

      {hosts.length > 0 ? (
        <div className="form-stack mt-4">
          {hosts.map((host) => (
            <div key={`${host.host}:${host.port}`} className="target-shell-card">
              <div className="min-w-0">
                <p className="panel-title">{host.host}:{host.port}</p>
                <p className="shell-muted" title={host.fingerprint}>
                  {host.algorithm} - {formatFingerprint(host.fingerprint)}
                </p>
              </div>
              <div className="target-shell-card-actions">
                <button
                  type="button"
                  className="app-button app-button-danger"
                  data-testid={`known-host-remove-${host.host}-${host.port}`}
                  onClick={() => void handleRemove(host)}
                >
                  Remove
                </button>
              </div>
            </div>
          ))}
        </div>
      ) : null}
    </section>
  );
}
