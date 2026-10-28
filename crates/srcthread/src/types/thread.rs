//! `threads/<id>/thread.toml`.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Thread metadata (prose lives in `notes.md`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ThreadMeta {
    pub id: String,
    pub title: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    /// ISO-8601 timestamp.
    pub created: String,
    /// ISO-8601 timestamp.
    pub updated: String,
}
