use std::process::ExitCode;

use usage_rs::{Args, Run};

use super::util::stub;

/// Environment / daemon / store health checks
#[derive(Args)]
#[usage(verbatim_doc_comment)]
pub(crate) struct Doctor {}

impl Run for Doctor {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        stub("doctor")
    }
}
