use std::process::ExitCode;

use usage_rs::{Args, Run};

use crate::cli::find;

#[derive(Args)]
pub(crate) struct Show {
    /// Anchor id
    pub(crate) id: String,
}

impl Run for Show {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        match find::open_and_find_anchor(&self.id) {
            Ok((_eng, loc)) => {
                let a = &loc.anchor;
                println!("id:\t{}", a.id);
                println!("thread:\t{}", loc.thread_id);
                println!("mirror:\t{}", a.meta.mirror);
                println!("sha:\t{}", a.meta.sha);
                println!("path:\t{}", a.meta.path);
                if let Some(line) = a.meta.line {
                    println!("line:\t{line}");
                }
                if let Some(end) = a.meta.end_line {
                    println!("end_line:\t{end}");
                }
                if let Some(sym) = &a.meta.symbol {
                    println!("symbol:\t{sym}");
                }
                if let Some(title) = &a.meta.title {
                    println!("title:\t{title}");
                }
                println!("file:\t{}", a.path.display());
                ExitCode::SUCCESS
            }
            Err(err) => {
                eprintln!("srcthread: {err}");
                ExitCode::from(1)
            }
        }
    }
}
