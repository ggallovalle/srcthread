use std::process::ExitCode;

use usage_rs::{Args, Run};

use crate::cli::{find, mirrors_cache};

#[derive(Args)]
pub(crate) struct Open {
    /// Anchor id
    pub(crate) id: String,
}

impl Run for Open {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        let (eng, loc) = match find::open_and_find_anchor(&self.id) {
            Ok(v) => v,
            Err(err) => {
                eprintln!("srcthread: {err}");
                return ExitCode::from(1);
            }
        };
        let git = eng
            .disk()
            .and_then(|d| d.mirror_git(&loc.anchor.meta.mirror).map(|s| s.to_string()));
        let Some(clone) = mirrors_cache::resolve_clone_dir(&loc.anchor.meta.mirror, git.as_deref())
        else {
            eprintln!(
                "srcthread: mirror not present for {:?} (expected under {})",
                loc.anchor.meta.mirror,
                mirrors_cache::mirrors_root().display()
            );
            return ExitCode::from(1);
        };
        let Some(file) = mirrors_cache::resolve_file(&clone, &loc.anchor.meta.path) else {
            eprintln!(
                "srcthread: mirror file missing: {}/{}",
                clone.display(),
                loc.anchor.meta.path
            );
            return ExitCode::from(1);
        };
        match loc.anchor.meta.line {
            Some(line) => println!("{}:{line}", file.display()),
            None => println!("{}", file.display()),
        }
        ExitCode::SUCCESS
    }
}
