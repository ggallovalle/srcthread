//! Shared CLI helpers.

use std::process::ExitCode;

pub(crate) fn stub(name: &str) -> ExitCode {
    eprintln!("srcthread: `{name}` is not implemented yet");
    ExitCode::from(2)
}
