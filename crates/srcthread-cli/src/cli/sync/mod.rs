mod git;
mod peer;

use std::process::ExitCode;

use usage_rs::{Args, Run, Subcommands};

/// Sync stores via git or nearby peer transfer
#[derive(Args)]
#[usage(verbatim_doc_comment)]
pub(crate) struct Sync {
    #[usage(subcommand)]
    command: Commands,
}

#[derive(Subcommands)]
#[usage(run)]
enum Commands {
    /// Git-backed store sync
    Git(git::Git),
    /// Nearby peer transfer (Localsend-like)
    Peer(peer::Peer),
}

impl Run for Sync {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        self.command.run()
    }
}
