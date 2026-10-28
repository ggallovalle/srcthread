use std::process::ExitCode;

use usage_rs::{Args, Run};

use crate::cli::discover;

/// Create a research thread (mutation).
#[derive(Args)]
pub(crate) struct New {
    /// Short id/title for the research thread
    pub(crate) title: String,
}

impl Run for New {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        let eng = match discover::open_active_store() {
            Ok(e) => e,
            Err(err) => {
                eprintln!("srcthread: {err}");
                return ExitCode::from(1);
            }
        };
        match eng.create_thread_from_title(&self.title) {
            Ok(meta) => {
                println!("{}", meta.id);
                ExitCode::SUCCESS
            }
            Err(err) => {
                eprintln!("srcthread: {err}");
                ExitCode::from(1)
            }
        }
    }
}
