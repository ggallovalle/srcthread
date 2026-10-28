//! Agent targets and their MCP config paths.

use std::path::PathBuf;
use std::str::FromStr;

use super::scope::{Roots, Scope};

/// Coding agents that can receive a `srcthread mcp` registration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Target {
    Cursor,
    Claude,
    Opencode,
    Codex,
}

impl Target {
    pub(crate) const ALL: [Self; 4] = [Self::Cursor, Self::Claude, Self::Opencode, Self::Codex];

    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Cursor => "cursor",
            Self::Claude => "claude",
            Self::Opencode => "opencode",
            Self::Codex => "codex",
        }
    }

    /// Config file for this target and scope.
    pub(crate) fn config_path(self, scope: Scope, roots: &Roots) -> PathBuf {
        match (self, scope) {
            (Self::Cursor, Scope::User) => roots.home.join(".cursor/mcp.json"),
            (Self::Cursor, Scope::Local) => roots.cwd.join(".cursor/mcp.json"),
            (Self::Claude, Scope::User) => roots.home.join(".claude.json"),
            (Self::Claude, Scope::Local) => roots.cwd.join(".mcp.json"),
            (Self::Opencode, Scope::User) => roots.config_home.join("opencode/opencode.json"),
            (Self::Opencode, Scope::Local) => roots.cwd.join("opencode.json"),
            (Self::Codex, Scope::User) => roots.home.join(".codex/config.toml"),
            (Self::Codex, Scope::Local) => roots.cwd.join(".codex/config.toml"),
        }
    }

    pub(crate) fn format(self) -> ConfigFormat {
        match self {
            Self::Cursor | Self::Claude => ConfigFormat::McpServersJson,
            Self::Opencode => ConfigFormat::OpencodeJson,
            Self::Codex => ConfigFormat::CodexToml,
        }
    }
}

impl FromStr for Target {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cursor" => Ok(Self::Cursor),
            "claude" => Ok(Self::Claude),
            "opencode" => Ok(Self::Opencode),
            "codex" => Ok(Self::Codex),
            other => Err(format!(
                "unknown target `{other}` (expected cursor, claude, opencode, or codex)"
            )),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ConfigFormat {
    /// Cursor / Claude: top-level `mcpServers` object.
    McpServersJson,
    /// OpenCode: top-level `mcp` object with `type`/`command`/`enabled`.
    OpencodeJson,
    /// Codex: `[mcp_servers.<id>]` in TOML.
    CodexToml,
}
