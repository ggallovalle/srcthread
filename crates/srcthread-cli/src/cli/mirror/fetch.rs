use std::process::ExitCode;

use usage_rs::{Args, Run};

use crate::cli::util::stub;

#[derive(Args)]
pub(crate) struct Fetch {
    /// Mirror id or repo
    pub(crate) repo: String,
}

impl Run for Fetch {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        let _ = self.repo;
        stub("mirror fetch")
    }
}
