//! Process-wide CLI context (globals applied in `main` before subcommands).

use std::path::PathBuf;
use std::sync::OnceLock;

static STORE_OVERRIDE: OnceLock<Option<String>> = OnceLock::new();

/// Record `--store` from the root CLI (call once at startup).
pub(crate) fn set_store_override(store: Option<String>) {
    let _ = STORE_OVERRIDE.set(store);
}

pub(crate) fn store_override() -> Option<&'static str> {
    STORE_OVERRIDE.get().and_then(|s| s.as_deref())
}

/// XDG config home for the store registry.
pub(crate) fn xdg_config_home() -> PathBuf {
    if let Some(p) = std::env::var_os("XDG_CONFIG_HOME") {
        return PathBuf::from(p);
    }
    dirs_fallback_home().join(".config")
}

fn dirs_fallback_home() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
}

pub(crate) fn stores_toml_path() -> PathBuf {
    xdg_config_home().join("srcthread").join("stores.toml")
}
