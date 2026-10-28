use std::process::ExitCode;

use srcthread::RefKind;
use usage_rs::{Args, Run};

use crate::cli::discover;

#[derive(Args)]
pub(crate) struct List {
    /// Thread id
    #[usage(long)]
    pub(crate) thread: String,
}

impl Run for List {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        let eng = match discover::open_active_store() {
            Ok(e) => e,
            Err(err) => {
                eprintln!("srcthread: {err}");
                return ExitCode::from(1);
            }
        };
        match eng.load_thread(&self.thread) {
            Ok(t) => {
                for r in t.refs {
                    let url = r.meta.url.as_deref().unwrap_or("-");
                    let title = r.meta.title.as_deref().unwrap_or("-");
                    println!("{}\t{}\t{url}\t{title}", r.id, kind_str(r.meta.kind));
                }
                ExitCode::SUCCESS
            }
            Err(err) => {
                eprintln!("srcthread: {err}");
                ExitCode::from(1)
            }
        }
    }
}

fn kind_str(k: RefKind) -> &'static str {
    match k {
        RefKind::Pr => "pr",
        RefKind::Issue => "issue",
        RefKind::Doc => "doc",
        RefKind::Chat => "chat",
        RefKind::Other => "other",
    }
}
