//! `refs/<id>.toml`.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// External ref kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum RefKind {
    Pr,
    Issue,
    Doc,
    Chat,
    Other,
}

/// External reference metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct RefMeta {
    pub kind: RefKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional path relative to the thread dir for a pasted body.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_path: Option<String>,
}
