//! Standalone excerpt frontmatter (`excerpts/<id>.md`).

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// TOML frontmatter for a standalone excerpt file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ExcerptMeta {
    pub id: String,
    /// Anchor id this excerpt freezes.
    pub anchor_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
