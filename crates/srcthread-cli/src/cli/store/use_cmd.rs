use std::process::ExitCode;

use usage_rs::{Args, Run};

use crate::cli::util::stub;

#[derive(Args)]
pub(crate) struct Use {
    /// Store name or path
    pub(crate) store: String,
}

impl Run for Use {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        let _ = self.store;
        stub("store use")
    }
}
