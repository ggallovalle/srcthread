mod add;
mod show;

use std::process::ExitCode;

use usage_rs::{Args, Run, Subcommands};

/// Offline frozen excerpts around anchors
#[derive(Args)]
#[usage(verbatim_doc_comment)]
pub(crate) struct Excerpt {
    #[usage(subcommand)]
    command: Commands,
}

#[derive(Subcommands)]
#[usage(run)]
enum Commands {
    /// Freeze an offline excerpt around an anchor
    Add(add::Add),
    /// Show excerpt body
    Show(show::Show),
}

impl Run for Excerpt {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        self.command.run()
    }
}
