use std::process::ExitCode;

use usage_rs::{Args, Run};

use crate::cli::discover;

/// List research threads (query).
#[derive(Args)]
pub(crate) struct List {}

impl Run for List {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        let eng = match discover::open_active_store() {
            Ok(e) => e,
            Err(err) => {
                eprintln!("srcthread: {err}");
                return ExitCode::from(1);
            }
        };
        match eng.list_thread_meta() {
            Ok(threads) => {
                for t in threads {
                    println!("{}\t{}", t.id, t.title);
                }
                ExitCode::SUCCESS
            }
            Err(err) => {
                eprintln!("srcthread: {err}");
                ExitCode::from(1)
            }
        }
    }
}
