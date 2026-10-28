//! UniFFI surface over the [`srcthread`] engine.
//!
//! Android loads `libsrcthread_uniffi.so` and talks both ways through a
//! per-instance [`SrcthreadFfi`] handle:
//! - Kotlin → Rust: [`SrcthreadFfi::hello`], [`list_threads`], [`create_thread`]
//! - Rust → Kotlin: [`HostCallbacks`] registered on that same handle

use std::sync::{Arc, Mutex};

use srcthread::Srcthread;

uniffi::setup_scaffolding!();

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

/// Per-instance UniFFI handle over an independent in-memory [`Srcthread`].
#[derive(uniffi::Object)]
pub struct SrcthreadFfi {
    inner: Srcthread,
    host: Mutex<Option<Arc<dyn HostCallbacks>>>,
}

#[uniffi::export]
impl SrcthreadFfi {
    /// Open a fresh in-memory engine (independent of any other handle).
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            inner: Srcthread::open_memory(),
            host: Mutex::new(None),
        })
    }

    /// Register Kotlin callbacks for this handle only (Rust → Kotlin).
    pub fn set_host_callbacks(self: Arc<Self>, callbacks: Arc<dyn HostCallbacks>) {
        *self.host.lock().expect("host lock") = Some(callbacks);
    }

    /// Greeting token from this handle's engine (query).
    pub fn hello(self: Arc<Self>) -> String {
        let msg = self.inner.hello().to_string();
        self.notify_host(format!("hello → {msg}"));
        msg
    }

    /// Demo thread ids for this handle (query).
    pub fn list_threads(self: Arc<Self>) -> Vec<String> {
        let threads = self.inner.list_threads();
        self.notify_host(format!("list_threads → {} ids", threads.len()));
        threads
    }

    /// Create a thread id on this handle (mutation).
    pub fn create_thread(self: Arc<Self>, id: String) -> Result<String, ApiError> {
        self.inner.create_thread(id.clone())?;
        self.notify_host(format!("create_thread → {id}"));
        Ok(id)
    }
}

impl SrcthreadFfi {
    fn notify_host(&self, log: impl Into<String>) {
        let log = log.into();
        let threads = self.inner.list_threads();
        if let Some(host) = self.host.lock().expect("host lock").as_ref() {
            host.on_log(log);
            host.on_threads_changed(threads);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[derive(Default)]
    struct RecordingHost {
        threads: Mutex<Vec<String>>,
        logs: Mutex<Vec<String>>,
        changes: AtomicUsize,
    }

    impl HostCallbacks for RecordingHost {
        fn on_threads_changed(&self, threads: Vec<String>) {
            self.changes.fetch_add(1, Ordering::SeqCst);
            *self.threads.lock().expect("threads") = threads;
        }

        fn on_log(&self, message: String) {
            self.logs.lock().expect("logs").push(message);
        }
    }

    #[test]
    fn two_handles_have_independent_stores() {
        let a = SrcthreadFfi::new();
        let b = SrcthreadFfi::new();

        a.clone()
            .create_thread("only-a".into())
            .expect("create on a");

        assert!(
            a.list_threads().contains(&"only-a".to_string()),
            "a should see its own thread"
        );
        assert!(
            !b.list_threads().contains(&"only-a".to_string()),
            "b must not see a's thread"
        );
    }

    #[test]
    fn host_callbacks_are_per_handle() {
        let a = SrcthreadFfi::new();
        let b = SrcthreadFfi::new();
        let host_a = Arc::new(RecordingHost::default());
        let host_b = Arc::new(RecordingHost::default());

        a.clone().set_host_callbacks(host_a.clone());
        b.clone().set_host_callbacks(host_b.clone());

        a.create_thread("from-a".into()).expect("create on a");

        assert_eq!(host_a.changes.load(Ordering::SeqCst), 1);
        assert_eq!(host_b.changes.load(Ordering::SeqCst), 0);
        assert!(
            host_a
                .threads
                .lock()
                .expect("threads")
                .contains(&"from-a".to_string())
        );
        assert!(host_b.threads.lock().expect("threads").is_empty()
            || !host_b
                .threads
                .lock()
                .expect("threads")
                .contains(&"from-a".to_string()));
    }
}
