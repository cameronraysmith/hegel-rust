RELEASE_TYPE: minor

Manage hegel binary via pinned .hegel/venv instead of relying on system installation.

- `HEGEL_VERSION` is now a plain git commit hash for hegel-core, hard-coded in `runner.rs`
- On first use, creates a project-local `.hegel/venv` virtualenv via `uv venv` and installs hegel into it via `uv pip install`
- A `.hegel/venv/hegel-version` file tracks the installed hash; subsequent runs skip installation if it matches `HEGEL_VERSION`
- On version mismatch the venv is recreated and hegel is reinstalled
- `HEGEL_CMD` environment variable overrides to a specific binary path, skipping installation entirely
- The release script pins `HEGEL_VERSION` to the current hegel-core main HEAD SHA at release time via `gh api`
