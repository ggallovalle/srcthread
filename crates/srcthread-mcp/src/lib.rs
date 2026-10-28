//! MCP server stub for srcthread.
//!
//! Entered from the `srcthread` CLI via `srcthread mcp`. Hello-world exposes
//! one query (`thread.list` / hello) and one mutation (`thread.create`) over
//! the same in-process engine (or the daemon socket when present).

use std::io::{self, BufRead, Write};
use std::process::ExitCode;

use serde_json::{Value, json};
use srcthread::Srcthread;

/// Run a minimal JSON-RPC-ish stdio loop for hello-world tools.
///
/// Lines in: `{"id", "method", "params"}`  
/// Lines out: `{"id", "result"}` or `{"id", "error"}`  
/// Methods: `hello`, `thread.list` (query), `thread.create` (mutation).
pub fn run() -> ExitCode {
    let engine = Srcthread::open_memory();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    eprintln!("srcthread-mcp hello (stdio; methods: hello, thread.list, thread.create)");

    for line in stdin.lock().lines() {
        let Ok(line) = line else {
            break;
        };
        if line.trim().is_empty() {
            continue;
        }
        let response = match serde_json::from_str::<Value>(&line) {
            Ok(req) => {
                let id = req.get("id").cloned().unwrap_or(Value::Null);
                let method = req.get("method").and_then(|m| m.as_str()).unwrap_or("");
                let params = req.get("params").cloned().unwrap_or(Value::Null);
                match dispatch(&engine, method, &params) {
                    Ok(result) => json!({ "id": id, "result": result }),
                    Err(message) => json!({ "id": id, "error": { "message": message } }),
                }
            }
            Err(err) => {
                json!({ "id": null, "error": { "message": format!("invalid json: {err}") } })
            }
        };
        if writeln!(stdout, "{response}").is_err() {
            break;
        }
        let _ = stdout.flush();
    }
    ExitCode::SUCCESS
}

fn dispatch(engine: &Srcthread, method: &str, params: &Value) -> Result<Value, String> {
    match method {
        "hello" | "tools/hello" => Ok(json!(engine.hello())),
        "thread.list" | "tools/list_threads" => Ok(json!(engine.list_threads())),
        "thread.create" | "tools/create_thread" => {
            let id = params
                .get("id")
                .or_else(|| params.get("title"))
                .and_then(|v| v.as_str())
                .ok_or_else(|| "params.id required".to_string())?;
            engine.create_thread(id).map_err(|e| e.to_string())?;
            Ok(json!({ "id": id, "threads": engine.list_threads() }))
        }
        other => Err(format!("unknown method: {other}")),
    }
}
