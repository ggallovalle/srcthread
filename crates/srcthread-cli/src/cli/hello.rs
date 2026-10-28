use std::process::ExitCode;

use serde_json::json;
use srcthread::Srcthread;
use usage_rs::{Args, Run};

use super::util::daemon_call;

/// Hello-world: daemon socket if up, else in-process lib
#[derive(Args)]
#[usage(verbatim_doc_comment)]
pub(crate) struct Hello {}

impl Run for Hello {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        let path = srcthread_daemon::default_socket_path();
        if path.exists() {
            match daemon_call("hello", json!({}), Some(path.as_path())) {
                Ok(result) => {
                    let greeting = result
                        .as_str()
                        .map(str::to_string)
                        .unwrap_or_else(|| result.to_string());
                    println!("{greeting}");
                    return ExitCode::SUCCESS;
                }
                Err(err) => {
                    eprintln!("daemon hello failed ({err}); falling back to in-process");
                }
            }
        }
        let eng = Srcthread::open_memory();
        println!("{} ({})", eng.hello(), eng.list_threads().join(", "));
        ExitCode::SUCCESS
    }
}
