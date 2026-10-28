use std::process::ExitCode;

use usage_rs::{Args, Run};

use crate::cli::util::stub;

#[derive(Args)]
pub(crate) struct List {}

impl Run for List {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        stub("store list")
    }
}
