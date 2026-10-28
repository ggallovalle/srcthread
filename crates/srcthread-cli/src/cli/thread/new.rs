use std::process::ExitCode;

use usage_rs::{Args, Run};

use crate::cli::util::stub;

#[derive(Args)]
pub(crate) struct New {
    /// Short title for the research thread
    pub(crate) title: String,
}

impl Run for New {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        let _ = self.title;
        stub("thread new")
    }
}
