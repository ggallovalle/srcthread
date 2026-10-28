//! UniFFI surface over the [`srcthread`] engine.
//!
//! Android loads `libsrcthread_uniffi.so` and talks both ways:
//! - Kotlin → Rust: [`hello`], [`list_threads`], [`create_thread`]
//! - Rust → Kotlin: [`HostCallbacks`] implemented on the Kotlin side

use std::sync::{Arc, Mutex, OnceLock};

use srcthread::Srcthread;

uniffi::setup_scaffolding!();

fn engine() -> &'static Srcthread {
    static ENGINE: OnceLock<Srcthread> = OnceLock::new();
    ENGINE.get_or_init(Srcthread::open_memory)
}

/// Errors raised across the UniFFI boundary.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error, uniffi::Error)]
pub enum ApiError {
    /// Human-readable failure from the engine.
    #[error("{msg}")]
    Failed { msg: String },
}

impl From<srcthread::Error> for ApiError {
    fn from(value: srcthread::Error) -> Self {
        Self::Failed {
            msg: value.to_string(),
        }
    }
}

/// Foreign (Kotlin) callbacks Rust can invoke.
#[uniffi::export(with_foreign)]
pub trait HostCallbacks: Send + Sync {
    /// Called after a successful mutation so the UI can refresh.
    fn on_threads_changed(&self, threads: Vec<String>);
    /// Optional log line from Rust.
    fn on_log(&self, message: String);
}

static HOST: OnceLock<Mutex<Option<Arc<dyn HostCallbacks>>>> = OnceLock::new();

fn host_slot() -> &'static Mutex<Option<Arc<dyn HostCallbacks>>> {
    HOST.get_or_init(|| Mutex::new(None))
}

/// Register Kotlin callbacks (Rust → Kotlin direction).
#[uniffi::export]
pub fn set_host_callbacks(callbacks: Arc<dyn HostCallbacks>) {
    *host_slot().lock().expect("host lock") = Some(callbacks);
}

fn notify_host(log: impl Into<String>) {
    let log = log.into();
    let threads = engine().list_threads();
    if let Some(host) = host_slot().lock().expect("host lock").as_ref() {
        host.on_log(log);
        host.on_threads_changed(threads);
    }
}

/// Greeting token from the in-memory engine (query).
#[uniffi::export]
pub fn hello() -> String {
    let msg = engine().hello().to_string();
    notify_host(format!("hello → {msg}"));
    msg
}

/// Demo thread ids (query).
#[uniffi::export]
pub fn list_threads() -> Vec<String> {
    let threads = engine().list_threads();
    notify_host(format!("list_threads → {} ids", threads.len()));
    threads
}

/// Create a thread id (mutation).
#[uniffi::export]
pub fn create_thread(id: String) -> Result<String, ApiError> {
    engine().create_thread(id.clone())?;
    notify_host(format!("create_thread → {id}"));
    Ok(id)
}
