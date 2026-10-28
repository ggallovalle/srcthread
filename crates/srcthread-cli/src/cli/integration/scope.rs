//! `--user` / `--local` install scope (usage-rs ArgGroup).

use std::path::PathBuf;

use usage_rs::ArgGroup;

/// Where to read/write an agent's MCP config.
///
/// Exactly one of these flags must be given on install/uninstall; status
/// may omit the group to report both.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ArgGroup)]
pub(crate) enum Scope {
    /// Write into the user/home agent config
    User,
    /// Write into the project (cwd) agent config
    Local,
}

impl Scope {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Local => "local",
        }
    }
}

/// Process roots used to resolve config paths (overridable in tests).
#[derive(Clone, Debug)]
pub(crate) struct Roots {
    pub(crate) home: PathBuf,
    pub(crate) config_home: PathBuf,
    pub(crate) cwd: PathBuf,
}

impl Roots {
    pub(crate) fn from_env() -> Result<Self, String> {
        let home = std::env::var_os("HOME")
            .map(PathBuf::from)
            .ok_or_else(|| "HOME is not set".to_string())?;
        let config_home = std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| home.join(".config"));
        let cwd = std::env::current_dir().map_err(|e| format!("current directory: {e}"))?;
        Ok(Self {
            home,
            config_home,
            cwd,
        })
    }

    #[cfg(test)]
    pub(crate) fn for_test(root: &std::path::Path) -> Self {
        Self {
            home: root.join("home"),
            config_home: root.join("config"),
            cwd: root.join("project"),
        }
    }
}
