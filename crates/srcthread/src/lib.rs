//! srcthread engine library.
//!
//! This crate is the only source-of-truth engine for the project.
//! The daemon, CLI, MCP server, and desktop UI all call into this library;
//! they do not own persistence or domain logic themselves.
//!
//! Hello-world builds expose an in-memory [`Srcthread`] / [`Store`] facade
//! with at least one **query** ([`Srcthread::list_threads`]) and one
//! **mutation** ([`Srcthread::create_thread`]).

use std::sync::{Arc, Mutex};

use thiserror::Error;

/// Errors from the in-memory hello-world store.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum Error {
    /// Thread id already exists.
    #[error("thread already exists: {0}")]
    ThreadExists(String),
    /// Empty thread id.
    #[error("thread id must not be empty")]
    EmptyThreadId,
}

/// In-memory store handle (hello-world stub).
#[derive(Debug, Default)]
pub struct Store {
    threads: Vec<String>,
}

/// Facade over a [`Store`]. Created with [`Srcthread::open_memory`] for tests
/// and local use. Cheap to clone: shares the store via [`Arc`].
#[derive(Debug, Clone)]
pub struct Srcthread {
    store: Arc<Mutex<Store>>,
}

impl Default for Srcthread {
    fn default() -> Self {
        Self::open_memory()
    }
}

impl Srcthread {
    /// Open an in-memory engine (no disk). Suitable for hello-world and unit tests.
    pub fn open_memory() -> Self {
        Self {
            store: Arc::new(Mutex::new(Store {
                threads: vec!["demo-thread".to_string()],
            })),
        }
    }

    /// Returns the project greeting token (query).
    pub fn hello(&self) -> &'static str {
        "srcthread"
    }

    /// List known thread ids (query).
    pub fn list_threads(&self) -> Vec<String> {
        self.store.lock().expect("store lock").threads.clone()
    }

    /// Create a research thread id (mutation).
    pub fn create_thread(&self, id: impl Into<String>) -> Result<(), Error> {
        let id = id.into();
        if id.trim().is_empty() {
            return Err(Error::EmptyThreadId);
        }
        let mut store = self.store.lock().expect("store lock");
        if store.threads.iter().any(|t| t == &id) {
            return Err(Error::ThreadExists(id));
        }
        store.threads.push(id);
        Ok(())
    }

    /// Borrow a snapshot of the underlying store (clone of thread ids).
    pub fn store(&self) -> StoreSnapshot {
        StoreSnapshot {
            threads: self.list_threads(),
        }
    }
}

/// Read-only snapshot of store contents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreSnapshot {
    /// Thread ids currently in the store.
    pub threads: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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
        assert_eq!(
            eng.create_thread("from-test"),
            Err(Error::ThreadExists("from-test".into()))
        );
    }
}
