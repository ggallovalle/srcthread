use std::fs;
use std::process::ExitCode;

use usage_rs::{Args, Run};

use crate::cli::{discover, find};

#[derive(Args)]
pub(crate) struct Show {
    /// Excerpt id
    pub(crate) id: String,
}

impl Run for Show {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        let eng = match discover::open_active_store() {
            Ok(e) => e,
            Err(err) => {
                eprintln!("srcthread: {err}");
                return ExitCode::from(1);
            }
        };
        match find::find_excerpt(&eng, &self.id) {
            Ok((_thread, ex)) => match fs::read_to_string(&ex.path) {
                Ok(text) => {
                    print!("{text}");
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("srcthread: {}: {err}", ex.path.display());
                    ExitCode::from(1)
                }
            },
            Err(err) => {
                eprintln!("srcthread: {err}");
                ExitCode::from(1)
            }
        }
    }
}
