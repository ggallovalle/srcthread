use std::process::ExitCode;

use usage_rs::{Args, Run};

use crate::cli::util::stub;

#[derive(Args)]
pub(crate) struct Add {
    /// Git URL or owner/repo
    pub(crate) repo: String,
}

impl Run for Add {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        let _ = self.repo;
        stub("mirror add")
    }
}
