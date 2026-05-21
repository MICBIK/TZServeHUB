//! Integration tests for `tauri.conf.json` — CSP and identifier (F-CSP).
//!
//! These tests parse the on-disk JSON to enforce production-ready security
//! posture. They fail until ADR-005 ships.

use serde_json::Value;
use std::fs;
use std::path::PathBuf;

fn load_config() -> Value {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join("tauri.conf.json");
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));
    serde_json::from_str(&raw).expect("tauri.conf.json is invalid JSON")
}

fn load_cargo_toml() -> String {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join("Cargo.toml");
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()))
}

fn csp(cfg: &Value) -> &Value {
    cfg.get("app")
        .and_then(|app| app.get("security"))
        .and_then(|sec| sec.get("csp"))
        .expect("missing app.security.csp")
}

/// CSP-001: identifier must be the production value, not Tauri's template default.
#[test]
fn identifier_is_dev_serverhub_app() {
    let cfg = load_config();
    let id = cfg
        .get("identifier")
        .and_then(|v| v.as_str())
        .expect("identifier missing");
    assert_eq!(
        id, "dev.serverhub.app",
        "identifier must be dev.serverhub.app (was {id})"
    );
    let cargo_toml = load_cargo_toml();
    assert!(
        cargo_toml.contains("name = \"serverhub\""),
        "Cargo package.name must remain serverhub"
    );
}

/// CSP-002: CSP cannot be null in v0.2 and must declare all required directives.
#[test]
fn csp_is_not_null() {
    let cfg = load_config();
    assert!(
        !csp(&cfg).is_null(),
        "app.security.csp must not be null"
    );
    assert!(csp(&cfg).is_object(), "app.security.csp must be an object");
    for directive in [
        "default-src",
        "script-src",
        "style-src",
        "connect-src",
        "img-src",
        "font-src",
        "frame-src",
        "object-src",
        "base-uri",
    ] {
        assert!(
            csp(&cfg).get(directive).is_some(),
            "app.security.csp is missing {directive}"
        );
    }
}

/// CSP-003: default-src is `'self'`.
#[test]
fn csp_default_src_is_self() {
    let cfg = load_config();
    let val = csp(&cfg)
        .get("default-src")
        .and_then(|v| v.as_str())
        .expect("default-src missing");
    assert_eq!(val, "'self'");
}

/// CSP-004: script-src must not contain `'unsafe-inline'` / `'unsafe-eval'`.
#[test]
fn csp_script_src_has_no_unsafe_inline() {
    let cfg = load_config();
    let val = csp(&cfg)
        .get("script-src")
        .and_then(|v| v.as_str())
        .expect("script-src missing");
    assert_eq!(val, "'self'", "script-src must only allow self");
    assert!(
        !val.contains("'unsafe-inline'"),
        "script-src must not include unsafe-inline (was {val})"
    );
    assert!(
        !val.contains("'unsafe-eval'"),
        "script-src must not include unsafe-eval (was {val})"
    );
    assert!(
        !val.contains("http://") && !val.contains("https://"),
        "script-src must not include external hosts (was {val})"
    );
}

/// CSP-005: style-src documented as `'self' 'unsafe-inline'`.
#[test]
fn csp_style_src_includes_unsafe_inline_documented() {
    let cfg = load_config();
    let val = csp(&cfg)
        .get("style-src")
        .and_then(|v| v.as_str())
        .expect("style-src missing");
    assert!(val.contains("'self'"));
    assert!(
        val.contains("'unsafe-inline'"),
        "style-src must include 'unsafe-inline' (Recharts inline SVG styles); see ADR-005"
    );
}

/// CSP-006: connect-src allows Tauri IPC origins.
#[test]
fn csp_connect_src_allows_tauri_ipc() {
    let cfg = load_config();
    let val = csp(&cfg)
        .get("connect-src")
        .and_then(|v| v.as_str())
        .expect("connect-src missing");
    assert!(val.contains("'self'"));
    assert!(val.contains("ipc:"));
    assert!(val.contains("http://ipc.localhost"));
    assert_eq!(
        val, "'self' ipc: http://ipc.localhost",
        "connect-src must not include external hosts"
    );
}

/// CSP-007: frame-src and object-src disabled entirely.
#[test]
fn csp_frame_and_object_src_are_none() {
    let cfg = load_config();
    let frame = csp(&cfg)
        .get("frame-src")
        .and_then(|v| v.as_str())
        .expect("frame-src missing");
    let object = csp(&cfg)
        .get("object-src")
        .and_then(|v| v.as_str())
        .expect("object-src missing");
    assert_eq!(frame, "'none'");
    assert_eq!(object, "'none'");
}
