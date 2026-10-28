use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process::ExitCode;

use srcthread::{ExcerptMeta, new_entity_id};
use usage_rs::{Args, Run};

use crate::cli::{discover, find, mirrors_cache};

#[derive(Args)]
pub(crate) struct Add {
    /// Anchor id to excerpt
    #[usage(long)]
    pub(crate) anchor: String,
    /// Context lines before/after when reading from a local mirror
    #[usage(long, default = "10")]
    pub(crate) context: u32,
    /// Thread id (default: thread that owns the anchor)
    #[usage(long)]
    pub(crate) thread: Option<String>,
    /// Offline body file when the mirror clone is absent
    #[usage(long)]
    pub(crate) body_file: Option<String>,
    /// Fence language (default: guessed from anchor path, else text)
    #[usage(long)]
    pub(crate) lang: Option<String>,
    /// Explicit excerpt id
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
        let loc = match find::find_anchor(&eng, &self.anchor) {
            Ok(l) => l,
            Err(err) => {
                eprintln!("srcthread: {err}");
                return ExitCode::from(1);
            }
        };
        let thread_id = self.thread.unwrap_or(loc.thread_id.clone());
        let lang = self
            .lang
            .unwrap_or_else(|| guess_lang(&loc.anchor.meta.path));
        let body = match load_body(&eng, &loc, self.context, self.body_file.as_deref()) {
            Ok(b) => b,
            Err(err) => {
                eprintln!("srcthread: {err}");
                return ExitCode::from(1);
            }
        };
        let Some(disk) = eng.disk() else {
            eprintln!("srcthread: excerpt add requires an on-disk store");
            return ExitCode::from(1);
        };
        let id = self.id.unwrap_or_else(|| {
            new_entity_id(loc.anchor.meta.symbol.as_deref().unwrap_or("excerpt"))
        });
        let meta = ExcerptMeta {
            id: id.clone(),
            anchor_id: self.anchor,
            title: None,
        };
        match disk.add_excerpt(&thread_id, meta, &lang, &body) {
            Ok(path) => {
                println!("{id}\t{}", path.display());
                ExitCode::SUCCESS
            }
            Err(err) => {
                eprintln!("srcthread: {err}");
                ExitCode::from(1)
            }
        }
    }
}

fn guess_lang(path: &str) -> String {
    PathBuf::from(path)
        .extension()
        .and_then(|e| e.to_str())
        .map(|ext| match ext {
            "rs" => "rust",
            "ts" | "tsx" => "typescript",
            "js" | "jsx" | "mjs" | "cjs" => "javascript",
            "py" => "python",
            "md" => "markdown",
            "toml" => "toml",
            "json" => "json",
            other => other,
        })
        .unwrap_or("text")
        .to_string()
}

fn load_body(
    eng: &srcthread::Srcthread,
    loc: &find::LocatedAnchor,
    context: u32,
    body_file: Option<&str>,
) -> Result<String, String> {
    if let Some(path) = body_file {
        return fs::read_to_string(path).map_err(|e| format!("{path}: {e}"));
    }
    let git = eng
        .disk()
        .and_then(|d| d.mirror_git(&loc.anchor.meta.mirror).map(|s| s.to_string()));
    if let Some(clone) = mirrors_cache::resolve_clone_dir(&loc.anchor.meta.mirror, git.as_deref())
        && let Some(file) = mirrors_cache::resolve_file(&clone, &loc.anchor.meta.path)
    {
        return slice_file(
            &file,
            loc.anchor.meta.line,
            loc.anchor.meta.end_line,
            context,
        );
    }
    // Offline: require stdin body when no mirror / no --body-file.
    if atty_stdin() {
        return Err(format!(
            "mirror not present for {:?}; pass --body-file or pipe body on stdin",
            loc.anchor.meta.mirror
        ));
    }
    let mut buf = String::new();
    io::stdin()
        .read_to_string(&mut buf)
        .map_err(|e| format!("stdin: {e}"))?;
    if buf.is_empty() {
        return Err("empty excerpt body (stdin)".into());
    }
    Ok(buf)
}

fn atty_stdin() -> bool {
    std::io::IsTerminal::is_terminal(&io::stdin())
}

fn slice_file(
    path: &std::path::Path,
    line: Option<u32>,
    end_line: Option<u32>,
    context: u32,
) -> Result<String, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let lines: Vec<&str> = text.lines().collect();
    let Some(start_line) = line else {
        return Ok(text);
    };
    let start_idx = start_line.saturating_sub(1) as usize;
    let end_idx = end_line
        .map(|n| n.saturating_sub(1) as usize)
        .unwrap_or(start_idx);
    let from = start_idx.saturating_sub(context as usize);
    let to = (end_idx + context as usize + 1).min(lines.len());
    Ok(lines[from..to].join("\n") + "\n")
}
