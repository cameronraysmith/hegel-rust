# Design: Replace custom uv downloader with official installer script

## Summary

Replace the hand-rolled uv download/extract pipeline in `src/uv.rs` with the official uv installer script (`https://astral.sh/uv/install.sh`), committed to the repository and embedded at compile time via `include_str!`. At runtime, the installer is piped to `sh` via stdin with `UV_UNMANAGED_INSTALL` set to the hegel cache directory.

## Motivation

The current implementation maintains its own:
- Platform-to-archive-name mapping
- SHA-256 checksums per platform
- curl download + tar extraction pipeline

This means manually updating version numbers, checksums, and platform support every uv release. The official installer handles all of this correctly and is maintained by the uv team.

## Architecture

### New file: `src/uv-install.sh`

A committed copy of `https://astral.sh/uv/install.sh`. Embedded at compile time:

```rust
const UV_INSTALLER: &str = include_str!("uv-install.sh");
```

To update uv, replace this file with a new download of the installer.

### Changes to `src/uv.rs`

**Removed:**
- `UV_VERSION` constant
- `expected_sha256` function (per-platform SHA-256 table)
- `compute_sha256`, `compute_sha256_with`, `verify_sha256` functions
- `archive_name_for`, `platform_archive_name` functions
- `download_url_to_cache` function (curl + tar pipeline)
- `CleanupGuard` struct

**Added:**
- `const UV_INSTALLER: &str = include_str!("uv-install.sh")`
- `install_uv_to(cache: &Path)` — replaces `download_uv_to`

**Unchanged:**
- `find_uv`, `resolve_uv`, `find_in_path`, `cache_dir`, `cache_dir_from`, `cached_uv_path`

### `install_uv_to` implementation

```rust
fn install_uv_to(cache: &Path) {
    std::fs::create_dir_all(cache)
        .unwrap_or_else(|e| panic!("Failed to create cache directory {}: {e}", cache.display()));
    let mut child = std::process::Command::new("sh")
        .stdin(std::process::Stdio::piped())
        .env("UV_UNMANAGED_INSTALL", cache)
        .spawn()
        .unwrap_or_else(|e| panic!("Failed to run uv installer: {e}. Install uv manually: https://docs.astral.sh/uv/getting-started/installation/"));
    use std::io::Write;
    child.stdin.take().unwrap().write_all(UV_INSTALLER.as_bytes())
        .unwrap_or_else(|e| panic!("Failed to write to uv installer stdin: {e}"));
    let status = child.wait()
        .unwrap_or_else(|e| panic!("Failed to wait for uv installer: {e}"));
    assert!(status.success(), "uv installer failed. Install uv manually: https://docs.astral.sh/uv/getting-started/installation/");
}
```

`resolve_uv` calls `install_uv_to(&cache)` instead of `download_uv_to(&cache)`.

### Unmanaged install

Setting `UV_UNMANAGED_INSTALL=<dir>` causes the installer to place `uv` and `uvx` binaries directly in `<dir>` without modifying shell profiles or PATH. See: https://docs.astral.sh/uv/reference/installer/#unmanaged-installations

The hegel cache directory (`~/.cache/hegel` or `$XDG_CACHE_HOME/hegel`) is used, so the installed binary ends up at `~/.cache/hegel/uv` — same path as before.

## Tests

**Removed** (~15 tests):
- All SHA-256 tests (`test_verify_sha256_*`, `test_compute_sha256_*`, `test_all_supported_platforms_have_sha256`, `test_expected_sha256_*`)
- All archive/platform tests (`test_all_supported_platforms_have_real_release_archives`, `test_all_release_archives_are_covered`, `test_unsupported_platform_returns_error`)
- All download pipeline tests (`test_download_and_extract_pipeline`, `test_download_with_sha256_*`, `test_download_invalid_*`, `test_download_bad_url`, `test_cleanup_guard_removes_directory`)

**Kept:**
- `test_cache_dir_with_xdg`
- `test_cache_dir_with_home`
- `test_find_in_path_finds_known_binary`
- `test_find_in_path_returns_none_for_missing`
- `test_resolve_uv_prefers_path`
- `test_resolve_uv_uses_cache`
- `test_resolve_uv_downloads_from_github` (now exercises the installer path)

## Error handling

All failure modes panic with a message pointing to the manual installation docs. This is consistent with the current behaviour — uv is a hard dependency, so there is no meaningful fallback.
