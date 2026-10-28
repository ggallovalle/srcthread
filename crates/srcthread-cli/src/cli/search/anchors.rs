use std::process::ExitCode;

use usage_rs::{Args, Run};

use crate::cli::discover;

#[derive(Args)]
pub(crate) struct Anchors {
    /// Query string
    pub(crate) query: String,
}

impl Run for Anchors {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        let eng = match discover::open_active_store() {
            Ok(e) => e,
            Err(err) => {
                eprintln!("srcthread: {err}");
                return ExitCode::from(1);
            }
        };
        let q = self.query.to_ascii_lowercase();
        let metas = match eng.list_thread_meta() {
            Ok(m) => m,
            Err(err) => {
                eprintln!("srcthread: {err}");
                return ExitCode::from(1);
            }
        };
        for meta in metas {
            let t = match eng.load_thread(&meta.id) {
                Ok(t) => t,
                Err(err) => {
                    eprintln!("srcthread: {}: {err}", meta.id);
                    continue;
                }
            };
            for a in t.anchors {
                let hay = format!(
                    "{} {} {} {} {} {}",
                    a.id,
                    a.meta.mirror,
                    a.meta.path,
                    a.meta.sha,
                    a.meta.symbol.as_deref().unwrap_or(""),
                    a.meta.title.as_deref().unwrap_or("")
                )
                .to_ascii_lowercase();
                if hay.contains(&q) {
                    println!("{}\t{}\t{}\t{}", meta.id, a.id, a.meta.path, a.meta.mirror);
                }
            }
        }
        ExitCode::SUCCESS
    }
}
