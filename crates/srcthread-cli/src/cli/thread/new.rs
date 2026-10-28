use std::process::ExitCode;

use serde_json::json;
use srcthread::Srcthread;
use usage_rs::{Args, Run};

use crate::cli::util::daemon_call;

/// Create a research thread (mutation). Prefers the daemon when its socket is up.
#[derive(Args)]
pub(crate) struct New {
    /// Short id/title for the research thread
    pub(crate) title: String,
}

impl Run for New {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        let path = srcthread_daemon::default_socket_path();
        if path.exists() {
            match daemon_call(
                "thread.create",
                json!({ "id": self.title }),
                Some(path.as_path()),
            ) {
                Ok(result) => {
                    let id = result
                        .get("id")
                        .and_then(|v| v.as_str())
                        .unwrap_or(self.title.as_str());
                    println!("{id}");
                    return ExitCode::SUCCESS;
                }
                Err(err) => {
                    eprintln!("daemon thread.create failed ({err}); falling back to in-process");
                }
            }
        }
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
