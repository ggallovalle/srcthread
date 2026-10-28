mod status;
mod stop;

use std::path::PathBuf;
use std::process::ExitCode;

use usage_rs::{Args, Run, Subcommands};

/// Run or control the local socket daemon
///
/// With no subcommand, runs the daemon in the foreground (herdr-style:
/// `srcthread daemon` *is* the server process). Use `stop` / `status` against
/// a running instance.
#[derive(Args)]
#[usage(verbatim_doc_comment)]
pub(crate) struct Daemon {
    #[usage(subcommand)]
    command: Option<Commands>,

    /// Unix socket path (default: `$XDG_RUNTIME_DIR/srcthread.sock`)
    #[usage(long, value_name = "PATH", global = true)]
    pub(crate) socket: Option<PathBuf>,
}

#[derive(Subcommands)]
#[usage(run)]
enum Commands {
    /// Stop a running daemon via the socket
    Stop(stop::Stop),
    /// Show daemon reachability and socket path
    Status(status::Status),
}

impl Run for Daemon {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        match self.command {
            Some(Commands::Stop(cmd)) => cmd.run_with_socket(self.socket),
            Some(Commands::Status(cmd)) => cmd.run_with_socket(self.socket),
            None => srcthread_daemon::run(srcthread_daemon::RunOpts {
                socket: self.socket,
                foreground: true,
            }),
        }
    }
}
