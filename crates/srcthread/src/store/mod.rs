//! File-canonical store open + cold walk + mutations.

mod mutate;
mod open;
mod thread;

pub use mutate::init_store;
pub(crate) use open::open_disk;
pub use open::{DiskStore, resolve_store_root};
pub use thread::{LoadedAnchor, LoadedExcerpt, LoadedRef, LoadedThread, load_thread_dir};
