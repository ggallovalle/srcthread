use std::process::ExitCode;

use srcthread::Srcthread;
use usage_rs::{Args, Run};

/// Hello-world: in-process lib greeting + thread list
#[derive(Args)]
#[usage(verbatim_doc_comment)]
pub(crate) struct Hello {}

impl Run for Hello {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        let eng = Srcthread::open_memory();
        println!("{} ({})", eng.hello(), eng.list_threads().join(", "));
        ExitCode::SUCCESS
    }
}
