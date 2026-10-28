mod add;
mod list;

use std::process::ExitCode;

use usage_rs::{Args, Run, Subcommands};

/// External refs (PR, issue, doc, chat)
#[derive(Args)]
#[usage(verbatim_doc_comment)]
pub(crate) struct RefCmd {
    #[usage(subcommand)]
    command: Commands,
}

#[derive(Subcommands)]
#[usage(run)]
enum Commands {
    /// Attach a URL or pasted transcript ref to a thread
    Add(add::Add),
    /// List refs on a thread
    #[usage(visible_alias = "ls")]
    List(list::List),
}

impl Run for RefCmd {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        self.command.run()
    }
}
