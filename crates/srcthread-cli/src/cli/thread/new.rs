use std::process::ExitCode;

use srcthread::Srcthread;
use usage_rs::{Args, Run};

/// Create a research thread (mutation).
#[derive(Args)]
pub(crate) struct New {
    /// Short id/title for the research thread
    pub(crate) title: String,
}

impl Run for New {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        let eng = Srcthread::open_memory();
        match eng.create_thread(self.title.clone()) {
            Ok(()) => {
                println!("{}", self.title);
                ExitCode::SUCCESS
            }
            Err(err) => {
                eprintln!("{err}");
                ExitCode::from(1)
            }
        }
    }
}
