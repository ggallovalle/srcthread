use std::path::PathBuf;
use std::process::ExitCode;

use serde_json::json;
use usage_rs::{Args, Run};

use crate::cli::util::{daemon_call, resolve_socket_path};

#[derive(Args)]
pub(crate) struct Status {}

impl Status {
    pub(crate) fn run_with_socket(self, socket: Option<PathBuf>) -> ExitCode {
        let path = resolve_socket_path(socket);
        print!("socket: {} — ", path.display());
        if !path.exists() {
            println!("not listening");
            return ExitCode::from(1);
        }
        match daemon_call("ping", json!({}), Some(path.as_path())) {
            Ok(_) => {
                println!("ok");
                ExitCode::SUCCESS
            }
            Err(err) => {
                println!("unreachable ({err})");
                ExitCode::from(1)
            }
        }
    }
}

impl Run for Status {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        self.run_with_socket(None)
    }
}
