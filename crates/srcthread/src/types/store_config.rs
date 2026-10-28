//! `srcthread/config.toml` — `[store]` + `[mirrors]`.

use std::collections::BTreeMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Root of `config.toml`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct StoreConfig {
    /// Human / registry metadata.
    pub store: StoreMeta,
    /// Mirror id → clone URL. Pins (sha/path) live on anchors.
    #[serde(default)]
    pub mirrors: BTreeMap<String, MirrorSpec>,
}

/// `[store]` table.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct StoreMeta {
    /// Required display name.
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authors: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
}

/// One `[mirrors.<id>]` entry — repo identity only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct MirrorSpec {
    /// Git clone URL.
    pub git: String,
}
