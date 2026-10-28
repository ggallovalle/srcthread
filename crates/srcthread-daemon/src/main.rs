//! `srcthreadd` — Unix socket NDJSON daemon (herdr-shaped protocol).
//!
//! Requests: newline-delimited JSON `{"id","method","params"}`.
//! Responses: `{"id","result"}` or `{"id","error"}`.

use std::env;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use srcthread::Srcthread;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};

#[derive(Debug, Deserialize)]
struct Request {
    id: Value,
    method: String,
    #[serde(default)]
    params: Value,
}

#[derive(Debug, Serialize)]
struct Response {
    id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<Value>,
}

fn default_socket_path() -> PathBuf {
    env::var_os("XDG_RUNTIME_DIR")
        .map(|dir| PathBuf::from(dir).join("srcthread.sock"))
        .unwrap_or_else(|| PathBuf::from("/tmp/srcthread.sock"))
}

fn parse_args() -> (PathBuf, bool) {
    let mut socket = default_socket_path();
    let mut foreground = false;
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--socket" => {
                if let Some(path) = args.next() {
                    socket = PathBuf::from(path);
                }
            }
            "--foreground" => foreground = true,
            "--help" | "-h" => {
                eprintln!(
                    "Usage: srcthreadd [--socket PATH] [--foreground]\n\
                     Default socket: $XDG_RUNTIME_DIR/srcthread.sock or /tmp/srcthread.sock"
                );
                std::process::exit(0);
            }
            other => {
                eprintln!("unknown argument: {other}");
                std::process::exit(2);
            }
        }
    }
    let _ = foreground; // stub: always runs in foreground for hello-world
    (socket, foreground)
}

fn handle_method(engine: &Srcthread, method: &str, _params: &Value) -> Result<Value, String> {
    match method {
        "ping" => Ok(json!("pong")),
        "hello" => Ok(json!(engine.hello())),
        "thread.list" => Ok(json!(engine.list_threads())),
        other => Err(format!("unknown method: {other}")),
    }
}

async fn handle_connection(mut stream: UnixStream, engine: Srcthread) {
    let (reader, mut writer) = stream.split();
    let mut lines = BufReader::new(reader).lines();

    while let Ok(Some(line)) = lines.next_line().await {
        if line.trim().is_empty() {
            continue;
        }

        let response = match serde_json::from_str::<Request>(&line) {
            Ok(req) => match handle_method(&engine, &req.method, &req.params) {
                Ok(result) => Response {
                    id: req.id,
                    result: Some(result),
                    error: None,
                },
                Err(msg) => Response {
                    id: req.id,
                    result: None,
                    error: Some(json!({ "message": msg })),
                },
            },
            Err(err) => Response {
                id: Value::Null,
                result: None,
                error: Some(json!({ "message": format!("invalid request: {err}") })),
            },
        };

        let mut out = serde_json::to_string(&response).unwrap_or_else(|_| {
            r#"{"id":null,"error":{"message":"serialize failed"}}"#.to_string()
        });
        out.push('\n');
        if writer.write_all(out.as_bytes()).await.is_err() {
            break;
        }
    }
}

fn prepare_socket(path: &Path) -> std::io::Result<()> {
    if path.exists() {
        std::fs::remove_file(path)?;
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let (socket_path, _foreground) = parse_args();
    let engine = Srcthread::open_memory();

    if let Err(err) = prepare_socket(&socket_path) {
        eprintln!("failed to prepare socket {}: {err}", socket_path.display());
        std::process::exit(1);
    }

    let listener = match UnixListener::bind(&socket_path) {
        Ok(l) => l,
        Err(err) => {
            eprintln!("failed to bind {}: {err}", socket_path.display());
            std::process::exit(1);
        }
    };

    println!("listening on {}", socket_path.display());

    loop {
        match listener.accept().await {
            Ok((stream, _addr)) => {
                let eng = engine.clone();
                tokio::spawn(async move {
                    handle_connection(stream, eng).await;
                });
            }
            Err(err) => {
                eprintln!("accept error: {err}");
            }
        }
    }
}
