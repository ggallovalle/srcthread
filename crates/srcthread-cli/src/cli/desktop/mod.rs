//! Desktop UI launch and FreeDesktop XDG helpers.

mod install_xdg;
mod uninstall_xdg;
mod xdg;
mod xdg_status;

use std::process::ExitCode;

use usage_rs::{Args, Run, Subcommands};

/// Open the rune desktop UI
///
/// With no subcommand, launches the desktop. Use `install-xdg` /
/// `uninstall-xdg` / `xdg-status` for FreeDesktop launcher and icons.
#[derive(Args)]
#[usage(verbatim_doc_comment)]
pub(crate) struct Desktop {
    #[usage(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommands)]
#[usage(run)]
enum Commands {
    /// Install FreeDesktop .desktop entry and icon
    InstallXdg(install_xdg::InstallXdg),
    /// Remove FreeDesktop .desktop entry and icon
    UninstallXdg(uninstall_xdg::UninstallXdg),
    /// Show FreeDesktop install status
    XdgStatus(xdg_status::XdgStatus),
}

impl Run for Desktop {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        match self.command {
            Some(Commands::InstallXdg(cmd)) => cmd.run(),
            Some(Commands::UninstallXdg(cmd)) => cmd.run(),
            Some(Commands::XdgStatus(cmd)) => cmd.run(),
            None => launch_ui(),
        }
    }
}

fn launch_ui() -> ExitCode {
    #[cfg(feature = "desktop")]
    {
        srcthread_desktop::run()
    }
    #[cfg(not(feature = "desktop"))]
    {
        eprintln!("srcthread: desktop UI was not compiled in (build with `--features desktop`)");
        ExitCode::from(2)
    }
}
