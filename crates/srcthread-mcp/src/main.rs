//! `srcthread-mcp` — stdio MCP stub.
//!
//! Real MCP (tools/resources over JSON-RPC stdio) comes last in the build
//! order, after the library, daemon, and CLI surfaces stabilize.

fn main() {
    println!("srcthread-mcp hello (stdio MCP stub)");
}
