//! Shared CLI helpers (stubs + daemon socket).

use std::env;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::process::ExitCode;

use serde_json::{Value, json};

pub(crate) fn stub(name: &str) -> ExitCode {
    eprintln!("srcthread: `{name}` is not implemented yet");
    ExitCode::from(2)
}

pub(crate) fn default_socket_path() -> PathBuf {
    env::var_os("XDG_RUNTIME_DIR")
        .map(|dir| PathBuf::from(dir).join("srcthread.sock"))
        .unwrap_or_else(|| PathBuf::from("/tmp/srcthread.sock"))
}

pub(crate) fn daemon_call(method: &str, params: Value) -> Result<Value, String> {
    let path = default_socket_path();
    let mut stream =
        UnixStream::connect(&path).map_err(|e| format!("connect {}: {e}", path.display()))?;
    let req = json!({
        "id": 1,
        "method": method,
        "params": params,
    });
    let mut line = serde_json::to_string(&req).map_err(|e| e.to_string())?;
    line.push('\n');
    stream
        .write_all(line.as_bytes())
        .map_err(|e| e.to_string())?;
    let mut reader = BufReader::new(stream);
    let mut response = String::new();
    reader.read_line(&mut response).map_err(|e| e.to_string())?;
    let v: Value = serde_json::from_str(&response).map_err(|e| e.to_string())?;
    if let Some(err) = v.get("error") {
        return Err(err.to_string());
    }
    Ok(v.get("result").cloned().unwrap_or(Value::Null))
}
