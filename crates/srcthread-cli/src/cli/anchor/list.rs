use std::process::ExitCode;

use usage_rs::{Args, Run};

use crate::cli::discover;

#[derive(Args)]
pub(crate) struct List {
    /// Thread id
    #[usage(long)]
    pub(crate) thread: String,
}

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
        match eng.load_thread(&self.thread) {
            Ok(t) => {
                for a in t.anchors {
                    let line = a
                        .meta
                        .line
                        .map(|n| n.to_string())
                        .unwrap_or_else(|| "-".into());
                    println!(
                        "{}\t{}\t{}\t{}:{}",
                        a.id, a.meta.mirror, a.meta.sha, a.meta.path, line
                    );
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
