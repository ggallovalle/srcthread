use std::process::ExitCode;

use usage_rs::{Args, Run};

use super::xdg::{self, Flavor, XdgRoots};

/// Remove FreeDesktop .desktop entry and icon
#[derive(Args)]
#[usage(verbatim_doc_comment)]
pub(crate) struct UninstallXdg {
    /// Remove the ship-badged -dev launcher
    #[usage(long)]
    pub(crate) dev: bool,
}

impl Run for UninstallXdg {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        match (|| {
            let roots = XdgRoots::from_env()?;
            xdg::uninstall(&roots, Flavor::from_dev_flag(self.dev))
        })() {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("srcthread: {e}");
                ExitCode::from(1)
            }
        }
    }
}
