//! Structural lint for a file-canonical store (lenient load vs strict check).

use std::path::Path;

use crate::error::Result;
use crate::store::DiskStore;
use crate::types::RefKind;

/// One lint finding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LintDiagnostic {
    pub path: String,
    pub message: String,
}

/// Lint a store root (or project containing `srcthread/`).
pub fn lint_store(path: &Path) -> Result<Vec<LintDiagnostic>> {
    let disk = crate::store::open_disk(path)?;
    Ok(lint_disk(&disk))
}

fn lint_disk(disk: &DiskStore) -> Vec<LintDiagnostic> {
    let mut out = Vec::new();
    if disk.config().store.name.trim().is_empty() {
        out.push(LintDiagnostic {
            path: "config.toml".into(),
            message: "[store].name must not be empty".into(),
        });
    }
    for (id, spec) in &disk.config().mirrors {
        if spec.git.trim().is_empty() {
            out.push(LintDiagnostic {
                path: format!("config.toml:[mirrors].{id}"),
                message: "git URL is empty".into(),
            });
        }
    }

    let threads = match disk.list_threads() {
        Ok(t) => t,
        Err(err) => {
            out.push(LintDiagnostic {
                path: "threads/".into(),
                message: format!("failed to list threads: {err}"),
            });
            return out;
        }
    };

    for meta in threads {
        let dir_id = meta.id.clone();
        if meta.id.trim().is_empty() {
            out.push(LintDiagnostic {
                path: format!("threads/{dir_id}/thread.toml"),
                message: "id must not be empty".into(),
            });
        }
        if meta.title.trim().is_empty() {
            out.push(LintDiagnostic {
                path: format!("threads/{dir_id}/thread.toml"),
                message: "title must not be empty".into(),
            });
        }
        match disk.load_thread(&dir_id) {
            Ok(loaded) => {
                if loaded.meta.id != dir_id {
                    out.push(LintDiagnostic {
                        path: format!("threads/{dir_id}/thread.toml"),
                        message: format!("id {:?} does not match directory name", loaded.meta.id),
                    });
                }
                for a in &loaded.anchors {
                    if !disk.config().mirrors.contains_key(&a.meta.mirror) {
                        out.push(LintDiagnostic {
                            path: a.path.display().to_string(),
                            message: format!("mirror {:?} not in [mirrors]", a.meta.mirror),
                        });
                    }
                    if a.meta.sha.trim().is_empty() || a.meta.path.trim().is_empty() {
                        out.push(LintDiagnostic {
                            path: a.path.display().to_string(),
                            message: "sha and path are required".into(),
                        });
                    }
                }
                for e in &loaded.excerpts {
                    if !loaded.anchors.iter().any(|a| a.id == e.meta.anchor_id) {
                        out.push(LintDiagnostic {
                            path: e.path.display().to_string(),
                            message: format!(
                                "anchor_id {:?} not found in anchors/",
                                e.meta.anchor_id
                            ),
                        });
                    }
                    if e.fences.is_empty() {
                        out.push(LintDiagnostic {
                            path: e.path.display().to_string(),
                            message: "excerpt has no fenced code body".into(),
                        });
                    }
                }
                for r in &loaded.refs {
                    if matches!(r.meta.kind, RefKind::Pr | RefKind::Issue | RefKind::Doc)
                        && r.meta
                            .url
                            .as_ref()
                            .map(|u| u.trim().is_empty())
                            .unwrap_or(true)
                    {
                        out.push(LintDiagnostic {
                            path: r.path.display().to_string(),
                            message: format!("{:?} ref should have a url", r.meta.kind),
                        });
                    }
                }
            }
            Err(err) => out.push(LintDiagnostic {
                path: format!("threads/{dir_id}/"),
                message: format!("failed to load thread: {err}"),
            }),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn seed_store_is_clean() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let diags = lint_store(&root).unwrap();
        assert!(diags.is_empty(), "unexpected diagnostics: {diags:#?}");
    }
}
