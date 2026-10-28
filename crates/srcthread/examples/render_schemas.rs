//! Write schemars JSON Schema files into `schema/`.
//!
//! ```sh
//! cargo run -p srcthread --example render_schemas
//! ```

use std::env;
use std::fs;
use std::path::PathBuf;

use srcthread::schema::{
    anchor_meta_schema, excerpt_meta_schema, ref_meta_schema, store_config_schema,
    thread_meta_schema,
};

fn main() {
    let root = env::var_os("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .map(|p| p.join("../.."))
        .unwrap_or_else(|| PathBuf::from("."));
    let root = root.canonicalize().unwrap_or(root);
    let out = root.join("schema");
    fs::create_dir_all(&out).expect("create schema/");

    let files = [
        ("srcthread.store.json", store_config_schema()),
        ("srcthread.thread.json", thread_meta_schema()),
        ("srcthread.anchor.json", anchor_meta_schema()),
        ("srcthread.excerpt.json", excerpt_meta_schema()),
        ("srcthread.ref.json", ref_meta_schema()),
    ];
    for (name, schema) in files {
        let path = out.join(name);
        let json = serde_json::to_string_pretty(&schema).expect("serialize schema");
        fs::write(&path, format!("{json}\n")).expect("write schema");
        println!("wrote {}", path.display());
    }
}
