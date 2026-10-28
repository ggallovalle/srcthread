mod start;
mod status;
mod stop;

use std::process::ExitCode;

use usage_rs::{Args, Run, Subcommands};

/// Manage the local `srcthreadd` daemon
///
/// With no subcommand, prints daemon status (stub).
#[derive(Args)]
#[usage(verbatim_doc_comment)]
pub(crate) struct Daemon {
    #[usage(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommands)]
#[usage(run)]
enum Commands {
    /// Start `srcthreadd` (socket daemon)
    Start(start::Start),
    /// Stop a running daemon
    Stop(stop::Stop),
    /// Show daemon reachability and socket path
    Status(status::Status),
}

impl Run for Daemon {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        match self.command {
            Some(cmd) => cmd.run(),
            None => status::Status {}.run(),
        }
    }
}
