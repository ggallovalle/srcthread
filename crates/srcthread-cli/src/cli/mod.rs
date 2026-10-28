//! CLI surface shaped like mise + herdr: one module per top-level command,
//! nested groups as directories, global flags on the root `Cli`, and a
//! **single distributable binary** that also runs the desktop UI and MCP.
//!
//! See `research/07-mise-cli-structure.md`,
//! `research/08-herdr-single-binary.md`, and `research/09-no-daemon.md`.

mod anchor;
mod desktop;
mod doctor;
mod excerpt;
mod hello;
mod integration;
mod mcp;
mod mirror;
mod r#ref;
mod search;
mod store;
mod sync;
mod thread;
mod util;

use usage_rs::{Cli, Subcommands};

/// Local-first research threads over pinned source mirrors.
///
/// Declare stores, pin source mirrors, and keep research threads with
/// anchors and offline excerpts. The same binary runs the rune desktop UI
/// (`srcthread desktop`) and the MCP server (`srcthread mcp`).
#[derive(Cli)]
#[usage(bin = "srcthread", version = env!("CARGO_PKG_VERSION"), verbatim_doc_comment)]
pub(crate) struct SrcthreadCli {
    #[usage(subcommand)]
    pub(crate) command: Commands,

    /// Change to DIR before running (and prefer its store, later)
    #[usage(short = 'C', long, global = true, value_name = "DIR")]
    pub(crate) cd: Option<String>,

    /// Active store name or path (overrides default selection)
    #[usage(long, global = true, value_name = "STORE")]
    pub(crate) store: Option<String>,

    /// Suppress non-error messages
    #[usage(short = 'q', long, global = true)]
    pub(crate) quiet: bool,

    /// Extra output (-vv for more)
    #[usage(short = 'v', long, global = true, count)]
    pub(crate) verbose: u8,

    /// Answer yes to confirmation prompts
    #[usage(short = 'y', long, global = true)]
    pub(crate) yes: bool,
}

#[derive(Subcommands)]
#[usage(run)]
pub(crate) enum Commands {
    /// Open the rune desktop UI
    Desktop(desktop::Desktop),
    /// Manage file-canonical research stores (git vaults)
    Store(store::Store),
    /// Research threads (primary object)
    #[usage(visible_alias = "t")]
    Thread(thread::Thread),
    /// Anchors into mirrored source at a pinned revision
    #[usage(visible_alias = "a")]
    Anchor(anchor::Anchor),
    /// Offline frozen excerpts around anchors
    Excerpt(excerpt::Excerpt),
    /// External refs (PR, issue, doc, chat)
    Ref(r#ref::RefCmd),
    /// Shared upstream mirror cache
    Mirror(mirror::Mirror),
    /// Search threads or anchors in the active store
    Search(search::Search),
    /// Sync stores via git or nearby peer transfer
    Sync(sync::Sync),
    /// Run the MCP stdio server (stub)
    Mcp(mcp::Mcp),
    /// Manage built-in agent MCP integrations
    Integration(integration::Integration),
    /// Environment / store health checks
    #[usage(visible_alias = "dr")]
    Doctor(doctor::Doctor),
    /// Hello-world: in-process lib greeting
    Hello(hello::Hello),
}
