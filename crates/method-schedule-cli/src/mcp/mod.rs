//! `method-schedule mcp`: a Model Context Protocol server over stdio, so coding agents can read and
//! edit schedule files with typed tools instead of shell commands (docs/MCP.md). Ported
//! from Method Budget's server, same design.
//!
//! It works on files, not on the running app, and keeps no state between calls: every
//! tool names a schedule by its path and reads it from disk. Reads return the file's
//! `revision` (a fingerprint of its bytes), and every write must hand that revision back,
//! so a write is refused when the file changed on disk since the agent last read it.
//!
//! - [`protocol`]: JSON-RPC 2.0 over newline-delimited stdio, the `initialize`
//!   handshake (revisions 2024-11-05 to 2025-11-25) and the per-request `_meta` of
//!   revision 2026-07-28, cancellation.
//! - [`paths`]: every path resolves inside an allowed root; symlinks that leave it,
//!   hidden folders and unexpected extensions are refused.
//! - [`files`]: revisions, size limits, the checked edit and atomic writes.
//! - [`schema`]: the JSON Schema subset tool inputs are written in, and the validator
//!   every call's arguments go through.
//! - [`tools`]: the tools themselves, thin layers over `method-schedule-core` and `method-schedule-format`.
//!
//! Nothing here runs a program, opens a network connection or deletes a file. Logs go to
//! stderr; stdout carries protocol messages only.

pub mod files;
pub mod paths;
pub mod protocol;
pub mod schema;
pub mod tools;

#[cfg(test)]
mod tests;

use std::path::PathBuf;

/// How the server was started.
#[derive(Clone, Debug)]
pub struct Config {
    /// Folders the server may read and write in (at least one).
    pub roots: Vec<PathBuf>,
    /// Hide every tool that writes a file.
    pub read_only: bool,
}

/// Limits on what the server accepts, so a confused or hostile client can't make it use
/// unbounded memory or time.
pub mod limits {
    /// Longest protocol message (one line of stdin), in bytes.
    pub const MAX_MESSAGE_BYTES: usize = 8 * 1024 * 1024;
    /// Largest schedule or script file the server reads or writes, in bytes (the same as
    /// the command line's own limit).
    pub const MAX_FILE_BYTES: u64 = 64 * 1024 * 1024;
    /// Most strips one `move_strips` call may move.
    pub const MAX_BATCH: usize = 5_000;
    /// Longest synopsis, comment or other free text, in characters.
    pub const MAX_TEXT: usize = 10_000;
    /// Longest name, title or board ID, in characters.
    pub const MAX_NAME: usize = 200;
    /// Longest path argument, in bytes.
    pub const MAX_PATH: usize = 4_096;
    /// Most sheets one `read_schedule` call lists.
    pub const MAX_PAGE: u64 = 5_000;
    /// Most files `list_schedules` returns, and how deep it looks.
    pub const MAX_LISTED_FILES: usize = 2_000;
    pub const MAX_LIST_DEPTH: usize = 8;
    /// Longest event, in days, first and last included (each day is a day out of days
    /// column, so a mistyped year is refused, as the app does; decisions.md, entry 21).
    pub use method_schedule_core::model::MAX_EVENT_DAYS;
    /// Most elements on one event.
    pub const MAX_EVENT_ELEMENTS: usize = 1_000;
    /// Largest day break limit: 10,000 pages in eighths, or 10,000 hours in minutes or
    /// seconds' worth of screen time.
    pub const MAX_LIMIT: i64 = 600_000;
}

/// Runs the server on this process's stdin and stdout until stdin closes.
pub fn serve_stdio(config: Config) -> Result<(), String> {
    let server = protocol::Server::new(&config)?;
    eprintln!(
        "method-schedule mcp: serving {} tool(s) over stdio{}; roots: {}",
        server.tool_count(),
        if config.read_only { " (read-only)" } else { "" },
        server
            .roots()
            .iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );
    let source = protocol::spawn_reader(std::io::stdin());
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    protocol::run(server, source, &mut out).map_err(|e| format!("stdout: {e}"))
}
