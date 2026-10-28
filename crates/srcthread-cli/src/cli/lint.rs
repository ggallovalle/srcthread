use std::process::ExitCode;

use usage_rs::{Args, Run};

use crate::cli::discover;

/// Lint the active store (structural checks)
///
/// Everyday commands stay lenient; use this for a strict pass. `--fix` is
/// reserved for safe autofixes (not implemented yet).
#[derive(Args)]
#[usage(verbatim_doc_comment)]
pub(crate) struct Lint {
    /// Apply safe autofixes (stub)
    #[usage(long)]
    pub(crate) fix: bool,
}

impl Run for Lint {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        if self.fix {
            tracing::warn!("lint --fix is not implemented yet; reporting only");
        }
        let root = match discover::resolve_active_store_root() {
            Ok(r) => r,
            Err(err) => {
                eprintln!("srcthread: {err}");
                return ExitCode::from(1);
            }
        };
        tracing::info!(path = %root.display(), "linting store");
        match srcthread::lint_store(&root) {
            Ok(diags) if diags.is_empty() => {
                println!("ok: {}", root.display());
                ExitCode::SUCCESS
            }
            Ok(diags) => {
                for d in &diags {
                    println!("{}: {}", d.path, d.message);
                    tracing::warn!(path = %d.path, message = %d.message, "lint");
                }
                eprintln!("{} issue(s)", diags.len());
                ExitCode::from(1)
            }
            Err(err) => {
                eprintln!("srcthread: {err}");
                ExitCode::from(1)
            }
        }
    }
}
