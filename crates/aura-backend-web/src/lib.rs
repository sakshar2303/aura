//! # Aura Web Backend
//!
//! Generates HTML/CSS/JavaScript from Aura HIR/LIR.
//!
//! ## Output Structure
//! - `index.html` — App shell
//! - `styles.css` — Design tokens as CSS custom properties + component styles
//! - `app.js` — Reactive state management + event handlers
//!
//! ## Design Token Mapping
//! - Spacing tokens → CSS custom properties (`--spacing-md: 0.5rem`)
//! - Typography → CSS font properties
//! - Colors → CSS custom properties with light/dark variants
//! - Shapes → CSS border-radius
//! - Motion → CSS transitions/animations

// Web backend will be implemented in Phase 2.
