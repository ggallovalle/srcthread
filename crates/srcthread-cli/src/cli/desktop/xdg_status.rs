use std::process::ExitCode;

use usage_rs::{Args, Run};

use super::xdg::{self, Flavor, XdgRoots};

/// Show FreeDesktop install status
#[derive(Args)]
#[usage(verbatim_doc_comment)]
pub(crate) struct XdgStatus {
    /// Check the ship-badged -dev launcher
    #[usage(long)]
    pub(crate) dev: bool,
}

impl Run for XdgStatus {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        match (|| {
            let roots = XdgRoots::from_env()?;
            xdg::status(&roots, Flavor::from_dev_flag(self.dev))
        })() {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("srcthread: {e}");
                ExitCode::from(1)
            }
        }
    }
}
