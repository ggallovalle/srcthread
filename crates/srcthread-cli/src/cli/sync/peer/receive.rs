use std::process::ExitCode;

use usage_rs::{Args, Run};

use crate::cli::util::stub;

#[derive(Args)]
pub(crate) struct Receive {}

impl Run for Receive {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        stub("sync peer receive")
    }
}
