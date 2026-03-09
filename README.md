# hegel-rust

A Rust SDK for [Hegel](https://github.com/antithesishq/hegel-core) — universal
property-based testing powered by [Hypothesis](https://hypothesis.works/).

Hegel generates random inputs for your tests, finds failures, and automatically
shrinks them to minimal counterexamples.

## Installation

Add to your `Cargo.toml`:

```toml
[dev-dependencies]
hegel = { git = "ssh://git@github.com/antithesishq/hegel-rust" }
```

### Hegel server

The SDK automatically manages the `hegel` server binary. On first use it
creates a project-local `.hegel/venv` virtualenv and installs the pinned
version of [hegel-core](https://github.com/antithesishq/hegel-core) into it.
Subsequent runs reuse the cached binary unless the pinned version changes.

To use your own `hegel` binary instead (e.g. a local development build), set
the `HEGEL_CMD` environment variable:

```bash
export HEGEL_CMD=/path/to/hegel
```

The SDK requires [`uv`](https://docs.astral.sh/uv/) to be installed for
automatic server management.

## Quick Start

```rust
use hegel::generators::{self, Generate};

#[test]
fn test_addition_commutative() {
    hegel::hegel(|| {
        let x = generators::integers::<i32>().generate();
        let y = generators::integers::<i32>().generate();
        assert_eq!(x + y, y + x);
    });
}
```

Run with `cargo test` as normal. Hegel generates 100 random input pairs and
reports the minimal counterexample if it finds one.

For a full walkthrough, see [docs/getting-started.md](docs/getting-started.md).

## Development

```bash
just setup       # Install dependencies (hegel binary)
just check       # Full CI: lint + docs + tests
just test        # Run tests only
just conformance # Run cross-language conformance tests
```
