use std::path::PathBuf;
use std::process::ExitCode;

use serde_json::json;
use usage_rs::{Args, Run};

use crate::cli::util::{daemon_call, resolve_socket_path, stub};

#[derive(Args)]
pub(crate) struct Stop {}

impl Stop {
    pub(crate) fn run_with_socket(self, socket: Option<PathBuf>) -> ExitCode {
        let path = resolve_socket_path(socket);
        if !path.exists() {
            eprintln!("no daemon at {}", path.display());
            return ExitCode::from(1);
        }
        // server.stop lands once the daemon implements it; hello-world stubs.
        match daemon_call("server.stop", json!({}), Some(path.as_path())) {
            Ok(_) => {
                println!("stop requested");
                ExitCode::SUCCESS
            }
            Err(_) => stub("daemon stop (server.stop not implemented yet)"),
        }
    }
}

impl Run for Stop {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        self.run_with_socket(None)
    }
}
