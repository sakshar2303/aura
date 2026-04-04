//! # Aura SwiftUI Backend
//!
//! Generates SwiftUI code from Aura HIR.
//!
//! ## Strategy
//! This backend consumes HIR directly (not LIR) because SwiftUI has native
//! equivalents for most Aura HIR concepts:
//! - HIRColumn → VStack
//! - HIRRow → HStack
//! - HIRList/Each → List/ForEach
//! - HIRTextField → TextField
//! - HIRToggle → Toggle
//! - HIRNavigate → NavigationLink
//!
//! ## Output
//! A complete Xcode project with SwiftUI views, models, and navigation.

// SwiftUI backend will be implemented in Phase 3.
