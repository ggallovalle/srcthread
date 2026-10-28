use std::process::ExitCode;

use usage_rs::{Args, Run};

/// Run the MCP stdio server (stub)
#[derive(Args)]
#[usage(verbatim_doc_comment)]
pub(crate) struct Mcp {}

impl Run for Mcp {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        srcthread_mcp::run()
    }
}
