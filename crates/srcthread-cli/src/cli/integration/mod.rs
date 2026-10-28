//! Agent MCP integrations (`install` / `uninstall` / `status`).
//!
//! Registers `srcthread mcp` in each agent's config. Scope is `--user`
//! (home) or `--local` (project / cwd). See `research/09-mise-packaging-plugin.md`.

mod install;
mod mcp_config;
mod scope;
mod status;
mod targets;
mod uninstall;

#[cfg(test)]
mod tests;

use std::process::ExitCode;

use usage_rs::{Args, Run, Subcommands};

/// Manage built-in agent MCP integrations
///
/// Install, uninstall, or show status for registering `srcthread mcp` in
/// coding-agent configs (Cursor, Claude, OpenCode, Codex).
#[derive(Args)]
#[usage(verbatim_doc_comment)]
pub(crate) struct Integration {
    #[usage(subcommand)]
    command: Commands,
}

#[derive(Subcommands)]
#[usage(run)]
enum Commands {
    /// Install an MCP integration
    Install(install::Install),
    /// Uninstall an MCP integration
    Uninstall(uninstall::Uninstall),
    /// Show integration status
    Status(status::Status),
}

impl Run for Integration {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        self.command.run()
    }
}
