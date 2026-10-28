use std::process::ExitCode;

use srcthread::Srcthread;
use usage_rs::{Args, Run};

/// List research threads (query).
#[derive(Args)]
pub(crate) struct List {}

impl Run for List {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        for id in Srcthread::open_memory().list_threads() {
            println!("{id}");
        }
        ExitCode::SUCCESS
    }
}
