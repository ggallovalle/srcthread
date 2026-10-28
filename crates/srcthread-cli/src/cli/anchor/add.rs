use std::process::ExitCode;

use srcthread::AnchorMeta;
use usage_rs::{Args, Run};

use crate::cli::discover;

#[derive(Args)]
pub(crate) struct Add {
    /// Thread id
    #[usage(long)]
    pub(crate) thread: String,
    /// Mirror id (must exist in [mirrors] unless already a known id string)
    #[usage(long)]
    pub(crate) repo: String,
    /// Pinned commit SHA
    #[usage(long)]
    pub(crate) sha: String,
    /// Path inside the repo
    #[usage(long)]
    pub(crate) path: String,
    /// Optional 1-based line
    #[usage(long)]
    pub(crate) line: Option<u32>,
    /// Optional symbol name (e.g. op_get_env)
    #[usage(long)]
    pub(crate) symbol: Option<String>,
    /// Optional explicit anchor id
    #[usage(long)]
    pub(crate) id: Option<String>,
}

impl Run for Add {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        let eng = match discover::open_active_store() {
            Ok(e) => e,
            Err(err) => {
                eprintln!("srcthread: {err}");
                return ExitCode::from(1);
            }
        };
        let Some(disk) = eng.disk() else {
            eprintln!("srcthread: anchor add requires an on-disk store");
            return ExitCode::from(1);
        };
        let mirror = self.repo;
        if disk.mirror_git(&mirror).is_none() {
            eprintln!(
                "srcthread: mirror {mirror:?} not in [mirrors] (add it to config.toml first)"
            );
            return ExitCode::from(1);
        }
        let meta = AnchorMeta {
            mirror,
            sha: self.sha,
            path: self.path,
            line: self.line,
            end_line: None,
            symbol: self.symbol,
            title: None,
        };
        match disk.add_anchor_toml(&self.thread, self.id, meta) {
            Ok(id) => {
                println!("{id}");
                ExitCode::SUCCESS
            }
            Err(err) => {
                eprintln!("srcthread: {err}");
                ExitCode::from(1)
            }
        }
    }
}
