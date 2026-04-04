//! # Aura Design Token System
//!
//! Design tokens are first-class language constructs — part of Aura's grammar.
//! They define spacing, typography, color, shape, and motion.
//!
//! Tokens are resolved against a theme to produce platform-specific values.
//! See spec/language.md Section 6 for the complete reference.

use serde::{Deserialize, Serialize};

/// Resolved design properties for an HIR node.
///
/// Every HIR node carries this — it contains the computed design values
/// after theme resolution.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResolvedDesign {
    pub spacing: Option<ResolvedSpacing>,
    pub typography: Option<ResolvedTypography>,
    pub color: Option<ResolvedColor>,
    pub shape: Option<ResolvedShape>,
    pub motion: Option<ResolvedMotion>,
    pub size: Option<ResolvedSize>,
    pub alignment: Option<ResolvedAlignment>,
    pub shadow: Option<ResolvedShadow>,
    pub opacity: Option<f64>,
}

/// Resolved spacing values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedSpacing {
    pub gap: Option<f64>,
    pub padding_top: Option<f64>,
    pub padding_bottom: Option<f64>,
    pub padding_left: Option<f64>,
    pub padding_right: Option<f64>,
    pub margin_top: Option<f64>,
    pub margin_bottom: Option<f64>,
    pub margin_left: Option<f64>,
    pub margin_right: Option<f64>,
}

/// Resolved typography values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedTypography {
    pub size: Option<f64>,
    pub weight: Option<u16>,
    pub italic: bool,
    pub mono: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub alignment: Option<TextAlignment>,
    pub transform: Option<TextTransform>,
    pub font_family: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextAlignment {
    Leading,
    Center,
    Trailing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextTransform {
    Uppercase,
    Lowercase,
    Capitalize,
}

/// Resolved color values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedColor {
    pub foreground: Option<String>,
    pub background: Option<String>,
}

/// Resolved shape values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedShape {
    pub radius: f64,
    pub kind: ShapeKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShapeKind {
    Sharp,
    Subtle,
    Rounded,
    Smooth,
    Pill,
    Circle,
}

/// Resolved motion/animation values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedMotion {
    pub curve: MotionCurve,
    pub duration_ms: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MotionCurve {
    Ease,
    Spring,
    Bounce,
    Linear,
}

/// Resolved sizing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedSize {
    pub width: Option<SizeValue>,
    pub height: Option<SizeValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SizeValue {
    Fixed(f64),
    Fill,
    Fit,
}

/// Resolved alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolvedAlignment {
    pub align: Alignment,
    pub justify: Option<Justification>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Alignment {
    Start,
    Center,
    End,
    Stretch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Justification {
    Start,
    Center,
    End,
    Between,
    Around,
}

/// Resolved shadow.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedShadow {
    pub x: f64,
    pub y: f64,
    pub blur: f64,
    pub spread: f64,
    pub color: String,
}

/// Spacing token names and their base multipliers.
///
/// With a base of 8px:
/// - .xs = 2px  (0.25x)
/// - .sm = 4px  (0.5x)
/// - .md = 8px  (1.0x)
/// - .lg = 16px (2.0x)
/// - .xl = 24px (3.0x)
/// - .2xl = 32px (4.0x)
/// - .3xl = 48px (6.0x)
/// - .4xl = 64px (8.0x)
pub fn spacing_multiplier(token: &str) -> Option<f64> {
    match token {
        "xs" => Some(0.25),
        "sm" => Some(0.5),
        "md" => Some(1.0),
        "lg" => Some(2.0),
        "xl" => Some(3.0),
        "2xl" => Some(4.0),
        "3xl" => Some(6.0),
        "4xl" => Some(8.0),
        _ => None,
    }
}

/// Resolve a spacing token to a concrete pixel value.
pub fn resolve_spacing(token: &str, base: f64) -> Option<f64> {
    spacing_multiplier(token).map(|m| m * base)
}

/// Typography size tokens (in base rem units for web, points for native).
pub fn typography_size(token: &str) -> Option<f64> {
    match token {
        "xs" => Some(0.75),
        "sm" => Some(0.875),
        "md" => Some(1.0),
        "lg" => Some(1.125),
        "xl" => Some(1.25),
        "2xl" => Some(1.5),
        "3xl" => Some(2.125),
        "display" => Some(3.0),
        _ => None,
    }
}

/// Font weight tokens.
pub fn font_weight(token: &str) -> Option<u16> {
    match token {
        "thin" => Some(100),
        "light" => Some(300),
        "regular" => Some(400),
        "medium" => Some(500),
        "semibold" => Some(600),
        "bold" => Some(700),
        "heavy" => Some(800),
        "black" => Some(900),
        _ => None,
    }
}

/// Shape radius tokens (in pixels).
pub fn shape_radius(token: &str) -> Option<f64> {
    match token {
        "sharp" => Some(0.0),
        "subtle" => Some(4.0),
        "rounded" => Some(8.0),
        "smooth" => Some(12.0),
        "pill" => Some(9999.0),
        _ => None,
    }
}

/// Motion duration tokens (in milliseconds).
pub fn motion_duration(token: &str) -> Option<f64> {
    match token {
        "instant" => Some(0.0),
        "fast" => Some(150.0),
        "normal" => Some(300.0),
        "slow" => Some(500.0),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spacing_resolution() {
        assert_eq!(resolve_spacing("xs", 8.0), Some(2.0));
        assert_eq!(resolve_spacing("md", 8.0), Some(8.0));
        assert_eq!(resolve_spacing("lg", 8.0), Some(16.0));
        assert_eq!(resolve_spacing("2xl", 8.0), Some(32.0));
        assert_eq!(resolve_spacing("invalid", 8.0), None);
    }

    #[test]
    fn test_custom_base_spacing() {
        // With a base of 4px instead of 8px:
        assert_eq!(resolve_spacing("md", 4.0), Some(4.0));
        assert_eq!(resolve_spacing("lg", 4.0), Some(8.0));
    }

    #[test]
    fn test_font_weights() {
        assert_eq!(font_weight("bold"), Some(700));
        assert_eq!(font_weight("thin"), Some(100));
        assert_eq!(font_weight("invalid"), None);
    }

    #[test]
    fn test_shape_radius() {
        assert_eq!(shape_radius("sharp"), Some(0.0));
        assert_eq!(shape_radius("rounded"), Some(8.0));
        assert_eq!(shape_radius("pill"), Some(9999.0));
    }
}
