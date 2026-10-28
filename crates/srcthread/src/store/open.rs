//! Resolve and open a store directory.

use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{Error, Result};
use crate::store::thread::{LoadedThread, load_thread_dir};
use crate::types::{StoreConfig, ThreadMeta};

/// On-disk store handle (config loaded; threads cold-walked on demand).
#[derive(Debug, Clone)]
pub struct DiskStore {
    pub(crate) root: PathBuf,
    pub(crate) config: StoreConfig,
}

impl DiskStore {
    /// Absolute path to the store root (directory containing `config.toml`).
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Parsed `config.toml`.
    pub fn config(&self) -> &StoreConfig {
        &self.config
    }

    /// Cold-walk `threads/*/thread.toml`.
    pub fn list_threads(&self) -> Result<Vec<ThreadMeta>> {
        let threads_dir = self.root.join("threads");
        if !threads_dir.is_dir() {
            return Ok(Vec::new());
        }
        let mut out = Vec::new();
        for entry in fs::read_dir(&threads_dir)? {
            let entry = entry?;
            if !entry.file_type()?.is_dir() {
                continue;
            }
            let meta_path = entry.path().join("thread.toml");
            if !meta_path.is_file() {
                continue;
            }
            let text = fs::read_to_string(&meta_path)?;
            let meta: ThreadMeta = toml::from_str(&text).map_err(Error::toml)?;
            out.push(meta);
        }
        out.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(out)
    }

    /// Load one thread by id (directory name / `thread.toml` id).
    pub fn get_thread(&self, id: &str) -> Result<ThreadMeta> {
        let meta_path = self.root.join("threads").join(id).join("thread.toml");
        if !meta_path.is_file() {
            return Err(Error::ThreadNotFound(id.to_string()));
        }
        let text = fs::read_to_string(meta_path)?;
        toml::from_str(&text).map_err(Error::toml)
    }

    /// Load anchors, excerpts, refs, and notes for a thread.
    pub fn load_thread(&self, id: &str) -> Result<LoadedThread> {
        let dir = self.root.join("threads").join(id);
        if !dir.is_dir() {
            return Err(Error::ThreadNotFound(id.to_string()));
        }
        load_thread_dir(&dir)
    }

    /// Resolve a mirror id to its git URL from config.
    pub fn mirror_git(&self, id: &str) -> Option<&str> {
        self.config.mirrors.get(id).map(|m| m.git.as_str())
    }
}

/// Resolve `path` to a store root that contains `config.toml`.
///
/// Accepts either the store directory itself or a project directory that
/// contains `srcthread/config.toml`.
pub fn resolve_store_root(path: &Path) -> Result<PathBuf> {
    let direct = path.join("config.toml");
    if direct.is_file() {
        return Ok(path.to_path_buf());
    }
    let nested = path.join("srcthread").join("config.toml");
    if nested.is_file() {
        return Ok(path.join("srcthread"));
    }
    Err(Error::NotAStore(path.to_path_buf()))
}

pub(crate) fn open_disk(path: &Path) -> Result<DiskStore> {
    let root = resolve_store_root(path)?;
    let text = fs::read_to_string(root.join("config.toml"))?;
    let config: StoreConfig = toml::from_str(&text).map_err(Error::toml)?;
    Ok(DiskStore { root, config })
}
