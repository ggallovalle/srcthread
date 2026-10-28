use std::process::ExitCode;

use usage_rs::{Args, Run};

use super::run_git;

#[derive(Args)]
pub(crate) struct Pull {}

impl Run for Pull {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        run_git(&["pull", "--ff-only"])
    }
}
