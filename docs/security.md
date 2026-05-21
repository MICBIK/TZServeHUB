# ServerHUB Security Notes

## Tauri Identifier

`src-tauri/tauri.conf.json` uses `dev.serverhub.app` as the desktop app identifier. This replaces the Tauri template value and follows reverse-DNS naming, while keeping the Rust package name as `serverhub`.

## Content Security Policy

ServerHUB v0.2 enables CSP in `src-tauri/tauri.conf.json` instead of relying on `csp: null`.

| Directive | Value | Reason |
|---|---|---|
| `default-src` | `'self'` | Default deny for external resources. |
| `script-src` | `'self'` | Allows only bundled app scripts; no inline script, eval, or CDN execution. |
| `style-src` | `'self' 'unsafe-inline'` | Allows bundled styles plus inline styles used by Tailwind dev injection and SVG/chart rendering. This is the only `unsafe-inline` exception. |
| `connect-src` | `'self' ipc: http://ipc.localhost` | Allows Tauri IPC transport and same-origin app requests, without external hosts. |
| `img-src` | `'self' asset: http://asset.localhost data:` | Allows bundled assets, Tauri asset protocol, and small data URI images/icons. |
| `font-src` | `'self' data:` | Allows bundled fonts and embedded font data only. |
| `frame-src` | `'none'` | Blocks embedded frames because the app has no iframe use case. |
| `object-src` | `'none'` | Blocks legacy plugin/object content. |
| `base-uri` | `'self'` | Prevents hostile base URL rewriting. |

Future external services such as map tiles, remote fonts, or third-party dashboards must be added explicitly to the relevant directive and reviewed before shipping.
