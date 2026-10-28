use std::process::ExitCode;

use usage_rs::{Args, Run};

use super::xdg::{self, Flavor, XdgRoots};

/// Install FreeDesktop .desktop entry and icon
#[derive(Args)]
#[usage(verbatim_doc_comment)]
pub(crate) struct InstallXdg {
    /// Install the ship-badged -dev launcher (workspace / shellrc)
    #[usage(long)]
    pub(crate) dev: bool,
}

impl Run for InstallXdg {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        match (|| {
            let roots = XdgRoots::from_env()?;
            xdg::install(&roots, Flavor::from_dev_flag(self.dev))
        })() {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("srcthread: {e}");
                ExitCode::from(1)
            }
        }
    }
}
