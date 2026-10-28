mod edit;
mod list;
mod new;
mod rm;
mod show;

use std::process::ExitCode;

use usage_rs::{Args, Run, Subcommands};

/// Research threads (primary object)
///
/// With no subcommand, lists threads.
#[derive(Args)]
#[usage(verbatim_doc_comment)]
pub(crate) struct Thread {
    #[usage(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommands)]
#[usage(run)]
enum Commands {
    /// List research threads
    #[usage(visible_alias = "ls")]
    List(list::List),
    /// Show one thread
    Show(show::Show),
    /// Create a research thread
    New(new::New),
    /// Edit thread metadata / notes
    Edit(edit::Edit),
    /// Remove a thread
    Rm(rm::Rm),
}

impl Run for Thread {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        match self.command {
            Some(cmd) => cmd.run(),
            None => list::List {}.run(),
        }
    }
}
