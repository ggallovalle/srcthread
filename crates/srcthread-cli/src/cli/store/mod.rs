mod init;
mod list;
mod path;
mod use_cmd;

use std::process::ExitCode;

use usage_rs::{Args, Run, Subcommands};

/// Manage file-canonical research stores (git vaults)
///
/// With no subcommand, prints the active store path (stub).
#[derive(Args)]
#[usage(verbatim_doc_comment)]
pub(crate) struct Store {
    #[usage(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommands)]
#[usage(run)]
enum Commands {
    /// Create a new file-canonical store directory
    Init(init::Init),
    /// List known stores
    #[usage(visible_alias = "ls")]
    List(list::List),
    /// Select the active store
    Use(use_cmd::Use),
    /// Print the active store path
    Path(path::Path),
}

impl Run for Store {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        match self.command {
            Some(cmd) => cmd.run(),
            None => path::Path {}.run(),
        }
    }
}
