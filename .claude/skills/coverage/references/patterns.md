# Testability Patterns

## Parameterize over environment

Extract logic from functions that read environment/global state into parameterized functions that take those values as arguments.

```rust
// Hard to test — reads env vars directly
fn cache_dir() -> PathBuf {
    if let Ok(xdg) = std::env::var("XDG_CACHE_HOME") {
        return PathBuf::from(xdg).join("myapp");
    }
    // ...
}

// Testable — takes values as parameters
fn cache_dir_from(xdg: Option<String>, home: Option<PathBuf>) -> PathBuf {
    if let Some(xdg) = xdg {
        return PathBuf::from(xdg).join("myapp");
    }
    // ...
}

// Thin wrapper calls the testable version
fn cache_dir() -> PathBuf {
    cache_dir_from(std::env::var("XDG_CACHE_HOME").ok(), std::env::home_dir())
}
```

## Platform-specific match arms

Take arch/os as parameters so all branches are testable from any platform:

```rust
// Can only test the current platform's branch
fn platform_archive_name() -> Result<String, String> {
    archive_name_for(std::env::consts::ARCH, std::env::consts::OS)
}

// All branches testable
fn archive_name_for(arch: &str, os: &str) -> Result<String, String> {
    match (arch, os) {
        ("aarch64", "macos") => ...,
        ("x86_64", "linux") => ...,
        _ => Err(...),
    }
}
```

