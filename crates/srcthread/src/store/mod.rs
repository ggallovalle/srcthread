//! File-canonical store open + cold walk.

mod open;

pub(crate) use open::open_disk;
pub use open::{DiskStore, resolve_store_root};
