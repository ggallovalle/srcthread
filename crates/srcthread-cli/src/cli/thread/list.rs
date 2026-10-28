use std::process::ExitCode;

use serde_json::json;
use srcthread::Srcthread;
use usage_rs::{Args, Run};

use crate::cli::util::daemon_call;

/// List research threads (query). Prefers the daemon when its socket is up.
#[derive(Args)]
pub(crate) struct List {}

impl Run for List {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        let path = srcthread_daemon::default_socket_path();
        if path.exists() {
            match daemon_call("thread.list", json!({}), Some(path.as_path())) {
                Ok(result) => {
                    if let Some(arr) = result.as_array() {
                        for id in arr {
                            if let Some(s) = id.as_str() {
                                println!("{s}");
                            }
                        }
                        return ExitCode::SUCCESS;
                    }
                    eprintln!("unexpected thread.list result: {result}");
                    return ExitCode::from(1);
                }
                Err(err) => {
                    eprintln!("daemon thread.list failed ({err}); falling back to in-process");
                }
            }
        }
        for id in Srcthread::open_memory().list_threads() {
            println!("{id}");
        }
        ExitCode::SUCCESS
    }
}
