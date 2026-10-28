use std::process::ExitCode;

use usage_rs::{Args, Run};

use crate::cli::discover;

/// Show one thread
#[derive(Args)]
pub(crate) struct Show {
    /// Thread id
    pub(crate) id: String,
}

impl Run for Show {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        let eng = match discover::open_active_store() {
            Ok(e) => e,
            Err(err) => {
                eprintln!("srcthread: {err}");
                return ExitCode::from(1);
            }
        };
        match eng.load_thread(&self.id) {
            Ok(t) => {
                println!("id:\t{}", t.meta.id);
                println!("title:\t{}", t.meta.title);
                println!("tags:\t{}", t.meta.tags.join(", "));
                println!("anchors:\t{}", t.anchors.len());
                println!("excerpts:\t{}", t.excerpts.len());
                println!("refs:\t{}", t.refs.len());
                println!();
                print!("{}", t.notes);
                ExitCode::SUCCESS
            }
            Err(err) => {
                eprintln!("srcthread: {err}");
                ExitCode::from(1)
            }
        }
    }
}
