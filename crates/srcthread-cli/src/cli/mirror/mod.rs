mod add;
mod fetch;
mod list;
mod path;

use std::process::ExitCode;

use usage_rs::{Args, Run, Subcommands};

/// Shared upstream mirror cache
///
/// With no subcommand, lists cached mirrors (stub).
#[derive(Args)]
#[usage(verbatim_doc_comment)]
pub(crate) struct Mirror {
    #[usage(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommands)]
#[usage(run)]
enum Commands {
    /// Register / fetch an upstream repo into the mirror cache
    Add(add::Add),
    /// List cached mirrors
    #[usage(visible_alias = "ls")]
    List(list::List),
    /// Print local path for a mirror@sha
    Path(path::Path),
    /// Fetch updates for a mirror
    Fetch(fetch::Fetch),
}

impl Run for Mirror {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        match self.command {
            Some(cmd) => cmd.run(),
            None => list::List {}.run(),
        }
    }
}
