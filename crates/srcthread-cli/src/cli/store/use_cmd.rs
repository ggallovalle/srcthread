use std::path::{Path, PathBuf};
use std::process::ExitCode;

use srcthread::resolve_store_root;
use usage_rs::{Args, Run};

use crate::cli::ctx;
use crate::cli::discover;

#[derive(Args)]
pub(crate) struct Use {
    /// Store name or path
    pub(crate) store: String,
}

impl Run for Use {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        match set_active_store(&self.store) {
            Ok((id, root)) => {
                println!("{id}\t{}", root.display());
                ExitCode::SUCCESS
            }
            Err(err) => {
                eprintln!("srcthread: {err}");
                ExitCode::from(1)
            }
        }
    }
}

fn set_active_store(spec: &str) -> Result<(String, PathBuf), String> {
    let path = PathBuf::from(spec);
    let (id, root) = if path.exists() {
        let root = resolve_store_root(&path).map_err(|e| e.to_string())?;
        let id = suggest_id(&root);
        (id, root)
    } else if let Some(p) = discover::registry_path_for(spec)? {
        let root = resolve_store_root(&p).map_err(|e| e.to_string())?;
        (spec.to_string(), root)
    } else {
        return Err(format!(
            "store not found: {spec} (not a path and not in stores.toml)"
        ));
    };
    discover::upsert_registry_store(&id, &root)?;
    discover::set_default_store(&id)?;
    let _ = ctx::stores_toml_path();
    Ok((id, root))
}

fn suggest_id(root: &Path) -> String {
    root.file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "store".into())
}
