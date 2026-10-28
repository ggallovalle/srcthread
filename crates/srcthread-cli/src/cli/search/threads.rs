use std::process::ExitCode;

use usage_rs::{Args, Run};

use crate::cli::util::stub;

#[derive(Args)]
pub(crate) struct Threads {
    /// Query string
    pub(crate) query: String,
}

impl Run for Threads {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        let _ = self.query;
        stub("search threads")
    }
}
