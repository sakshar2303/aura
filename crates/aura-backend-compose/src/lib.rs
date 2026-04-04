//! # Aura Jetpack Compose Backend
//!
//! Generates Kotlin/Jetpack Compose code from Aura HIR.
//!
//! ## Strategy
//! Consumes HIR directly — Compose has native equivalents:
//! - HIRColumn → Column
//! - HIRRow → Row
//! - HIRList/Each → LazyColumn/items
//! - HIRTextField → TextField
//! - HIRToggle → Switch
//! - HIRNavigate → NavController.navigate
//!
//! ## Output
//! Kotlin files with @Composable functions + Gradle build config.

// Compose backend will be implemented in Phase 3.
