use std::process::ExitCode;

use usage_rs::{Args, Run};

use super::mcp_config;
use super::scope::{Roots, Scope};
use super::targets::Target;

/// Show MCP integration status
///
/// With no scope flag, prints both user and local for every target.
#[derive(Args)]
#[usage(verbatim_doc_comment)]
pub(crate) struct Status {
    #[usage(arg_group)]
    pub(crate) scope: Option<Scope>,
}

impl Run for Status {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        match run_status(self.scope) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("srcthread: {e}");
                ExitCode::from(1)
            }
        }
    }
}

pub(crate) fn run_status(scope: Option<Scope>) -> Result<(), String> {
    let scopes = match scope {
        None => vec![Scope::User, Scope::Local],
        Some(s) => vec![s],
    };
    let roots = Roots::from_env()?;
    status_with(&roots, &scopes)
}

pub(crate) fn status_with(roots: &Roots, scopes: &[Scope]) -> Result<(), String> {
    for target in Target::ALL {
        for &scope in scopes {
            let path = target.config_path(scope, roots);
            let state = if mcp_config::is_installed(&path, target.format())? {
                "installed"
            } else {
                "not installed"
            };
            println!(
                "{} ({}): {} ({})",
                target.as_str(),
                scope.as_str(),
                state,
                path.display()
            );
        }
    }
    Ok(())
}
