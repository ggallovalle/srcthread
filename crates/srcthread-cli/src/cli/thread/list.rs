use std::process::ExitCode;

use srcthread::Srcthread;
use usage_rs::{Args, Run};

#[derive(Args)]
pub(crate) struct List {}

impl Run for List {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        let eng = Srcthread::open_memory();
        for id in eng.list_threads() {
            println!("{id}");
        }
        ExitCode::SUCCESS
    }
}
