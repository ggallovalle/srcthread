use std::process::ExitCode;

use srcthread::{RefKind, RefMeta};
use usage_rs::{Args, Run};

use crate::cli::discover;

#[derive(Args)]
pub(crate) struct Add {
    /// Thread id
    #[usage(long)]
    pub(crate) thread: String,
    /// URL (PR, issue, doc, chat)
    #[usage(long)]
    pub(crate) url: Option<String>,
    /// Optional local transcript / note path to ingest
    #[usage(long)]
    pub(crate) file: Option<String>,
    /// Ref kind (default: inferred from URL, else other)
    #[usage(long)]
    pub(crate) kind: Option<String>,
    /// Display title
    #[usage(long)]
    pub(crate) title: Option<String>,
    /// Explicit ref id
    #[usage(long)]
    pub(crate) id: Option<String>,
}

impl Run for Add {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        if self.url.is_none() && self.file.is_none() {
            eprintln!("srcthread: ref add requires --url and/or --file");
            return ExitCode::from(1);
        }
        let eng = match discover::open_active_store() {
            Ok(e) => e,
            Err(err) => {
                eprintln!("srcthread: {err}");
                return ExitCode::from(1);
            }
        };
        let Some(disk) = eng.disk() else {
            eprintln!("srcthread: ref add requires an on-disk store");
            return ExitCode::from(1);
        };
        let kind = match self.kind.as_deref() {
            Some(k) => match parse_kind(k) {
                Ok(k) => k,
                Err(err) => {
                    eprintln!("srcthread: {err}");
                    return ExitCode::from(1);
                }
            },
            None => infer_kind(self.url.as_deref()),
        };
        let body_path = self.file;
        let title = self.title.or_else(|| {
            self.url
                .as_ref()
                .map(|u| u.rsplit('/').next().unwrap_or(u).to_string())
        });
        let meta = RefMeta {
            kind,
            url: self.url,
            title,
            body_path,
        };
        match disk.add_ref(&self.thread, self.id, meta) {
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

fn parse_kind(s: &str) -> Result<RefKind, String> {
    match s.to_ascii_lowercase().as_str() {
        "pr" => Ok(RefKind::Pr),
        "issue" => Ok(RefKind::Issue),
        "doc" => Ok(RefKind::Doc),
        "chat" => Ok(RefKind::Chat),
        "other" => Ok(RefKind::Other),
        _ => Err(format!("unknown kind {s:?} (pr|issue|doc|chat|other)")),
    }
}

fn infer_kind(url: Option<&str>) -> RefKind {
    let Some(url) = url else {
        return RefKind::Other;
    };
    if url.contains("/pull/") || url.contains("/pulls/") {
        RefKind::Pr
    } else if url.contains("/issues/") {
        RefKind::Issue
    } else {
        RefKind::Doc
    }
}
