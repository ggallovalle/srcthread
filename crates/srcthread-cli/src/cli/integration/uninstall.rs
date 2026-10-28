use std::process::ExitCode;

use usage_rs::{Args, Run};

use super::mcp_config::{self, UninstallOutcome};
use super::scope::{Roots, Scope};
use super::targets::Target;

/// Uninstall an MCP integration for TARGET
#[derive(Args)]
#[usage(verbatim_doc_comment)]
pub(crate) struct Uninstall {
    /// Agent target: cursor, claude, opencode, or codex
    pub(crate) target: Target,

    #[usage(arg_group)]
    pub(crate) scope: Scope,
}

impl Run for Uninstall {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        match run_uninstall(self.target, self.scope) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("srcthread: {e}");
                ExitCode::from(1)
            }
        }
    }
}

pub(crate) fn run_uninstall(target: Target, scope: Scope) -> Result<(), String> {
    let roots = Roots::from_env()?;
    uninstall_with(&roots, target, scope)
}

pub(crate) fn uninstall_with(roots: &Roots, target: Target, scope: Scope) -> Result<(), String> {
    let path = target.config_path(scope, roots);
    let outcome = mcp_config::uninstall(&path, target.format())?;
    match outcome {
        UninstallOutcome::Removed => {
            println!(
                "removed {} ({}) MCP from {}",
                target.as_str(),
                scope.as_str(),
                path.display()
            );
        }
        UninstallOutcome::NotPresent => {
            println!(
                "no {} ({}) MCP entry at {}",
                target.as_str(),
                scope.as_str(),
                path.display()
            );
        }
    }
    Ok(())
}
