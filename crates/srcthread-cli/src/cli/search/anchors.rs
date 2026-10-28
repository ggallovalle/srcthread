use std::process::ExitCode;

use usage_rs::{Args, Run};

use crate::cli::util::stub;

#[derive(Args)]
pub(crate) struct Anchors {
    /// Query string
    pub(crate) query: String,
}

impl Run for Anchors {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        let _ = self.query;
        stub("search anchors")
    }
}
