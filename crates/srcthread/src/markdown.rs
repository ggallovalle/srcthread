//! TOML frontmatter (`+++`) and GFM fence extraction.

use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};

use crate::error::{Error, Result};

/// Split optional `+++` … `+++` TOML frontmatter from a markdown body.
pub fn split_toml_frontmatter(text: &str) -> Result<(Option<String>, &str)> {
    let trimmed = text.trim_start();
    if !trimmed.starts_with("+++") {
        return Ok((None, text));
    }
    let after_open = &trimmed[3..];
    let after_open = after_open.strip_prefix('\n').unwrap_or(after_open);
    let Some(end) = after_open.find("\n+++") else {
        return Err(Error::toml("unclosed +++ TOML frontmatter"));
    };
    let fm = after_open[..end].to_string();
    let mut rest = &after_open[end + 4..];
    if let Some(stripped) = rest.strip_prefix('\n') {
        rest = stripped;
    }
    Ok((Some(fm), rest))
}

/// One fenced code block from a markdown body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fence {
    /// Info-string language (first token), if any.
    pub lang: Option<String>,
    /// Full info string after the opening fence.
    pub info: String,
    pub body: String,
}

/// Walk markdown with GFM options and collect fenced code blocks.
pub fn extract_fences(markdown: &str) -> Vec<Fence> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_GFM);

    let parser = Parser::new_ext(markdown, options);
    let mut fences = Vec::new();
    let mut current_info: Option<String> = None;
    let mut current_body = String::new();
    let mut in_fence = false;

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(info))) => {
                in_fence = true;
                current_info = Some(info.to_string());
                current_body.clear();
            }
            Event::Text(t) | Event::Code(t) if in_fence => {
                current_body.push_str(&t);
            }
            Event::End(TagEnd::CodeBlock) if in_fence => {
                let info = current_info.take().unwrap_or_default();
                let lang = info
                    .split_whitespace()
                    .next()
                    .filter(|s| !s.is_empty())
                    .map(str::to_string);
                fences.push(Fence {
                    lang,
                    info,
                    body: current_body.clone(),
                });
                in_fence = false;
                current_body.clear();
            }
            _ => {}
        }
    }
    fences
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn splits_frontmatter_and_fences() {
        let text = "+++\nid = \"x\"\n+++\n\n```rust\nfn main() {}\n```\n";
        let (fm, body) = split_toml_frontmatter(text).unwrap();
        assert_eq!(fm.as_deref(), Some("id = \"x\""));
        let fences = extract_fences(body);
        assert_eq!(fences.len(), 1);
        assert_eq!(fences[0].lang.as_deref(), Some("rust"));
        assert!(fences[0].body.contains("fn main"));
    }
}
