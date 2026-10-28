//! Slug + 10-char Crockford ULID identifiers.

use ulid::Ulid;

const CROCKFORD: &[u8] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

/// Slugify a human title into a path-safe lowercase slug.
pub fn slugify(title: &str) -> String {
    let mut out = String::new();
    let mut prev_dash = true;
    for ch in title.chars() {
        let c = ch.to_ascii_lowercase();
        if c.is_ascii_alphanumeric() {
            out.push(c);
            prev_dash = false;
        } else if !prev_dash {
            out.push('-');
            prev_dash = true;
        }
    }
    let trimmed = out.trim_matches('-');
    if trimmed.is_empty() {
        "thread".into()
    } else {
        trimmed.to_string()
    }
}

/// 10-character Crockford-coded suffix (time-sortable fragment of a ULID).
pub fn short_ulid_suffix() -> String {
    let u = Ulid::new();
    // Full ULID is 26 Crockford chars; take the leading 10 (timestamp-heavy).
    let full = u.to_string();
    full.chars().take(10).collect()
}

/// `slug` + `-` + 10-char ULID suffix.
pub fn new_entity_id(title_or_slug: &str) -> String {
    format!("{}-{}", slugify(title_or_slug), short_ulid_suffix())
}

/// Validate Crockford alphabet for the suffix portion (best-effort).
#[allow(dead_code)]
pub(crate) fn is_crockford_char(c: char) -> bool {
    CROCKFORD.contains(&(c.to_ascii_uppercase() as u8))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn slugify_basic() {
        assert_eq!(slugify("How is Deno.env.get?"), "how-is-deno-env-get");
        assert_eq!(slugify("!!!"), "thread");
    }

    #[test]
    fn entity_id_shape() {
        let id = new_entity_id("Deno env get");
        let (slug, suffix) = id.rsplit_once('-').expect("dash");
        assert_eq!(slug, "deno-env-get");
        assert_eq!(suffix.len(), 10);
    }
}
