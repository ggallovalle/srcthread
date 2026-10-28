mod cli;

use std::process::ExitCode;

use cli::SrcthreadCli;

fn main() -> ExitCode {
    SrcthreadCli::parse().run()
}
