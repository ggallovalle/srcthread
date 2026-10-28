mod cli;

use std::process::ExitCode;

use cli::SrcthreadCli;
use usage_rs::Run;

fn main() -> ExitCode {
    SrcthreadCli::parse().command.run()
}
