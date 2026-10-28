use std::process::ExitCode;

use usage_rs::{Args, Run};

use crate::cli::util::stub;

#[derive(Args)]
pub(crate) struct Init {
    /// Directory for the new store (git vault)
    pub(crate) path: String,
}

impl Run for Init {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        let _ = self.path;
        stub("store init")
    }
}
