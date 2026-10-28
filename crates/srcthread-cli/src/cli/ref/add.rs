use std::process::ExitCode;

use usage_rs::{Args, Run};

use crate::cli::util::stub;

#[derive(Args)]
pub(crate) struct Add {
    /// Thread id
    #[usage(long)]
    pub(crate) thread: String,
    /// URL (PR, issue, doc, chat)
    #[usage(long)]
    pub(crate) url: Option<String>,
    /// Optional local transcript / note path to ingest
    #[usage(long)]
    pub(crate) file: Option<String>,
}

impl Run for Add {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        let _ = (self.thread, self.url, self.file);
        stub("ref add")
    }
}
