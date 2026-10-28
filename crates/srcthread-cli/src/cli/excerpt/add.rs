use std::process::ExitCode;

use usage_rs::{Args, Run};

use crate::cli::util::stub;

#[derive(Args)]
pub(crate) struct Add {
    /// Anchor id to excerpt
    #[usage(long)]
    pub(crate) anchor: String,
    /// Context lines before/after
    #[usage(long, default = "10")]
    pub(crate) context: u32,
}

impl Run for Add {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        let _ = (self.anchor, self.context);
        stub("excerpt add")
    }
}
