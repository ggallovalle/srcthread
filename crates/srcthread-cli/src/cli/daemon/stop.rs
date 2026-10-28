use std::process::ExitCode;

use usage_rs::{Args, Run};

use crate::cli::util::stub;

#[derive(Args)]
pub(crate) struct Stop {}

impl Run for Stop {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        stub("daemon stop")
    }
}
