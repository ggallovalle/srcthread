//! Socket NDJSON daemon library (herdr-shaped protocol).
//!
//! Entered from the `srcthread` CLI via `srcthread daemon` — same product
//! name, no trailing-`d` daemon binary. See
//! `research/08-herdr-single-binary.md`.
//!
//! Requests: newline-delimited JSON `{"id","method","params"}`.
//! Responses: `{"id","result"}` or `{"id","error"}`.

use std::env;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use srcthread::Srcthread;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};

/// Options for [`run`].
#[derive(Debug, Clone)]
pub struct RunOpts {
    /// Unix socket path. `None` → [`default_socket_path`].
    pub socket: Option<PathBuf>,
    /// Reserved for future daemonize; hello-world always foregrounds.
    pub foreground: bool,
}

impl Default for RunOpts {
    fn default() -> Self {
        Self {
            socket: None,
            foreground: true,
        }
    }
}

/// Default socket: `$XDG_RUNTIME_DIR/srcthread.sock` or `/tmp/srcthread.sock`.
pub fn default_socket_path() -> PathBuf {
    env::var_os("XDG_RUNTIME_DIR")
        .map(|dir| PathBuf::from(dir).join("srcthread.sock"))
        .unwrap_or_else(|| PathBuf::from("/tmp/srcthread.sock"))
}

/// Run the daemon until the process exits (foreground listen loop).
pub fn run(opts: RunOpts) -> ExitCode {
    let socket_path = opts.socket.unwrap_or_else(default_socket_path);
    let _ = opts.foreground;
    match tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt.block_on(serve(socket_path)),
        Err(err) => {
            eprintln!("failed to start runtime: {err}");
            ExitCode::from(1)
        }
    }
}

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

fn handle_method(engine: &Srcthread, method: &str, params: &Value) -> Result<Value, String> {
    match method {
        "ping" => Ok(json!("pong")),
        "hello" => Ok(json!(engine.hello())),
        "thread.list" => Ok(json!(engine.list_threads())),
        "thread.create" => {
            let id = params
                .get("id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| "params.id required".to_string())?;
            engine.create_thread(id).map_err(|e| e.to_string())?;
            Ok(json!({ "id": id }))
        }
        "server.stop" => Err("server.stop not implemented in hello-world".into()),
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

async fn serve(socket_path: PathBuf) -> ExitCode {
    let engine = Srcthread::open_memory();

    if let Err(err) = prepare_socket(&socket_path) {
        eprintln!("failed to prepare socket {}: {err}", socket_path.display());
        return ExitCode::from(1);
    }

    let listener = match UnixListener::bind(&socket_path) {
        Ok(l) => l,
        Err(err) => {
            eprintln!("failed to bind {}: {err}", socket_path.display());
            return ExitCode::from(1);
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
