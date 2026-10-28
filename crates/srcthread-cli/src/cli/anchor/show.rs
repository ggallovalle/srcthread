use std::process::ExitCode;

use usage_rs::{Args, Run};

use crate::cli::util::stub;

#[derive(Args)]
pub(crate) struct Show {
    /// Anchor id
    pub(crate) id: String,
}

impl Run for Show {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        let _ = self.id;
        stub("anchor show")
    }
}
