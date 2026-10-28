use std::path::{Path, PathBuf};
use std::process::ExitCode;

use srcthread::{StoreMeta, init_store};
use usage_rs::{Args, Run};

#[derive(Args)]
pub(crate) struct Init {
    /// Directory for the new store (git vault or project to nest under)
    pub(crate) path: String,

    /// Create the store at PATH itself instead of PATH/srcthread/
    #[usage(long)]
    pub(crate) at_path: bool,

    /// Display name in config.toml (default: directory name)
    #[usage(long)]
    pub(crate) name: Option<String>,
}

impl Run for Init {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        let base = PathBuf::from(&self.path);
        let root = match resolve_init_root(&base, self.at_path) {
            Ok(r) => r,
            Err(msg) => {
                eprintln!("srcthread: {msg}");
                return ExitCode::from(1);
            }
        };
        let name = self.name.unwrap_or_else(|| {
            root.file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("store")
                .to_string()
        });
        let meta = StoreMeta {
            name,
            description: None,
            authors: vec![],
            license: None,
            tags: vec![],
            edition: None,
        };
        match init_store(&root, meta) {
            Ok(root) => {
                println!("{}", root.display());
                ExitCode::SUCCESS
            }
            Err(err) => {
                eprintln!("srcthread: {err}");
                ExitCode::from(1)
            }
        }
    }
}

fn resolve_init_root(path: &Path, at_path: bool) -> Result<PathBuf, String> {
    if at_path {
        return Ok(path.to_path_buf());
    }
    // Already targeting a `srcthread` directory → store-at-PATH.
    if path
        .file_name()
        .and_then(|s| s.to_str())
        .is_some_and(|n| n == "srcthread")
    {
        return Ok(path.to_path_buf());
    }
    Ok(path.join("srcthread"))
}
