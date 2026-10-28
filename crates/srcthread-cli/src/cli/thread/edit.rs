use std::process::{Command, ExitCode};

use usage_rs::{Args, Run};

use crate::cli::discover;

#[derive(Args)]
pub(crate) struct Edit {
    /// Thread id
    pub(crate) id: String,
}

impl Run for Edit {
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
            eprintln!("srcthread: thread edit requires an on-disk store");
            return ExitCode::from(1);
        };
        let notes = match disk.notes_path(&self.id) {
            Ok(p) => p,
            Err(err) => {
                eprintln!("srcthread: {err}");
                return ExitCode::from(1);
            }
        };
        match std::env::var_os("EDITOR").or_else(|| std::env::var_os("VISUAL")) {
            Some(editor) => {
                let status = Command::new(&editor).arg(&notes).status();
                match status {
                    Ok(s) if s.success() => ExitCode::SUCCESS,
                    Ok(s) => {
                        eprintln!("srcthread: editor exited with {s}");
                        ExitCode::from(1)
                    }
                    Err(err) => {
                        eprintln!("srcthread: failed to launch editor: {err}");
                        eprintln!("{}", notes.display());
                        ExitCode::from(1)
                    }
                }
            }
            None => {
                // No EDITOR — print path for the user / scripts.
                println!("{}", notes.display());
                ExitCode::SUCCESS
            }
        }
    }
}
