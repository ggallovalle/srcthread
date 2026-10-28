use std::process::ExitCode;

use usage_rs::{Args, Run};

use crate::cli::util::stub;

#[derive(Args)]
pub(crate) struct Open {
    /// Anchor id
    pub(crate) id: String,
}

impl Run for Open {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        let _ = self.id;
        stub("anchor open")
    }
}
