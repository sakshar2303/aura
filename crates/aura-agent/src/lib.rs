//! # Aura Agent API
//!
//! JSON-RPC protocol for AI coding agents to interact with Aura programs
//! structurally — reading ASTs, getting diagnostics with confidence-scored
//! fixes, and querying completions.
//!
//! ## Protocol
//! Communication over stdin/stdout using JSON-RPC 2.0.
//!
//! ## Methods
//! - `ast.get` — Get the full AST as JSON for a source file
//! - `diagnostics.get` — Get errors/warnings with fix suggestions
//! - `completions.get` — Get available design tokens, types, and variables
//! - `hir.get` — Get the HIR (high-level IR) as JSON
//! - `explain` — Get plain English explanation
//! - `sketch` — Generate .aura code from a description

mod protocol;
mod server;

pub use protocol::*;
pub use server::AgentServer;
