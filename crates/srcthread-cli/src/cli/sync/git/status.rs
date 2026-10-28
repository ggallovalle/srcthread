use std::process::ExitCode;

use usage_rs::{Args, Run};

use super::run_git;

#[derive(Args)]
pub(crate) struct Status {}

impl Run for Status {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        run_git(&["status", "--short", "--branch"])
    }
}
