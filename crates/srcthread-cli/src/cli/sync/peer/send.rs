use std::process::ExitCode;

use usage_rs::{Args, Run};

use crate::cli::util::stub;

#[derive(Args)]
pub(crate) struct Send {
    /// Peer id from discover
    #[usage(long)]
    pub(crate) peer: String,
    /// Optional thread id (default: whole store)
    #[usage(long)]
    pub(crate) thread: Option<String>,
}

impl Run for Send {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        let _ = (self.peer, self.thread);
        stub("sync peer send")
    }
}
