use std::process::ExitCode;

use usage_rs::{Args, Run};

use crate::cli::util::stub;

#[derive(Args)]
pub(crate) struct List {
    /// Thread id
    #[usage(long)]
    pub(crate) thread: String,
}

impl Run for List {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        let _ = self.thread;
        stub("ref list")
    }
}
