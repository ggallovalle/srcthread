//! Idempotent MCP config merge / remove for each agent format.
//!
//! JSON configs are edited via `jsonc-parser`'s CST so comments, key order,
//! indentation, and trailing commas stay intact (same role as `toml_edit`
//! for Codex).

use std::fs;
use std::path::Path;

use jsonc_parser::cst::CstRootNode;
use jsonc_parser::json;
use toml_edit::{Array, DocumentMut, Item, Table, value};

use super::targets::ConfigFormat;

pub(crate) const SERVER_ID: &str = "srcthread";
const COMMAND: &str = "srcthread";
const ARG_MCP: &str = "mcp";

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum InstallOutcome {
    Installed,
    AlreadyPresent,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum UninstallOutcome {
    Removed,
    NotPresent,
}

/// Whether our server entry is present in the file (missing file => false).
pub(crate) fn is_installed(path: &Path, format: ConfigFormat) -> Result<bool, String> {
    if !path.is_file() {
        return Ok(false);
    }
    let text = fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    match format {
        ConfigFormat::McpServersJson => Ok(json_object_has(&text, path, "mcpServers")?),
        ConfigFormat::OpencodeJson => Ok(json_object_has(&text, path, "mcp")?),
        ConfigFormat::CodexToml => {
            let doc: DocumentMut = text
                .parse()
                .map_err(|e| format!("parse {}: {e}", path.display()))?;
            Ok(codex_has(&doc))
        }
    }
}

pub(crate) fn install(path: &Path, format: ConfigFormat) -> Result<InstallOutcome, String> {
    ensure_parent(path)?;
    match format {
        ConfigFormat::McpServersJson => {
            install_json_server(path, "mcpServers", cursor_claude_entry())
        }
        ConfigFormat::OpencodeJson => install_json_server(path, "mcp", opencode_entry()),
        ConfigFormat::CodexToml => install_codex_toml(path),
    }
}

pub(crate) fn uninstall(path: &Path, format: ConfigFormat) -> Result<UninstallOutcome, String> {
    if !path.is_file() {
        return Ok(UninstallOutcome::NotPresent);
    }
    match format {
        ConfigFormat::McpServersJson => uninstall_json_server(path, "mcpServers"),
        ConfigFormat::OpencodeJson => uninstall_json_server(path, "mcp"),
        ConfigFormat::CodexToml => uninstall_codex_toml(path),
    }
}

fn ensure_parent(path: &Path) -> Result<(), String> {
    let Some(parent) = path.parent() else {
        return Ok(());
    };
    if parent.as_os_str().is_empty() {
        return Ok(());
    }
    if parent.is_dir() {
        return Ok(());
    }
    // Create only the leaf config dir when its parent already exists
    // (e.g. `~/.cursor` for user, project root for local). Refuse to
    // invent a whole agent home tree — the agent should be installed first.
    let Some(grand) = parent.parent() else {
        return fs::create_dir_all(parent).map_err(|e| format!("mkdir {}: {e}", parent.display()));
    };
    if !grand.is_dir() && !grand.as_os_str().is_empty() {
        return Err(format!(
            "agent config directory not found at {} (install the agent first)",
            parent.display()
        ));
    }
    fs::create_dir_all(parent).map_err(|e| format!("mkdir {}: {e}", parent.display()))
}

fn parse_json_root(text: &str, path: &Path) -> Result<CstRootNode, String> {
    CstRootNode::parse(text, &Default::default())
        .map_err(|e| format!("parse {}: {e}", path.display()))
}

fn write_text(path: &Path, text: impl AsRef<str>) -> Result<(), String> {
    let mut out = text.as_ref().to_owned();
    if !out.ends_with('\n') {
        out.push('\n');
    }
    fs::write(path, out).map_err(|e| format!("write {}: {e}", path.display()))
}

fn cursor_claude_entry() -> jsonc_parser::cst::CstInputValue {
    json!({
        "command": COMMAND,
        "args": [ARG_MCP],
    })
}

fn opencode_entry() -> jsonc_parser::cst::CstInputValue {
    json!({
        "type": "local",
        "command": [COMMAND, ARG_MCP],
        "enabled": true,
    })
}

fn json_object_has(text: &str, path: &Path, container: &str) -> Result<bool, String> {
    let root = parse_json_root(text, path)?;
    let Some(obj) = root.object_value() else {
        return Ok(false);
    };
    Ok(obj
        .object_value(container)
        .is_some_and(|servers| servers.get(SERVER_ID).is_some()))
}

fn install_json_server(
    path: &Path,
    container: &str,
    entry: jsonc_parser::cst::CstInputValue,
) -> Result<InstallOutcome, String> {
    let text = if path.is_file() {
        fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?
    } else {
        String::new()
    };
    let root = parse_json_root(&text, path)?;
    let root_obj = root.object_value_or_set();
    let Some(servers) = root_obj.object_value_or_create(container) else {
        return Err(format!("{container} must be an object in {}", path.display()));
    };
    if servers.get(SERVER_ID).is_some() {
        return Ok(InstallOutcome::AlreadyPresent);
    }
    servers.append(SERVER_ID, entry);
    write_text(path, root.to_string())?;
    Ok(InstallOutcome::Installed)
}

fn uninstall_json_server(path: &Path, container: &str) -> Result<UninstallOutcome, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let root = parse_json_root(&text, path)?;
    let Some(root_obj) = root.object_value() else {
        return Ok(UninstallOutcome::NotPresent);
    };
    let Some(servers) = root_obj.object_value(container) else {
        return Ok(UninstallOutcome::NotPresent);
    };
    let Some(prop) = servers.get(SERVER_ID) else {
        return Ok(UninstallOutcome::NotPresent);
    };
    prop.remove();
    write_text(path, root.to_string())?;
    Ok(UninstallOutcome::Removed)
}

fn codex_has(doc: &DocumentMut) -> bool {
    doc.get("mcp_servers")
        .and_then(|i| i.as_table())
        .is_some_and(|t| t.contains_key(SERVER_ID))
}

fn install_codex_toml(path: &Path) -> Result<InstallOutcome, String> {
    let mut doc: DocumentMut = if path.is_file() {
        fs::read_to_string(path)
            .map_err(|e| format!("read {}: {e}", path.display()))?
            .parse()
            .map_err(|e| format!("parse {}: {e}", path.display()))?
    } else {
        DocumentMut::new()
    };
    if !doc.contains_key("mcp_servers") {
        doc["mcp_servers"] = Item::Table(Table::new());
    }
    let Some(servers) = doc["mcp_servers"].as_table_mut() else {
        return Err(format!(
            "{}: mcp_servers must be a table",
            path.display()
        ));
    };
    if servers.contains_key(SERVER_ID) {
        return Ok(InstallOutcome::AlreadyPresent);
    }
    let mut entry = Table::new();
    entry["command"] = value(COMMAND);
    let mut args = Array::new();
    args.push(ARG_MCP);
    entry["args"] = Item::Value(args.into());
    servers.insert(SERVER_ID, Item::Table(entry));
    write_text(path, doc.to_string())?;
    Ok(InstallOutcome::Installed)
}

fn uninstall_codex_toml(path: &Path) -> Result<UninstallOutcome, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let mut doc: DocumentMut = text
        .parse()
        .map_err(|e| format!("parse {}: {e}", path.display()))?;
    let Some(servers) = doc.get_mut("mcp_servers").and_then(|i| i.as_table_mut()) else {
        return Ok(UninstallOutcome::NotPresent);
    };
    if servers.remove(SERVER_ID).is_none() {
        return Ok(UninstallOutcome::NotPresent);
    }
    write_text(path, doc.to_string())?;
    Ok(UninstallOutcome::Removed)
}
