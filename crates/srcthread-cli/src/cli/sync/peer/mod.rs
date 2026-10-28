mod discover;
mod receive;
mod send;

use std::process::ExitCode;

use usage_rs::{Args, Run, Subcommands};

/// Nearby peer transfer (Localsend-like)
#[derive(Args)]
#[usage(verbatim_doc_comment)]
pub(crate) struct Peer {
    #[usage(subcommand)]
    command: Commands,
}

#[derive(Subcommands)]
#[usage(run)]
enum Commands {
    /// Discover nearby peers
    Discover(discover::Discover),
    /// Send store or selected threads to a peer
    Send(send::Send),
    /// Receive a store/threads transfer
    Receive(receive::Receive),
}

impl Run for Peer {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        self.command.run()
    }
}
