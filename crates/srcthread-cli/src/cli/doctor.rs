use std::process::ExitCode;

use usage_rs::{Args, Run};

use crate::cli::{ctx, discover, mirrors_cache};

/// Environment / store health checks
#[derive(Args)]
#[usage(verbatim_doc_comment)]
pub(crate) struct Doctor {}

impl Run for Doctor {
    type Output = ExitCode;

    fn run(self) -> Self::Output {
        println!("xdg_config:\t{}", ctx::xdg_config_home().display());
        println!("stores_toml:\t{}", ctx::stores_toml_path().display());
        println!("mirrors_dir:\t{}", mirrors_cache::mirrors_root().display());

        match discover::resolve_active_store_root() {
            Ok(root) => {
                println!("store:\t{}", root.display());
                match srcthread::Srcthread::open(&root) {
                    Ok(eng) => {
                        println!("config:\tok");
                        match eng.list_thread_meta() {
                            Ok(threads) => println!("threads:\t{}", threads.len()),
                            Err(err) => {
                                println!("threads:\terror ({err})");
                                return ExitCode::from(1);
                            }
                        }
                        match srcthread::lint_store(&root) {
                            Ok(diags) => {
                                println!("lint:\t{} finding(s)", diags.len());
                                for d in diags.iter().take(20) {
                                    println!("  {}\t{}", d.path, d.message);
                                }
                                if diags.len() > 20 {
                                    println!("  … {} more", diags.len() - 20);
                                }
                                if diags.is_empty() {
                                    ExitCode::SUCCESS
                                } else {
                                    ExitCode::from(1)
                                }
                            }
                            Err(err) => {
                                println!("lint:\terror ({err})");
                                ExitCode::from(1)
                            }
                        }
                    }
                    Err(err) => {
                        println!("config:\terror ({err})");
                        ExitCode::from(1)
                    }
                }
            }
            Err(err) => {
                println!("store:\tnot found ({err})");
                ExitCode::from(1)
            }
        }
    }
}
