mod anchors;
mod threads;

use std::process::ExitCode;

use usage_rs::{Args, Run, Subcommands};

/// Search threads or anchors in the active store
#[derive(Args)]
#[usage(verbatim_doc_comment)]
pub(crate) struct Search {
    #[usage(subcommand)]
    command: Commands,
}

#[derive(Subcommands)]
#[usage(run)]
enum Commands {
    /// Search research threads
    Threads(threads::Threads),
    /// Search anchors
    Anchors(anchors::Anchors),
}

impl Run for Search {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        self.command.run()
    }
}
