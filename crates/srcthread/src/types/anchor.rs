//! Anchor pin — whole `.toml` or `+++` TOML frontmatter on `.md`.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Typed anchor fields (same schema for `.toml` and markdown frontmatter).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct AnchorMeta {
    /// Id into `[mirrors]` in store config.
    pub mirror: String,
    /// Pinned commit SHA.
    pub sha: String,
    /// Path inside the mirrored repo.
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_line: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
