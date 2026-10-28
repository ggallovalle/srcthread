mod add;
mod list;
mod open;
mod show;

use std::process::ExitCode;

use usage_rs::{Args, Run, Subcommands};

/// Anchors into mirrored source at a pinned revision
#[derive(Args)]
#[usage(verbatim_doc_comment)]
pub(crate) struct Anchor {
    #[usage(subcommand)]
    command: Commands,
}

#[derive(Subcommands)]
#[usage(run)]
enum Commands {
    /// Add an anchor (repo@sha path line/span or symbol)
    Add(add::Add),
    /// List anchors in a thread
    #[usage(visible_alias = "ls")]
    List(list::List),
    /// Show one anchor
    Show(show::Show),
    /// Print path:line for editor open (nvim)
    Open(open::Open),
}

impl Run for Anchor {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        self.command.run()
    }
}
