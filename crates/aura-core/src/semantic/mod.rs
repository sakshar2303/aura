//! # Aura Semantic Analysis
//!
//! The semantic analyzer performs:
//! 1. Name resolution
//! 2. Type inference and checking
//! 3. Security type enforcement (E0200-E0299)
//! 4. State mutation validation (E0300-E0399)
//! 5. Design token resolution (E0400-E0499)
//! 6. Error poisoning (suppress cascade errors)
//!
//! Input: AST
//! Output: Typed HIR + errors

// Semantic analysis will be implemented in Phase 1.
