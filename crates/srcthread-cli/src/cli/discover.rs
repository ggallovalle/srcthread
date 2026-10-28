//! Active store resolution: `--store` → cwd/`-C` walk → `stores.toml`.

use std::fs;
use std::path::{Path, PathBuf};

use srcthread::{Srcthread, resolve_store_root};
use toml::Value;

use crate::cli::ctx;

#[derive(Debug)]
pub(crate) enum DiscoverError {
    Message(String),
}

impl std::fmt::Display for DiscoverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Message(m) => f.write_str(m),
        }
    }
}

/// Resolve the active store root and open it.
pub(crate) fn open_active_store() -> Result<Srcthread, DiscoverError> {
    let root = resolve_active_store_root()?;
    Srcthread::open(&root).map_err(|e| DiscoverError::Message(e.to_string()))
}

pub(crate) fn resolve_active_store_root() -> Result<PathBuf, DiscoverError> {
    if let Some(spec) = ctx::store_override() {
        return resolve_store_spec(spec);
    }
    let cwd = std::env::current_dir().map_err(|e| DiscoverError::Message(format!("cwd: {e}")))?;
    if let Some(root) = walk_for_store(&cwd) {
        return Ok(root);
    }
    if let Some(root) = default_from_registry()? {
        return Ok(root);
    }
    Err(DiscoverError::Message(
        "no store found (use --store, -C into a project with srcthread/, or register one in $XDG_CONFIG_HOME/srcthread/stores.toml)"
            .into(),
    ))
}

fn resolve_store_spec(spec: &str) -> Result<PathBuf, DiscoverError> {
    let path = PathBuf::from(spec);
    if path.exists() {
        return resolve_store_root(&path).map_err(|e| DiscoverError::Message(e.to_string()));
    }
    // Known name from registry
    let reg = read_registry()?;
    if let Some(p) = reg.get(spec) {
        return resolve_store_root(p).map_err(|e| DiscoverError::Message(e.to_string()));
    }
    Err(DiscoverError::Message(format!(
        "store not found: {spec} (not a path and not in stores.toml)"
    )))
}

fn walk_for_store(start: &Path) -> Option<PathBuf> {
    let mut cur = start.to_path_buf();
    loop {
        if let Ok(root) = resolve_store_root(&cur) {
            return Some(root);
        }
        if !cur.pop() {
            return None;
        }
    }
}

fn default_from_registry() -> Result<Option<PathBuf>, DiscoverError> {
    let path = ctx::stores_toml_path();
    if !path.is_file() {
        return Ok(None);
    }
    let text =
        fs::read_to_string(&path).map_err(|e| DiscoverError::Message(format!("{path:?}: {e}")))?;
    let value: Value =
        toml::from_str(&text).map_err(|e| DiscoverError::Message(format!("stores.toml: {e}")))?;
    let default_id = value
        .get("defaults")
        .and_then(|d| d.get("store"))
        .and_then(|v| v.as_str());
    let Some(id) = default_id else {
        return Ok(None);
    };
    let Some(entry) = value.get("stores").and_then(|s| s.get(id)) else {
        return Err(DiscoverError::Message(format!(
            "defaults.store={id} missing from [stores] in {}",
            path.display()
        )));
    };
    let Some(p) = entry.get("path").and_then(|v| v.as_str()) else {
        return Err(DiscoverError::Message(format!(
            "stores.{id} missing path in {}",
            path.display()
        )));
    };
    let expanded = expand_tilde(p);
    resolve_store_root(&expanded)
        .map(Some)
        .map_err(|e| DiscoverError::Message(e.to_string()))
}

fn read_registry() -> Result<std::collections::BTreeMap<String, PathBuf>, DiscoverError> {
    Ok(list_registry_stores()?.into_iter().collect())
}

/// All `[stores.<id>]` entries from the user registry.
pub(crate) fn list_registry_stores() -> Result<Vec<(String, PathBuf)>, DiscoverError> {
    let path = ctx::stores_toml_path();
    let mut out = Vec::new();
    if !path.is_file() {
        return Ok(out);
    }
    let text =
        fs::read_to_string(&path).map_err(|e| DiscoverError::Message(format!("{path:?}: {e}")))?;
    let value: Value =
        toml::from_str(&text).map_err(|e| DiscoverError::Message(format!("stores.toml: {e}")))?;
    let Some(stores) = value.get("stores").and_then(|v| v.as_table()) else {
        return Ok(out);
    };
    for (id, entry) in stores {
        if let Some(p) = entry.get("path").and_then(|v| v.as_str()) {
            out.push((id.clone(), expand_tilde(p)));
        }
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(out)
}

/// Look up a registered store path by id.
pub(crate) fn registry_path_for(id: &str) -> Result<Option<PathBuf>, String> {
    read_registry()
        .map(|m| m.get(id).cloned())
        .map_err(|e| e.to_string())
}

/// Upsert `[stores.<id>].path` in the user registry.
pub(crate) fn upsert_registry_store(id: &str, root: &Path) -> Result<(), String> {
    let path = ctx::stores_toml_path();
    let mut value = load_or_empty_registry(&path)?;
    let root_str = root.display().to_string();
    let stores = value
        .as_table_mut()
        .ok_or_else(|| "stores.toml root must be a table".to_string())?
        .entry("stores".to_string())
        .or_insert_with(|| Value::Table(Default::default()))
        .as_table_mut()
        .ok_or_else(|| "[stores] must be a table".to_string())?;
    let entry = stores
        .entry(id.to_string())
        .or_insert_with(|| Value::Table(Default::default()))
        .as_table_mut()
        .ok_or_else(|| format!("[stores.{id}] must be a table"))?;
    entry.insert("path".into(), Value::String(root_str));
    write_registry(&path, &value)
}

/// Set `[defaults].store`.
pub(crate) fn set_default_store(id: &str) -> Result<(), String> {
    let path = ctx::stores_toml_path();
    let mut value = load_or_empty_registry(&path)?;
    let defaults = value
        .as_table_mut()
        .ok_or_else(|| "stores.toml root must be a table".to_string())?
        .entry("defaults".to_string())
        .or_insert_with(|| Value::Table(Default::default()))
        .as_table_mut()
        .ok_or_else(|| "[defaults] must be a table".to_string())?;
    defaults.insert("store".into(), Value::String(id.to_string()));
    write_registry(&path, &value)
}

fn load_or_empty_registry(path: &Path) -> Result<Value, String> {
    if !path.is_file() {
        return Ok(Value::Table(Default::default()));
    }
    let text = fs::read_to_string(path).map_err(|e| format!("{path:?}: {e}"))?;
    toml::from_str(&text).map_err(|e| format!("stores.toml: {e}"))
}

fn write_registry(path: &Path, value: &Value) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("{parent:?}: {e}"))?;
    }
    let body = toml::to_string_pretty(value).map_err(|e| e.to_string())?;
    fs::write(path, body).map_err(|e| format!("{path:?}: {e}"))
}

fn expand_tilde(p: &str) -> PathBuf {
    if p == "~"
        && let Some(home) = std::env::var_os("HOME")
    {
        return PathBuf::from(home);
    }
    if let Some(rest) = p.strip_prefix("~/")
        && let Some(home) = std::env::var_os("HOME")
    {
        return PathBuf::from(home).join(rest);
    }
    PathBuf::from(p)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expand_tilde_home() {
        // smoke: non-tilde unchanged
        assert_eq!(expand_tilde("/tmp/x"), PathBuf::from("/tmp/x"));
    }
}
