//! UniFFI surface over the [`srcthread`] engine.
//!
//! Android (and later other mobile hosts) load `libsrcthread_uniffi.so` and
//! call the generated Kotlin bindings under `dev.kbroom.srcthread.uniffi`.

uniffi::setup_scaffolding!();

/// Greeting token from the in-memory engine.
#[uniffi::export]
pub fn hello() -> String {
    srcthread::Srcthread::open_memory().hello().to_string()
}

/// Demo thread ids from the in-memory engine.
#[uniffi::export]
pub fn list_threads() -> Vec<String> {
    srcthread::Srcthread::open_memory().list_threads()
}
