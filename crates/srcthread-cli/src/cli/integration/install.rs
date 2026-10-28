use std::process::ExitCode;

use usage_rs::{Args, Run};

use super::mcp_config::{self, InstallOutcome};
use super::scope::{Roots, Scope};
use super::targets::Target;

/// Install an MCP integration for TARGET
#[derive(Args)]
#[usage(verbatim_doc_comment)]
pub(crate) struct Install {
    /// Agent target: cursor, claude, opencode, or codex
    pub(crate) target: Target,

    #[usage(arg_group)]
    pub(crate) scope: Scope,
}

impl Run for Install {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        match run_install(self.target, self.scope) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("srcthread: {e}");
                ExitCode::from(1)
            }
        }
    }
}

pub(crate) fn run_install(target: Target, scope: Scope) -> Result<(), String> {
    let roots = Roots::from_env()?;
    install_with(&roots, target, scope)
}

pub(crate) fn install_with(roots: &Roots, target: Target, scope: Scope) -> Result<(), String> {
    let path = target.config_path(scope, roots);
    let outcome = mcp_config::install(&path, target.format())?;
    match outcome {
        InstallOutcome::Installed => {
            println!(
                "installed {} ({}) MCP at {}",
                target.as_str(),
                scope.as_str(),
                path.display()
            );
        }
        InstallOutcome::AlreadyPresent => {
            println!(
                "{} ({}) MCP already present at {}",
                target.as_str(),
                scope.as_str(),
                path.display()
            );
        }
    }
    Ok(())
}
