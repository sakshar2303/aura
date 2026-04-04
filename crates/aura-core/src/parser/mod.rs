//! # Aura Parser
//!
//! Parses a token stream into an AST using the chumsky parser combinator library.
//!
//! ## Design Principles
//! - Error recovery: the parser produces a partial AST even on error
//! - All errors collected (does not stop at first)
//! - Indentation tracked as Indent/Dedent virtual tokens
//! - Zero ambiguity: every valid program has exactly one parse tree

// Parser implementation will be built incrementally.
// Phase 1: basic declarations (app, model, screen)
// Phase 1.5: view elements (layout, widgets, buttons)
// Phase 2: expressions, actions, control flow
// Phase 3: full grammar

/// Parse result containing the AST and any errors.
pub struct ParseResult {
    pub program: Option<crate::ast::Program>,
    pub errors: Vec<crate::errors::AuraError>,
}
