use std::fs;

use pretty_assertions::assert_eq;

use super::install::install_with;
use super::mcp_config::{self, InstallOutcome, UninstallOutcome, SERVER_ID};
use super::scope::{Roots, Scope};
use super::status::status_with;
use super::targets::Target;
use super::uninstall::uninstall_with;

fn setup() -> (tempfile::TempDir, Roots) {
    let tmp = tempfile::tempdir().expect("tempdir");
    let roots = Roots::for_test(tmp.path());
    fs::create_dir_all(roots.home.join(".cursor")).unwrap();
    fs::create_dir_all(&roots.home).unwrap();
    fs::create_dir_all(roots.config_home.join("opencode")).unwrap();
    fs::create_dir_all(roots.home.join(".codex")).unwrap();
    fs::create_dir_all(roots.cwd.join(".cursor")).unwrap();
    fs::create_dir_all(roots.cwd.join(".codex")).unwrap();
    fs::create_dir_all(&roots.cwd).unwrap();
    (tmp, roots)
}

#[test]
fn cursor_user_install_uninstall_idempotent() {
    let (_tmp, roots) = setup();
    let path = Target::Cursor.config_path(Scope::User, &roots);

    assert_eq!(
        mcp_config::install(&path, Target::Cursor.format()).unwrap(),
        InstallOutcome::Installed
    );
    assert!(mcp_config::is_installed(&path, Target::Cursor.format()).unwrap());
    assert_eq!(
        mcp_config::install(&path, Target::Cursor.format()).unwrap(),
        InstallOutcome::AlreadyPresent
    );

    let text = fs::read_to_string(&path).unwrap();
    let v: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert_eq!(
        v["mcpServers"][SERVER_ID]["command"],
        serde_json::json!("srcthread")
    );
    assert_eq!(
        v["mcpServers"][SERVER_ID]["args"],
        serde_json::json!(["mcp"])
    );

    assert_eq!(
        mcp_config::uninstall(&path, Target::Cursor.format()).unwrap(),
        UninstallOutcome::Removed
    );
    assert!(!mcp_config::is_installed(&path, Target::Cursor.format()).unwrap());
    assert_eq!(
        mcp_config::uninstall(&path, Target::Cursor.format()).unwrap(),
        UninstallOutcome::NotPresent
    );
}

#[test]
fn claude_local_uses_mcp_json() {
    let (_tmp, roots) = setup();
    let path = Target::Claude.config_path(Scope::Local, &roots);
    install_with(&roots, Target::Claude, Scope::Local).unwrap();
    assert!(path.ends_with(".mcp.json"));
    assert!(mcp_config::is_installed(&path, Target::Claude.format()).unwrap());
}

#[test]
fn opencode_user_mcp_shape() {
    let (_tmp, roots) = setup();
    let path = Target::Opencode.config_path(Scope::User, &roots);
    mcp_config::install(&path, Target::Opencode.format()).unwrap();
    let v: serde_json::Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
    assert_eq!(v["mcp"][SERVER_ID]["type"], "local");
    assert_eq!(
        v["mcp"][SERVER_ID]["command"],
        serde_json::json!(["srcthread", "mcp"])
    );
    assert_eq!(v["mcp"][SERVER_ID]["enabled"], true);
}

#[test]
fn codex_toml_round_trip_preserves_other_keys() {
    let (_tmp, roots) = setup();
    let path = Target::Codex.config_path(Scope::User, &roots);
    fs::write(&path, "model = \"gpt-5\"\n\n[projects.\"/tmp\"]\ntrust_level = \"trusted\"\n")
        .unwrap();
    mcp_config::install(&path, Target::Codex.format()).unwrap();
    let text = fs::read_to_string(&path).unwrap();
    assert!(text.contains("model = \"gpt-5\""));
    assert!(text.contains("[mcp_servers.srcthread]"));
    assert!(text.contains("command = \"srcthread\""));
    mcp_config::uninstall(&path, Target::Codex.format()).unwrap();
    let text = fs::read_to_string(&path).unwrap();
    assert!(!text.contains("srcthread"));
    assert!(text.contains("model = \"gpt-5\""));
}

#[test]
fn refuses_missing_agent_parent_dir() {
    let tmp = tempfile::tempdir().unwrap();
    let roots = Roots {
        home: tmp.path().join("missing-home"),
        config_home: tmp.path().join("missing-config"),
        cwd: tmp.path().join("missing-project"),
    };
    let err = install_with(&roots, Target::Cursor, Scope::User).unwrap_err();
    assert!(
        err.contains("not found"),
        "expected missing-agent error, got: {err}"
    );
}

#[test]
fn status_and_helpers_cover_all_targets() {
    let (_tmp, roots) = setup();
    install_with(&roots, Target::Cursor, Scope::User).unwrap();
    uninstall_with(&roots, Target::Cursor, Scope::Local).unwrap();
    status_with(&roots, &[Scope::User, Scope::Local]).unwrap();
}
