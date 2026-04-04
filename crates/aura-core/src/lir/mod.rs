//! # Aura Low-Level IR (LIR)
//!
//! The LIR breaks semantic HIR nodes into rendering primitives.
//! Backends that cannot directly express HIR concepts (e.g., HTML doesn't
//! have a native "List" widget) consume LIR instead.
//!
//! HIR → LIR lowering is handled by the `lower` module.

/// LIR node — a rendering primitive.
#[derive(Debug, Clone)]
pub enum LIRNode {
    /// A rectangle (background, container, etc.)
    Rect(LIRRect),
    /// Text content
    Text(LIRText),
    /// An image
    Image(LIRImage),
    /// A group of nodes with optional transform
    Group(LIRGroup),
    /// A scrollable container
    Scroll(LIRScroll),
    /// An interactive input
    Input(LIRInput),
    /// A tap/click target
    TouchTarget(LIRTouchTarget),
    /// An animation applied to a child
    Animation(LIRAnimation),
    /// Conditional rendering
    Conditional(LIRConditional),
}

/// Layout primitives.
#[derive(Debug, Clone)]
pub enum LIRLayout {
    /// Flex column (vertical)
    FlexColumn(LIRFlex),
    /// Flex row (horizontal)
    FlexRow(LIRFlex),
    /// Absolute positioning
    Absolute(Vec<LIRPositioned>),
    /// Grid
    Grid(LIRGrid),
}

#[derive(Debug, Clone)]
pub struct LIRRect {
    pub width: LIRDimension,
    pub height: LIRDimension,
    pub fill: Option<String>,
    pub stroke: Option<String>,
    pub stroke_width: f64,
    pub radius: f64,
    pub children: Vec<LIRNode>,
}

#[derive(Debug, Clone)]
pub struct LIRText {
    pub content: String,
    pub font_family: String,
    pub font_size: f64,
    pub font_weight: u16,
    pub color: String,
    pub alignment: super::design::TextAlignment,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
}

#[derive(Debug, Clone)]
pub struct LIRImage {
    pub source: String,
    pub width: LIRDimension,
    pub height: LIRDimension,
    pub fit: ImageFit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFit {
    Cover,
    Contain,
    Fill,
}

#[derive(Debug, Clone)]
pub struct LIRGroup {
    pub layout: LIRLayout,
    pub children: Vec<LIRNode>,
}

#[derive(Debug, Clone)]
pub struct LIRScroll {
    pub direction: super::hir::ScrollDirection,
    pub children: Vec<LIRNode>,
}

#[derive(Debug, Clone)]
pub struct LIRInput {
    pub kind: LIRInputKind,
    pub width: LIRDimension,
    pub height: LIRDimension,
    pub value_binding: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LIRInputKind {
    Text,
    TextArea,
    Checkbox,
    Toggle,
    Slider,
    Select,
    Date,
}

#[derive(Debug, Clone)]
pub struct LIRTouchTarget {
    pub width: LIRDimension,
    pub height: LIRDimension,
    pub action: String, // Reference to action name
    pub children: Vec<LIRNode>,
}

#[derive(Debug, Clone)]
pub struct LIRAnimation {
    pub property: String,
    pub from: f64,
    pub to: f64,
    pub curve: super::design::MotionCurve,
    pub duration_ms: f64,
    pub child: Box<LIRNode>,
}

#[derive(Debug, Clone)]
pub struct LIRConditional {
    pub condition: String, // Expression as string (backend evaluates)
    pub then_nodes: Vec<LIRNode>,
    pub else_nodes: Vec<LIRNode>,
}

#[derive(Debug, Clone)]
pub struct LIRFlex {
    pub gap: f64,
    pub padding_top: f64,
    pub padding_right: f64,
    pub padding_bottom: f64,
    pub padding_left: f64,
    pub align: super::design::Alignment,
    pub justify: super::design::Justification,
}

#[derive(Debug, Clone)]
pub struct LIRPositioned {
    pub x: f64,
    pub y: f64,
    pub node: LIRNode,
}

#[derive(Debug, Clone)]
pub struct LIRGrid {
    pub columns: usize,
    pub row_gap: f64,
    pub column_gap: f64,
}

/// Dimension value — can be fixed, percentage, or auto.
#[derive(Debug, Clone)]
pub enum LIRDimension {
    Fixed(f64),
    Percent(f64),
    Auto,
}
