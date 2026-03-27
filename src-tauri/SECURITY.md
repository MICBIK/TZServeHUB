# Security Audit Report

**Audit Date:** 2026-03-19
**Tool:** cargo-audit v0.22.1
**Advisory Database:** RustSec (951 advisories loaded)

## Summary

- **Vulnerabilities Found:** 1 (Medium Severity)
- **Unmaintained/Unsound Warnings:** 18

---

## Critical Findings

### 1. RSA Timing Sidechannel Vulnerability (RUSTSEC-2023-0071)

**Severity:** 5.9 (Medium)
**Crate:** `rsa 0.9.10`
**Issue:** Marvin Attack - potential key recovery through timing sidechannels
**Status:** ⚠️ No fixed upgrade available
**URL:** https://rustsec.org/advisories/RUSTSEC-2023-0071

**Dependency Chain:**
```
rsa 0.9.10
└── sqlx-mysql 0.8.6
    └── sqlx 0.8.6
        └── serverhub 0.1.0
```

**Impact:** This vulnerability affects MySQL connections through sqlx. The RSA implementation used for authentication may leak timing information that could theoretically be exploited to recover private keys.

**Mitigation Options:**
1. Monitor RustSec and sqlx releases for updates
2. Consider alternative database drivers if MySQL RSA authentication is critical
3. Use network-level encryption (TLS) to reduce attack surface
4. Evaluate if MySQL authentication method can be changed to avoid RSA

---

## Maintenance Warnings

### GTK3 Bindings (Unmaintained)

Multiple GTK3-related crates are no longer maintained as the gtk-rs project has moved to GTK4:

- `atk 0.18.2` (RUSTSEC-2024-0413)
- `atk-sys 0.18.2` (RUSTSEC-2024-0416)
- `gdk 0.18.2` (RUSTSEC-2024-0412)
- `gdk-pixbuf 0.18.2` (RUSTSEC-2024-0414)
- `gdk-pixbuf-sys 0.18.2` (RUSTSEC-2024-0417)
- `gdk-sys 0.18.2` (RUSTSEC-2024-0415)
- `gio 0.18.4` (RUSTSEC-2024-0411)
- `gio-sys 0.18.2` (RUSTSEC-2024-0418)
- `glib 0.18.5` (RUSTSEC-2024-0410) - Also has unsoundness issue (RUSTSEC-2024-0429)
- `glib-sys 0.18.2` (RUSTSEC-2024-0419)
- `gobject-sys 0.18.2` (RUSTSEC-2024-0420)
- `gtk 0.18.2` (RUSTSEC-2024-0409)
- `gtk-sys 0.18.2` (RUSTSEC-2024-0421)
- `pango 0.18.3` (RUSTSEC-2024-0408)
- `pango-sys 0.18.2` (RUSTSEC-2024-0422)

**Source:** Tauri's Linux rendering dependencies (webkit2gtk, wry, tao, muda, tray-icon)

**Recommendation:** These are transitive dependencies from Tauri. Monitor Tauri releases for GTK4 migration. No immediate action required unless specific GTK3 vulnerabilities are discovered.

### Other Unmaintained Crates

- `fxhash 0.2.1` (RUSTSEC-2025-0057) - Used by tauri-utils via selectors/kuchikiki
- `unic-ucd-ident 0.9.0` (RUSTSEC-2025-0100) - Used by urlpattern in tauri-utils
- `unic-ucd-version 0.9.0` (RUSTSEC-2025-0098) - Dependency of unic-ucd-ident

**Recommendation:** These are deep transitive dependencies. Monitor Tauri updates for replacements.

### Unsoundness Issue

- `glib 0.18.5` (RUSTSEC-2024-0429) - Unsound `Iterator` and `DoubleEndedIterator` implementations for `glib::VariantStrIter`

**Impact:** Could cause undefined behavior if `VariantStrIter` is used directly. Likely low risk as this is a transitive dependency.

---

## Action Items

1. **Immediate:** Document this report and monitor for sqlx/rsa updates
2. **Short-term:** Check if MySQL authentication can avoid RSA (use caching_sha2_password or similar)
3. **Medium-term:** Monitor Tauri releases for GTK4 migration and dependency updates
4. **Ongoing:** Run `cargo audit` regularly (recommend: before each release)

---

## Audit Command

```bash
cd src-tauri && cargo audit
```

To update the advisory database:
```bash
cargo audit --update-only
```
