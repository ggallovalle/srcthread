//! Cold-walk helpers to locate entities across threads.

use srcthread::{LoadedAnchor, LoadedExcerpt, Srcthread};

use crate::cli::discover::{DiscoverError, open_active_store};

pub(crate) struct LocatedAnchor {
    pub thread_id: String,
    pub anchor: LoadedAnchor,
}

pub(crate) fn open_and_find_anchor(id: &str) -> Result<(Srcthread, LocatedAnchor), DiscoverError> {
    let eng = open_active_store()?;
    let located = find_anchor(&eng, id)?;
    Ok((eng, located))
}

pub(crate) fn find_anchor(eng: &Srcthread, id: &str) -> Result<LocatedAnchor, DiscoverError> {
    let metas = eng
        .list_thread_meta()
        .map_err(|e| DiscoverError::Message(e.to_string()))?;
    for meta in metas {
        let t = eng
            .load_thread(&meta.id)
            .map_err(|e| DiscoverError::Message(e.to_string()))?;
        if let Some(anchor) = t.anchors.iter().find(|a| a.id == id).cloned() {
            return Ok(LocatedAnchor {
                thread_id: meta.id,
                anchor,
            });
        }
    }
    Err(DiscoverError::Message(format!("anchor not found: {id}")))
}

pub(crate) fn find_excerpt(
    eng: &Srcthread,
    id: &str,
) -> Result<(String, LoadedExcerpt), DiscoverError> {
    let metas = eng
        .list_thread_meta()
        .map_err(|e| DiscoverError::Message(e.to_string()))?;
    for meta in metas {
        let t = eng
            .load_thread(&meta.id)
            .map_err(|e| DiscoverError::Message(e.to_string()))?;
        if let Some(ex) = t.excerpts.iter().find(|e| e.meta.id == id).cloned() {
            return Ok((meta.id, ex));
        }
    }
    Err(DiscoverError::Message(format!("excerpt not found: {id}")))
}
