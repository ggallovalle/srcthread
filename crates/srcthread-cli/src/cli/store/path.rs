use std::process::ExitCode;

use usage_rs::{Args, Run};

use crate::cli::discover;

#[derive(Args)]
pub(crate) struct Path {}

impl Run for Path {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        match discover::resolve_active_store_root() {
            Ok(root) => {
                println!("{}", root.display());
                ExitCode::SUCCESS
            }
            Err(err) => {
                eprintln!("srcthread: {err}");
                ExitCode::from(1)
            }
        }
    }
}
