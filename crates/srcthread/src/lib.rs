//! srcthread engine library.
//!
//! Sole source-of-truth for store I/O and domain types. CLI, MCP, desktop,
//! nvim, and Android call this in-process.

mod error;
pub mod schema;
mod store;
pub mod types;

use std::path::Path;
use std::sync::{Arc, Mutex};

pub use error::{Error, Result};
pub use store::{DiskStore, resolve_store_root};
pub use types::*;

/// In-memory hello-world store (tests / surfaces not yet on disk).
#[derive(Debug, Default)]
struct MemoryInner {
    threads: Vec<String>,
}

enum Backend {
    Memory(Arc<Mutex<MemoryInner>>),
    Disk(DiskStore),
}

/// Facade over an in-memory or on-disk store.
#[derive(Clone)]
pub struct Srcthread {
    backend: Arc<Backend>,
}

impl Default for Srcthread {
    fn default() -> Self {
        Self::open_memory()
    }
}

impl std::fmt::Debug for Srcthread {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self.backend {
            Backend::Memory(_) => f.write_str("Srcthread(Memory)"),
            Backend::Disk(d) => f.debug_tuple("Srcthread").field(&d.root()).finish(),
        }
    }
}

impl Srcthread {
    /// Open an in-memory engine (no disk).
    pub fn open_memory() -> Self {
        Self {
            backend: Arc::new(Backend::Memory(Arc::new(Mutex::new(MemoryInner {
                threads: vec!["demo-thread".to_string()],
            })))),
        }
    }

    /// Open a file-canonical store at `path` (store root or project containing
    /// `srcthread/`).
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let disk = store::open_disk(path.as_ref())?;
        Ok(Self {
            backend: Arc::new(Backend::Disk(disk)),
        })
    }

    /// Returns the project greeting token.
    pub fn hello(&self) -> &'static str {
        "srcthread"
    }

    /// On-disk store handle when opened from disk.
    pub fn disk(&self) -> Option<&DiskStore> {
        match &*self.backend {
            Backend::Disk(d) => Some(d),
            Backend::Memory(_) => None,
        }
    }

    /// List thread ids (memory) or full [`ThreadMeta`] via [`Self::list_thread_meta`].
    pub fn list_threads(&self) -> Vec<String> {
        match &*self.backend {
            Backend::Memory(m) => m.lock().expect("store lock").threads.clone(),
            Backend::Disk(d) => d
                .list_threads()
                .unwrap_or_default()
                .into_iter()
                .map(|t| t.id)
                .collect(),
        }
    }

    /// List thread metadata from disk (empty on memory backend).
    pub fn list_thread_meta(&self) -> Result<Vec<ThreadMeta>> {
        match &*self.backend {
            Backend::Memory(m) => Ok(m
                .lock()
                .expect("store lock")
                .threads
                .iter()
                .map(|id| ThreadMeta {
                    id: id.clone(),
                    title: id.clone(),
                    tags: Vec::new(),
                    created: String::new(),
                    updated: String::new(),
                })
                .collect()),
            Backend::Disk(d) => d.list_threads(),
        }
    }

    /// Get one thread by id.
    pub fn get_thread(&self, id: &str) -> Result<ThreadMeta> {
        match &*self.backend {
            Backend::Memory(m) => {
                let threads = m.lock().expect("store lock");
                if threads.threads.iter().any(|t| t == id) {
                    Ok(ThreadMeta {
                        id: id.to_string(),
                        title: id.to_string(),
                        tags: Vec::new(),
                        created: String::new(),
                        updated: String::new(),
                    })
                } else {
                    Err(Error::ThreadNotFound(id.to_string()))
                }
            }
            Backend::Disk(d) => d.get_thread(id),
        }
    }

    /// Create a research thread id (memory only for now).
    pub fn create_thread(&self, id: impl Into<String>) -> Result<()> {
        let id = id.into();
        if id.trim().is_empty() {
            return Err(Error::EmptyThreadId);
        }
        match &*self.backend {
            Backend::Memory(m) => {
                let mut store = m.lock().expect("store lock");
                if store.threads.iter().any(|t| t == &id) {
                    return Err(Error::ThreadExists(id));
                }
                store.threads.push(id);
                Ok(())
            }
            Backend::Disk(_) => Err(Error::Toml(
                "create_thread on disk not implemented yet".into(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::path::PathBuf;

    fn repo_seed() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
    }

    #[test]
    fn open_memory_hello_list_and_create() {
        let eng = Srcthread::open_memory();
        assert_eq!(eng.hello(), "srcthread");
        assert_eq!(eng.list_threads(), vec!["demo-thread".to_string()]);
        eng.create_thread("from-test").unwrap();
        assert_eq!(
            eng.list_threads(),
            vec!["demo-thread".to_string(), "from-test".to_string()]
        );
        assert!(matches!(
            eng.create_thread("from-test"),
            Err(Error::ThreadExists(_))
        ));
    }

    #[test]
    fn open_seed_lists_deno_thread() {
        let eng = Srcthread::open(repo_seed()).expect("open seed store");
        let ids = eng.list_threads();
        assert!(
            ids.iter().any(|id| id == "deno-env-get-01HK3K6N8R"),
            "ids={ids:?}"
        );
        let meta = eng.get_thread("deno-env-get-01HK3K6N8R").unwrap();
        assert_eq!(meta.title, "How is Deno.env.get implemented?");
        let cfg = eng.disk().unwrap().config();
        assert_eq!(cfg.store.name, "srcthread dogfood");
        assert!(cfg.mirrors.contains_key("deno"));
    }
}
