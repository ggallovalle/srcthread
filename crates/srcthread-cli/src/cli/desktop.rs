use std::process::ExitCode;

use usage_rs::{Args, Run};

/// Open the rune desktop UI
#[derive(Args)]
#[usage(verbatim_doc_comment)]
pub(crate) struct Desktop {}

impl Run for Desktop {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        #[cfg(feature = "desktop")]
        {
            srcthread_desktop::run()
        }
        #[cfg(not(feature = "desktop"))]
        {
            eprintln!(
                "srcthread: desktop UI was not compiled in (build with `--features desktop`)"
            );
            ExitCode::from(2)
        }
    }
}
