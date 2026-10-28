//! Disk mutations for stores and threads.

use crate::error::{Error, Result};
use crate::ids::new_entity_id;
use crate::store::DiskStore;
use crate::types::{
    AnchorMeta, ExcerptMeta, MirrorSpec, RefMeta, StoreConfig, StoreMeta, ThreadMeta,
};
use std::fs;
use std::path::{Path, PathBuf};

fn rfc3339_now() -> String {
    // Use ULID timestamp millis for ordering; format as Zulu via ulid's time.
    let ms = ulid::Ulid::new().timestamp_ms();
    let secs = ms / 1000;
    let days = secs / 86400;
    let rem = secs % 86400;
    let hour = rem / 3600;
    let min = (rem % 3600) / 60;
    let sec = rem % 60;
    // Civil date from days since 1970-01-01 (Howard Hinnant algorithm).
    let (y, m, d) = civil_from_days(days as i64);
    format!("{y:04}-{m:02}-{d:02}T{hour:02}:{min:02}:{sec:02}Z")
}

fn civil_from_days(z: i64) -> (i32, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = (yoe as i64) + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y as i32, m as u32, d as u32)
}

/// Create a new store directory with `config.toml`.
///
/// If `path` already contains `config.toml`, errors. Creates parent dirs.
pub fn init_store(path: &Path, meta: StoreMeta) -> Result<PathBuf> {
    let config_path = path.join("config.toml");
    if config_path.exists() {
        return Err(Error::toml(format!(
            "store already exists at {}",
            path.display()
        )));
    }
    fs::create_dir_all(path)?;
    fs::create_dir_all(path.join("threads"))?;
    let cfg = StoreConfig {
        store: meta,
        mirrors: Default::default(),
    };
    let body = toml::to_string_pretty(&cfg).map_err(Error::toml)?;
    fs::write(&config_path, body)?;
    Ok(path.to_path_buf())
}

impl DiskStore {
    fn thread_dir(&self, id: &str) -> PathBuf {
        self.root.join("threads").join(id)
    }

    /// Create a thread directory from a human title (slug + ULID).
    pub fn create_thread_from_title(&self, title: &str) -> Result<ThreadMeta> {
        let id = new_entity_id(title);
        self.create_thread_with_meta(ThreadMeta {
            id,
            title: title.to_string(),
            tags: Vec::new(),
            created: rfc3339_now(),
            updated: rfc3339_now(),
        })
    }

    /// Create a thread with an explicit id/meta.
    pub fn create_thread_with_meta(&self, meta: ThreadMeta) -> Result<ThreadMeta> {
        if meta.id.trim().is_empty() {
            return Err(Error::EmptyThreadId);
        }
        let dir = self.thread_dir(&meta.id);
        if dir.exists() {
            return Err(Error::ThreadExists(meta.id));
        }
        fs::create_dir_all(dir.join("anchors"))?;
        fs::create_dir_all(dir.join("excerpts"))?;
        fs::create_dir_all(dir.join("refs"))?;
        let body = toml::to_string_pretty(&meta).map_err(Error::toml)?;
        fs::write(dir.join("thread.toml"), body)?;
        fs::write(dir.join("notes.md"), "")?;
        Ok(meta)
    }

    /// Overwrite `thread.toml`.
    pub fn update_thread_meta(&self, meta: &ThreadMeta) -> Result<()> {
        let dir = self.thread_dir(&meta.id);
        if !dir.is_dir() {
            return Err(Error::ThreadNotFound(meta.id.clone()));
        }
        let mut meta = meta.clone();
        meta.updated = rfc3339_now();
        let body = toml::to_string_pretty(&meta).map_err(Error::toml)?;
        fs::write(dir.join("thread.toml"), body)?;
        Ok(())
    }

    /// Replace `notes.md` body.
    pub fn set_notes(&self, thread_id: &str, notes: &str) -> Result<()> {
        let dir = self.thread_dir(thread_id);
        if !dir.is_dir() {
            return Err(Error::ThreadNotFound(thread_id.to_string()));
        }
        fs::write(dir.join("notes.md"), notes)?;
        Ok(())
    }

    /// Path to a thread's notes file.
    pub fn notes_path(&self, thread_id: &str) -> Result<PathBuf> {
        let dir = self.thread_dir(thread_id);
        if !dir.is_dir() {
            return Err(Error::ThreadNotFound(thread_id.to_string()));
        }
        Ok(dir.join("notes.md"))
    }

    /// Remove a thread directory tree.
    pub fn remove_thread(&self, thread_id: &str) -> Result<()> {
        let dir = self.thread_dir(thread_id);
        if !dir.is_dir() {
            return Err(Error::ThreadNotFound(thread_id.to_string()));
        }
        fs::remove_dir_all(dir)?;
        Ok(())
    }

    /// Write an anchor as `anchors/<id>.toml`.
    pub fn add_anchor_toml(
        &self,
        thread_id: &str,
        id: Option<String>,
        meta: AnchorMeta,
    ) -> Result<String> {
        let cfg = self.read_config()?;
        if !cfg.mirrors.contains_key(&meta.mirror) {
            return Err(Error::toml(format!(
                "mirror {:?} not in [mirrors]",
                meta.mirror
            )));
        }
        let dir = self.thread_dir(thread_id);
        if !dir.is_dir() {
            return Err(Error::ThreadNotFound(thread_id.to_string()));
        }
        let id = id
            .unwrap_or_else(|| new_entity_id(meta.symbol.as_deref().unwrap_or(meta.path.as_str())));
        let path = dir.join("anchors").join(format!("{id}.toml"));
        if path.exists() {
            return Err(Error::toml(format!("anchor already exists: {id}")));
        }
        let body = toml::to_string_pretty(&meta).map_err(Error::toml)?;
        fs::write(path, body)?;
        Ok(id)
    }

    /// Write a standalone excerpt markdown file.
    pub fn add_excerpt(
        &self,
        thread_id: &str,
        meta: ExcerptMeta,
        lang: &str,
        body: &str,
    ) -> Result<PathBuf> {
        let dir = self.thread_dir(thread_id);
        if !dir.is_dir() {
            return Err(Error::ThreadNotFound(thread_id.to_string()));
        }
        let path = dir.join("excerpts").join(format!("{}.md", meta.id));
        if path.exists() {
            return Err(Error::toml(format!("excerpt already exists: {}", meta.id)));
        }
        let fm = toml::to_string_pretty(&meta).map_err(Error::toml)?;
        let mut file = String::new();
        file.push_str("+++\n");
        file.push_str(&fm);
        if !fm.ends_with('\n') {
            file.push('\n');
        }
        file.push_str("+++\n\n");
        file.push_str("```");
        file.push_str(lang);
        file.push('\n');
        file.push_str(body);
        if !body.ends_with('\n') {
            file.push('\n');
        }
        file.push_str("```\n");
        fs::write(&path, file)?;
        Ok(path)
    }

    /// Write `refs/<id>.toml`.
    pub fn add_ref(&self, thread_id: &str, id: Option<String>, meta: RefMeta) -> Result<String> {
        let dir = self.thread_dir(thread_id);
        if !dir.is_dir() {
            return Err(Error::ThreadNotFound(thread_id.to_string()));
        }
        let id = id.unwrap_or_else(|| new_entity_id(meta.title.as_deref().unwrap_or("ref")));
        let path = dir.join("refs").join(format!("{id}.toml"));
        if path.exists() {
            return Err(Error::toml(format!("ref already exists: {id}")));
        }
        let body = toml::to_string_pretty(&meta).map_err(Error::toml)?;
        fs::write(path, body)?;
        Ok(id)
    }

    /// Upsert a mirror entry in `config.toml` (reloads in-memory config).
    pub fn upsert_mirror(&mut self, id: &str, git: &str) -> Result<()> {
        let mut cfg = self.read_config()?;
        cfg.mirrors.insert(
            id.to_string(),
            MirrorSpec {
                git: git.to_string(),
            },
        );
        let body = toml::to_string_pretty(&cfg).map_err(Error::toml)?;
        fs::write(self.root.join("config.toml"), body)?;
        self.config = cfg;
        Ok(())
    }

    fn read_config(&self) -> Result<StoreConfig> {
        let text = fs::read_to_string(self.root.join("config.toml"))?;
        toml::from_str(&text).map_err(Error::toml)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use tempfile::tempdir;

    #[test]
    fn init_create_thread_anchor_excerpt_ref() {
        let dir = tempdir().unwrap();
        let root = init_store(
            dir.path(),
            StoreMeta {
                name: "test".into(),
                description: None,
                authors: vec![],
                license: None,
                tags: vec![],
                edition: None,
            },
        )
        .unwrap();
        let mut store = crate::store::open_disk(&root).unwrap();
        store
            .upsert_mirror("deno", "https://github.com/denoland/deno")
            .unwrap();
        let store = crate::store::open_disk(&root).unwrap();
        let meta = store.create_thread_from_title("Hello World").unwrap();
        assert!(meta.id.starts_with("hello-world-"));
        let aid = store
            .add_anchor_toml(
                &meta.id,
                Some("a1".into()),
                AnchorMeta {
                    mirror: "deno".into(),
                    sha: "abc".into(),
                    path: "x.rs".into(),
                    line: Some(1),
                    end_line: None,
                    symbol: Some("foo".into()),
                    title: None,
                },
            )
            .unwrap();
        assert_eq!(aid, "a1");
        store
            .add_excerpt(
                &meta.id,
                ExcerptMeta {
                    id: "e1".into(),
                    anchor_id: "a1".into(),
                    title: None,
                },
                "rust",
                "fn foo() {}",
            )
            .unwrap();
        store
            .add_ref(
                &meta.id,
                Some("r1".into()),
                RefMeta {
                    kind: crate::types::RefKind::Doc,
                    url: Some("https://example.com".into()),
                    title: Some("ex".into()),
                    body_path: None,
                },
            )
            .unwrap();
        let loaded = store.load_thread(&meta.id).unwrap();
        assert_eq!(loaded.anchors.len(), 1);
        assert_eq!(loaded.excerpts.len(), 1);
        assert_eq!(loaded.refs.len(), 1);
        store.remove_thread(&meta.id).unwrap();
        assert!(store.get_thread(&meta.id).is_err());
    }
}
