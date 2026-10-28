//! On-disk serde types (and schemars) for the file-canonical store.

mod anchor;
mod excerpt;
mod r#ref;
mod store_config;
mod thread;

pub use anchor::AnchorMeta;
pub use excerpt::ExcerptMeta;
pub use r#ref::{RefKind, RefMeta};
pub use store_config::{MirrorSpec, StoreConfig, StoreMeta};
pub use thread::ThreadMeta;
