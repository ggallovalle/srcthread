//! Load anchors, excerpts, refs, and notes for one thread.

use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{Error, Result};
use crate::markdown::{Fence, extract_fences, split_toml_frontmatter};
use crate::types::{AnchorMeta, ExcerptMeta, RefMeta, ThreadMeta};

/// Loaded anchor file (toml-only or markdown with optional embedded fences).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadedAnchor {
    pub id: String,
    pub meta: AnchorMeta,
    pub path: PathBuf,
    pub embedded_fences: Vec<Fence>,
}

/// Loaded standalone excerpt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadedExcerpt {
    pub meta: ExcerptMeta,
    pub path: PathBuf,
    pub fences: Vec<Fence>,
}

/// Loaded ref.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadedRef {
    pub id: String,
    pub meta: RefMeta,
    pub path: PathBuf,
}

/// Full thread payload from disk.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadedThread {
    pub meta: ThreadMeta,
    pub notes: String,
    pub anchors: Vec<LoadedAnchor>,
    pub excerpts: Vec<LoadedExcerpt>,
    pub refs: Vec<LoadedRef>,
    pub dir: PathBuf,
}

fn file_stem_id(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
        .to_string()
}

fn load_anchor_file(path: &Path) -> Result<LoadedAnchor> {
    let text = fs::read_to_string(path)?;
    let id = file_stem_id(path);
    let (meta, fences) = if path.extension().and_then(|e| e.to_str()) == Some("md") {
        let (fm, body) = split_toml_frontmatter(&text)?;
        let fm =
            fm.ok_or_else(|| Error::toml(format!("anchor md missing +++ frontmatter: {id}")))?;
        let meta: AnchorMeta = toml::from_str(&fm).map_err(Error::toml)?;
        (meta, extract_fences(body))
    } else {
        let meta: AnchorMeta = toml::from_str(&text).map_err(Error::toml)?;
        (meta, Vec::new())
    };
    Ok(LoadedAnchor {
        id,
        meta,
        path: path.to_path_buf(),
        embedded_fences: fences,
    })
}

fn load_excerpt_file(path: &Path) -> Result<LoadedExcerpt> {
    let text = fs::read_to_string(path)?;
    let (fm, body) = split_toml_frontmatter(&text)?;
    let fm = fm.ok_or_else(|| {
        Error::toml(format!(
            "excerpt missing +++ frontmatter: {}",
            path.display()
        ))
    })?;
    let meta: ExcerptMeta = toml::from_str(&fm).map_err(Error::toml)?;
    Ok(LoadedExcerpt {
        meta,
        path: path.to_path_buf(),
        fences: extract_fences(body),
    })
}

fn load_ref_file(path: &Path) -> Result<LoadedRef> {
    let text = fs::read_to_string(path)?;
    let meta: RefMeta = toml::from_str(&text).map_err(Error::toml)?;
    Ok(LoadedRef {
        id: file_stem_id(path),
        meta,
        path: path.to_path_buf(),
    })
}

/// Load a thread directory (`threads/<id>/`).
pub fn load_thread_dir(dir: &Path) -> Result<LoadedThread> {
    let meta_path = dir.join("thread.toml");
    let text = fs::read_to_string(&meta_path)?;
    let meta: ThreadMeta = toml::from_str(&text).map_err(Error::toml)?;

    let notes_path = dir.join("notes.md");
    let notes = if notes_path.is_file() {
        fs::read_to_string(notes_path)?
    } else {
        String::new()
    };

    let mut anchors = Vec::new();
    let anchors_dir = dir.join("anchors");
    if anchors_dir.is_dir() {
        for entry in fs::read_dir(&anchors_dir)? {
            let entry = entry?;
            let path = entry.path();
            let ext = path.extension().and_then(|e| e.to_str());
            if matches!(ext, Some("toml") | Some("md")) {
                anchors.push(load_anchor_file(&path)?);
            }
        }
        anchors.sort_by(|a, b| a.id.cmp(&b.id));
    }

    let mut excerpts = Vec::new();
    let excerpts_dir = dir.join("excerpts");
    if excerpts_dir.is_dir() {
        for entry in fs::read_dir(&excerpts_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("md") {
                excerpts.push(load_excerpt_file(&path)?);
            }
        }
        excerpts.sort_by(|a, b| a.meta.id.cmp(&b.meta.id));
    }

    let mut refs = Vec::new();
    let refs_dir = dir.join("refs");
    if refs_dir.is_dir() {
        for entry in fs::read_dir(&refs_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("toml") {
                refs.push(load_ref_file(&path)?);
            }
        }
        refs.sort_by(|a, b| a.id.cmp(&b.id));
    }

    Ok(LoadedThread {
        meta,
        notes,
        anchors,
        excerpts,
        refs,
        dir: dir.to_path_buf(),
    })
}
