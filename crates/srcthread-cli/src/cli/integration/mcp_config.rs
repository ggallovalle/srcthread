//! Idempotent MCP config merge / remove for each agent format.

use std::fs;
use std::path::Path;

use serde_json::{Map, Value, json};
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
        ConfigFormat::McpServersJson => Ok(mcp_servers_has(&parse_json_object(&text, path)?)),
        ConfigFormat::OpencodeJson => Ok(opencode_has(&parse_json_object(&text, path)?)),
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
        ConfigFormat::McpServersJson => install_mcp_servers_json(path),
        ConfigFormat::OpencodeJson => install_opencode_json(path),
        ConfigFormat::CodexToml => install_codex_toml(path),
    }
}

pub(crate) fn uninstall(path: &Path, format: ConfigFormat) -> Result<UninstallOutcome, String> {
    if !path.is_file() {
        return Ok(UninstallOutcome::NotPresent);
    }
    match format {
        ConfigFormat::McpServersJson => uninstall_mcp_servers_json(path),
        ConfigFormat::OpencodeJson => uninstall_opencode_json(path),
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

fn parse_json_object(text: &str, path: &Path) -> Result<Map<String, Value>, String> {
    if text.trim().is_empty() {
        return Ok(Map::new());
    }
    let v: Value =
        serde_json::from_str(text).map_err(|e| format!("parse {}: {e}", path.display()))?;
    match v {
        Value::Object(map) => Ok(map),
        _ => Err(format!("{}: expected a JSON object", path.display())),
    }
}

fn write_json(path: &Path, map: &Map<String, Value>) -> Result<(), String> {
    let text = serde_json::to_string_pretty(map).map_err(|e| e.to_string())?;
    fs::write(path, text + "\n").map_err(|e| format!("write {}: {e}", path.display()))
}

fn server_entry_cursor_claude() -> Value {
    json!({
        "command": COMMAND,
        "args": [ARG_MCP],
    })
}

fn mcp_servers_has(root: &Map<String, Value>) -> bool {
    root.get("mcpServers")
        .and_then(|v| v.as_object())
        .is_some_and(|m| m.contains_key(SERVER_ID))
}

fn install_mcp_servers_json(path: &Path) -> Result<InstallOutcome, String> {
    let mut root = if path.is_file() {
        parse_json_object(
            &fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?,
            path,
        )?
    } else {
        Map::new()
    };
    let servers = root
        .entry("mcpServers".to_string())
        .or_insert_with(|| Value::Object(Map::new()));
    let Some(obj) = servers.as_object_mut() else {
        return Err(format!("{}: mcpServers must be an object", path.display()));
    };
    if obj.contains_key(SERVER_ID) {
        return Ok(InstallOutcome::AlreadyPresent);
    }
    obj.insert(SERVER_ID.to_string(), server_entry_cursor_claude());
    write_json(path, &root)?;
    Ok(InstallOutcome::Installed)
}

fn uninstall_mcp_servers_json(path: &Path) -> Result<UninstallOutcome, String> {
    let mut root = parse_json_object(
        &fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?,
        path,
    )?;
    let Some(servers) = root.get_mut("mcpServers").and_then(|v| v.as_object_mut()) else {
        return Ok(UninstallOutcome::NotPresent);
    };
    if servers.remove(SERVER_ID).is_none() {
        return Ok(UninstallOutcome::NotPresent);
    }
    write_json(path, &root)?;
    Ok(UninstallOutcome::Removed)
}

fn opencode_entry() -> Value {
    json!({
        "type": "local",
        "command": [COMMAND, ARG_MCP],
        "enabled": true,
    })
}

fn opencode_has(root: &Map<String, Value>) -> bool {
    root.get("mcp")
        .and_then(|v| v.as_object())
        .is_some_and(|m| m.contains_key(SERVER_ID))
}

fn install_opencode_json(path: &Path) -> Result<InstallOutcome, String> {
    let mut root = if path.is_file() {
        parse_json_object(
            &fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?,
            path,
        )?
    } else {
        Map::new()
    };
    let mcp = root
        .entry("mcp".to_string())
        .or_insert_with(|| Value::Object(Map::new()));
    let Some(obj) = mcp.as_object_mut() else {
        return Err(format!("{}: mcp must be an object", path.display()));
    };
    if obj.contains_key(SERVER_ID) {
        return Ok(InstallOutcome::AlreadyPresent);
    }
    obj.insert(SERVER_ID.to_string(), opencode_entry());
    write_json(path, &root)?;
    Ok(InstallOutcome::Installed)
}

fn uninstall_opencode_json(path: &Path) -> Result<UninstallOutcome, String> {
    let mut root = parse_json_object(
        &fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?,
        path,
    )?;
    let Some(mcp) = root.get_mut("mcp").and_then(|v| v.as_object_mut()) else {
        return Ok(UninstallOutcome::NotPresent);
    };
    if mcp.remove(SERVER_ID).is_none() {
        return Ok(UninstallOutcome::NotPresent);
    }
    write_json(path, &root)?;
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
    fs::write(path, doc.to_string()).map_err(|e| format!("write {}: {e}", path.display()))?;
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
    fs::write(path, doc.to_string()).map_err(|e| format!("write {}: {e}", path.display()))?;
    Ok(UninstallOutcome::Removed)
}
