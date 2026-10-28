use std::process::ExitCode;

use usage_rs::{Args, Run};

use crate::cli::discover;

#[derive(Args)]
pub(crate) struct List {}

impl Run for List {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        match discover::list_registry_stores() {
            Ok(entries) => {
                if entries.is_empty() {
                    let reg = crate::cli::ctx::stores_toml_path();
                    eprintln!("srcthread: no stores in registry ({})", reg.display());
                }
                for (id, path) in entries {
                    println!("{id}\t{}", path.display());
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
