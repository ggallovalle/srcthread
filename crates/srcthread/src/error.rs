//! Library errors.

use std::path::PathBuf;

use thiserror::Error;

/// Errors from opening or reading a file-canonical store.
#[derive(Debug, Error)]
pub enum Error {
    /// Thread id already exists (in-memory hello-world).
    #[error("thread already exists: {0}")]
    ThreadExists(String),
    /// Empty thread id.
    #[error("thread id must not be empty")]
    EmptyThreadId,
    /// Path is not a store (no `config.toml` / `srcthread/config.toml`).
    #[error("not a srcthread store: {0}")]
    NotAStore(PathBuf),
    /// Named thread missing on disk.
    #[error("thread not found: {0}")]
    ThreadNotFound(String),
    /// I/O failure.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    /// TOML parse / serialize failure.
    #[error("toml error: {0}")]
    Toml(String),
    /// JSON / schema failure.
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}

impl Error {
    pub(crate) fn toml(err: impl ToString) -> Self {
        Self::Toml(err.to_string())
    }
}

/// Result alias for the engine.
pub type Result<T> = std::result::Result<T, Error>;
