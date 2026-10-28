use std::io::{self, Write};
use std::process::ExitCode;

use usage_rs::{Args, Run};

use crate::cli::ctx;
use crate::cli::discover;

#[derive(Args)]
pub(crate) struct Rm {
    /// Thread id
    pub(crate) id: String,
}

impl Run for Rm {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        let eng = match discover::open_active_store() {
            Ok(e) => e,
            Err(err) => {
                eprintln!("srcthread: {err}");
                return ExitCode::from(1);
            }
        };
        let Some(disk) = eng.disk() else {
            eprintln!("srcthread: thread rm requires an on-disk store");
            return ExitCode::from(1);
        };
        if eng.get_thread(&self.id).is_err() {
            eprintln!("srcthread: thread not found: {}", self.id);
            return ExitCode::from(1);
        }
        if !ctx::yes() && !confirm_delete(&self.id) {
            eprintln!("srcthread: aborted");
            return ExitCode::from(1);
        }
        match disk.remove_thread(&self.id) {
            Ok(()) => {
                println!("removed {}", self.id);
                ExitCode::SUCCESS
            }
            Err(err) => {
                eprintln!("srcthread: {err}");
                ExitCode::from(1)
            }
        }
    }
}

fn confirm_delete(id: &str) -> bool {
    eprint!("srcthread: delete thread {id}? [y/N] ");
    let _ = io::stderr().flush();
    let mut line = String::new();
    if io::stdin().read_line(&mut line).is_err() {
        return false;
    }
    matches!(line.trim(), "y" | "Y" | "yes" | "YES")
}
