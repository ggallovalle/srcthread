use std::process::ExitCode;

use usage_rs::{Args, Run};

use crate::cli::util::stub;

#[derive(Args)]
pub(crate) struct Status {}

impl Run for Status {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        stub("sync git status")
    }
}
