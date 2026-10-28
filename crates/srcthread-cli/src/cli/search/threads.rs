use std::process::ExitCode;

use usage_rs::{Args, Run};

use crate::cli::discover;

#[derive(Args)]
pub(crate) struct Threads {
    /// Query string
    pub(crate) query: String,
}

impl Run for Threads {
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
            let hay_id = meta.id.to_ascii_lowercase();
            let hay_title = meta.title.to_ascii_lowercase();
            let notes_hit = eng
                .load_thread(&meta.id)
                .ok()
                .is_some_and(|t| t.notes.to_ascii_lowercase().contains(&q));
            if hay_id.contains(&q) || hay_title.contains(&q) || notes_hit {
                println!("{}\t{}", meta.id, meta.title);
            }
        }
        ExitCode::SUCCESS
    }
}
