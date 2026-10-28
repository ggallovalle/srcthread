//! JSON Schema export (schemars) for typed store surfaces.

use schemars::Schema;

use crate::types::{AnchorMeta, ExcerptMeta, RefMeta, StoreConfig, ThreadMeta};

/// Schema for `srcthread/config.toml`.
pub fn store_config_schema() -> Schema {
    schemars::schema_for!(StoreConfig)
}

/// Schema for `thread.toml`.
pub fn thread_meta_schema() -> Schema {
    schemars::schema_for!(ThreadMeta)
}

/// Schema for anchor `.toml` / `+++` frontmatter.
pub fn anchor_meta_schema() -> Schema {
    schemars::schema_for!(AnchorMeta)
}

/// Schema for standalone excerpt frontmatter.
pub fn excerpt_meta_schema() -> Schema {
    schemars::schema_for!(ExcerptMeta)
}

/// Schema for `refs/<id>.toml`.
pub fn ref_meta_schema() -> Schema {
    schemars::schema_for!(RefMeta)
}
