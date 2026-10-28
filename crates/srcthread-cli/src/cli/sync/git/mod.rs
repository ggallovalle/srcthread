mod pull;
mod push;
mod status;

use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};

use usage_rs::{Args, Run, Subcommands};

use crate::cli::discover;

/// Git-backed store sync
///
/// With no subcommand, shows git status.
#[derive(Args)]
#[usage(verbatim_doc_comment)]
pub(crate) struct Git {
    #[usage(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommands)]
#[usage(run)]
enum Commands {
    /// `git push` the active store
    Push(push::Push),
    /// `git pull` the active store
    Pull(pull::Pull),
    /// Show git status of the active store
    Status(status::Status),
}

impl Run for Git {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        match self.command {
            Some(cmd) => cmd.run(),
            None => status::Status {}.run(),
        }
    }
}

/// Git working tree for the active store (toplevel when nested under a repo).
pub(crate) fn git_cwd() -> Result<PathBuf, String> {
    let store = discover::resolve_active_store_root().map_err(|e| e.to_string())?;
    Ok(git_toplevel(&store).unwrap_or(store))
}

fn git_toplevel(start: &Path) -> Option<PathBuf> {
    let out = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(start)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if s.is_empty() {
        None
    } else {
        Some(PathBuf::from(s))
    }
}

pub(crate) fn run_git(args: &[&str]) -> ExitCode {
    let cwd = match git_cwd() {
        Ok(c) => c,
        Err(err) => {
            eprintln!("srcthread: {err}");
            return ExitCode::from(1);
        }
    };
    let status = Command::new("git").args(args).current_dir(&cwd).status();
    match status {
        Ok(s) if s.success() => ExitCode::SUCCESS,
        Ok(s) => ExitCode::from(s.code().unwrap_or(1) as u8),
        Err(err) => {
            eprintln!("srcthread: git: {err}");
            ExitCode::from(1)
        }
    }
}
