mod pull;
mod push;
mod status;

use std::process::ExitCode;

use usage_rs::{Args, Run, Subcommands};

/// Git-backed store sync
///
/// With no subcommand, shows git status (stub).
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
