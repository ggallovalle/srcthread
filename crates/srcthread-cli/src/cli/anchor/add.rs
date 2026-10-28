use std::process::ExitCode;

use usage_rs::{Args, Run};

use crate::cli::util::stub;

#[derive(Args)]
pub(crate) struct Add {
    /// Thread id
    #[usage(long)]
    pub(crate) thread: String,
    /// Upstream repo URL or mirror id
    #[usage(long)]
    pub(crate) repo: String,
    /// Pinned commit SHA
    #[usage(long)]
    pub(crate) sha: String,
    /// Path inside the repo
    #[usage(long)]
    pub(crate) path: String,
    /// Optional 1-based line
    #[usage(long)]
    pub(crate) line: Option<u32>,
    /// Optional symbol name (e.g. op_get_env)
    #[usage(long)]
    pub(crate) symbol: Option<String>,
}

impl Run for Add {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        let _ = (
            self.thread,
            self.repo,
            self.sha,
            self.path,
            self.line,
            self.symbol,
        );
        stub("anchor add")
    }
}
