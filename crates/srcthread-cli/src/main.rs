mod cli;

use std::process::ExitCode;

use cli::SrcthreadCli;

fn main() -> ExitCode {
    init_tracing();
    SrcthreadCli::parse().run()
}

fn init_tracing() {
    let filter = tracing_subscriber::EnvFilter::try_from_env("SRCTHREAD_LOG")
        .or_else(|_| tracing_subscriber::EnvFilter::try_new("warn"))
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn"));
    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .try_init();
}
