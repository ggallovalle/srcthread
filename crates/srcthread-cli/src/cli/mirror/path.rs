use std::process::ExitCode;

use usage_rs::{Args, Run};

use crate::cli::util::stub;

#[derive(Args)]
pub(crate) struct Path {
    /// Mirror id or repo
    pub(crate) repo: String,
    /// Optional SHA
    #[usage(long)]
    pub(crate) sha: Option<String>,
}

impl Run for Path {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        let _ = (self.repo, self.sha);
        stub("mirror path")
    }
}
