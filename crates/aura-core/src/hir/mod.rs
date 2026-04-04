//! # Aura High-Level IR (HIR)
//!
//! The HIR preserves semantic intent. It describes WHAT the UI should do, not HOW.
//! Backends like SwiftUI and Jetpack Compose consume HIR directly because they have
//! native equivalents for high-level concepts (List, NavigationStack, Toggle, etc.).
//!
//! Backends that cannot express HIR concepts natively (HTML/CSS) consume
//! the LIR (Low-level IR) instead, which is produced by lowering HIR.

pub mod nodes;

pub use nodes::*;
