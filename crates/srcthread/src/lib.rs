//! srcthread engine library.
//!
//! This crate is the only source-of-truth engine for the project.
//! The daemon, CLI, MCP server, and desktop UI all call into this library;
//! they do not own persistence or domain logic themselves.
//!
//! Hello-world builds expose an in-memory [`Srcthread`] / [`Store`] facade.

/// In-memory store handle (hello-world stub).
#[derive(Debug, Default, Clone)]
pub struct Store {
    demo_thread: Option<String>,
}

/// Facade over a [`Store`]. Created with [`Srcthread::open_memory`] for tests and local use.
#[derive(Debug, Default, Clone)]
pub struct Srcthread {
    store: Store,
}

impl Srcthread {
    /// Open an in-memory engine (no disk). Suitable for hello-world and unit tests.
    pub fn open_memory() -> Self {
        Self {
            store: Store {
                demo_thread: Some("demo-thread".to_string()),
            },
        }
    }

    /// Returns the project greeting token.
    pub fn hello(&self) -> &'static str {
        "srcthread"
    }

    /// List known thread ids (demo stub may return one id or empty).
    pub fn list_threads(&self) -> Vec<String> {
        self.store.demo_thread.iter().cloned().collect()
    }

    /// Borrow the underlying store.
    pub fn store(&self) -> &Store {
        &self.store
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn open_memory_hello_and_list() {
        let eng = Srcthread::open_memory();
        assert_eq!(eng.hello(), "srcthread");
        let threads = eng.list_threads();
        assert!(!threads.is_empty());
        assert_eq!(threads[0], "demo-thread");
    }
}
