//! Resolve-only mirror cache paths (no fetch).

use std::path::{Path, PathBuf};

/// Root of the shared mirror clone cache.
pub(crate) fn mirrors_root() -> PathBuf {
    if let Some(p) = std::env::var_os("SRCTHREAD_MIRRORS_DIR") {
        return PathBuf::from(p);
    }
    xdg_cache_home().join("srcthread").join("mirrors")
}

fn xdg_cache_home() -> PathBuf {
    if let Some(p) = std::env::var_os("XDG_CACHE_HOME") {
        return PathBuf::from(p);
    }
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".cache")
}

/// Normalize a git URL into `host/owner/repo` (or host/path) cache key.
pub(crate) fn normalize_git_cache_key(git: &str) -> String {
    let s = git.trim();
    let s = s.strip_suffix(".git").unwrap_or(s);
    if let Some(rest) = s.strip_prefix("git@") {
        // git@host:owner/repo
        let rest = rest.replace(':', "/");
        return rest.trim_matches('/').to_string();
    }
    let s = s
        .strip_prefix("https://")
        .or_else(|| s.strip_prefix("http://"))
        .or_else(|| s.strip_prefix("ssh://"))
        .unwrap_or(s);
    let s = s.strip_prefix("git@").unwrap_or(s);
    s.trim_matches('/').to_string()
}

/// Absolute path to a mirror clone if present (by git URL or mirror id dir).
pub(crate) fn resolve_clone_dir(mirror_id: &str, git: Option<&str>) -> Option<PathBuf> {
    let root = mirrors_root();
    if let Some(git) = git {
        let key = normalize_git_cache_key(git);
        let p = root.join(&key);
        if p.is_dir() {
            return Some(p);
        }
    }
    let by_id = root.join(mirror_id);
    if by_id.is_dir() {
        return Some(by_id);
    }
    None
}

/// Join clone dir + repo-relative path; None if file missing.
pub(crate) fn resolve_file(clone: &Path, rel: &str) -> Option<PathBuf> {
    let p = clone.join(rel);
    if p.is_file() { Some(p) } else { None }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_github_https() {
        assert_eq!(
            normalize_git_cache_key("https://github.com/denoland/deno.git"),
            "github.com/denoland/deno"
        );
        assert_eq!(
            normalize_git_cache_key("git@github.com:denoland/deno.git"),
            "github.com/denoland/deno"
        );
    }
}
