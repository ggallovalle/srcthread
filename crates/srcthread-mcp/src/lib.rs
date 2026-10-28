//! MCP server for srcthread using the official
//! [`rmcp`](https://github.com/modelcontextprotocol/rust-sdk) SDK.
//!
//! Entered from the `srcthread` CLI via `srcthread mcp`. Hello-world exposes
//! one query pair (`hello`, `list_threads`) and one mutation (`create_thread`)
//! over the in-process engine.

use std::process::ExitCode;

use rmcp::{
    ServerHandler, ServiceExt, handler::server::wrapper::Parameters, schemars, tool, tool_handler,
    tool_router, transport::stdio,
};
use serde::Deserialize;
use srcthread::Srcthread;

/// Stdio MCP server wrapping a [`Srcthread`] engine.
#[derive(Clone)]
pub struct SrcthreadMcp {
    engine: Srcthread,
}

impl Default for SrcthreadMcp {
    fn default() -> Self {
        Self::new()
    }
}

impl SrcthreadMcp {
    /// Build a server over an in-memory engine.
    pub fn new() -> Self {
        Self {
            engine: Srcthread::open_memory(),
        }
    }
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct CreateThreadParams {
    /// Thread id to create
    id: String,
}

#[tool_router]
impl SrcthreadMcp {
    /// Query: project greeting token.
    #[tool(description = "Return the srcthread greeting token (query)")]
    fn hello(&self) -> String {
        self.engine.hello().to_string()
    }

    /// Query: list known research thread ids.
    #[tool(description = "List research thread ids in the store (query)")]
    fn list_threads(&self) -> String {
        self.engine.list_threads().join("\n")
    }

    /// Mutation: create a research thread id.
    #[tool(description = "Create a research thread id (mutation)")]
    fn create_thread(
        &self,
        Parameters(CreateThreadParams { id }): Parameters<CreateThreadParams>,
    ) -> String {
        match self.engine.create_thread(id.clone()) {
            Ok(()) => format!("created {id}\n{}", self.engine.list_threads().join("\n")),
            Err(err) => format!("error: {err}"),
        }
    }
}

#[tool_handler(
    name = "srcthread",
    instructions = "Local-first research threads. Tools: hello + list_threads (query), create_thread (mutation)."
)]
impl ServerHandler for SrcthreadMcp {}

/// Serve MCP over stdio until the client disconnects.
pub fn run() -> ExitCode {
    match tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt.block_on(serve()),
        Err(err) => {
            eprintln!("failed to start runtime: {err}");
            ExitCode::from(1)
        }
    }
}

async fn serve() -> ExitCode {
    let server = SrcthreadMcp::new();
    match server.serve(stdio()).await {
        Ok(service) => match service.waiting().await {
            Ok(_reason) => ExitCode::SUCCESS,
            Err(err) => {
                eprintln!("mcp wait error: {err}");
                ExitCode::from(1)
            }
        },
        Err(err) => {
            eprintln!("mcp serve error: {err}");
            ExitCode::from(1)
        }
    }
}
