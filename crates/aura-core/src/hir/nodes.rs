//! HIR node definitions.
//!
//! Every HIR node carries resolved design tokens and type information.

use crate::design::ResolvedDesign;
use crate::types::AuraType;

/// A complete HIR module (compiled from one .aura file or project).
#[derive(Debug, Clone)]
pub struct HIRModule {
    pub app: HIRApp,
    pub models: Vec<HIRModel>,
    pub screens: Vec<HIRScreen>,
    pub components: Vec<HIRComponent>,
    pub themes: Vec<HIRTheme>,
}

/// The app-level declaration.
#[derive(Debug, Clone)]
pub struct HIRApp {
    pub name: String,
    pub theme: Option<String>,
    pub navigation: NavigationMode,
    pub routes: Vec<HIRRoute>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationMode {
    Stack,
    Tabs,
}

#[derive(Debug, Clone)]
pub struct HIRRoute {
    pub pattern: String,
    pub screen: String,
}

/// A data model.
#[derive(Debug, Clone)]
pub struct HIRModel {
    pub name: String,
    pub fields: Vec<HIRField>,
}

#[derive(Debug, Clone)]
pub struct HIRField {
    pub name: String,
    pub field_type: AuraType,
    pub default: Option<HIRExpr>,
}

/// A screen.
#[derive(Debug, Clone)]
pub struct HIRScreen {
    pub name: String,
    pub params: Vec<HIRParam>,
    pub state: Vec<HIRState>,
    pub view: HIRView,
    pub actions: Vec<HIRAction>,
    pub functions: Vec<HIRFunction>,
    pub tab: Option<HIRTab>,
}

#[derive(Debug, Clone)]
pub struct HIRTab {
    pub icon: String,
    pub label: String,
}

/// A reusable component.
#[derive(Debug, Clone)]
pub struct HIRComponent {
    pub name: String,
    pub props: Vec<HIRParam>,
    pub state: Vec<HIRState>,
    pub view: HIRView,
    pub actions: Vec<HIRAction>,
    pub functions: Vec<HIRFunction>,
}

#[derive(Debug, Clone)]
pub struct HIRParam {
    pub name: String,
    pub param_type: AuraType,
    pub default: Option<HIRExpr>,
}

#[derive(Debug, Clone)]
pub struct HIRState {
    pub name: String,
    pub state_type: AuraType,
    pub initial: Option<HIRExpr>,
}

/// A theme definition.
#[derive(Debug, Clone)]
pub struct HIRTheme {
    pub name: String,
    pub spacing_base: f64,
    pub radius: String,
    pub font_family: String,
    pub type_scale: f64,
    pub palette: Vec<(String, String)>,
    pub variants: Vec<String>,
}

/// The view tree (UI hierarchy).
#[derive(Debug, Clone)]
pub enum HIRView {
    // Layout
    Column(HIRLayout),
    Row(HIRLayout),
    Stack(HIRLayout),
    Grid(HIRGridLayout),
    Scroll(HIRScrollLayout),
    Wrap(HIRLayout),

    // Widgets
    Text(HIRText),
    Heading(HIRHeading),
    Image(HIRImage),
    Icon(HIRIcon),
    Badge(HIRBadge),
    Progress(HIRProgress),
    Avatar(HIRAvatar),

    // Inputs
    TextField(HIRTextField),
    TextArea(HIRTextArea),
    Checkbox(HIRCheckbox),
    Toggle(HIRToggle),
    Slider(HIRSlider),
    Picker(HIRPicker),
    DatePicker(HIRDatePicker),
    Segmented(HIRSegmented),

    // Buttons
    Button(HIRButton),

    // Control flow
    Conditional(HIRConditional),
    Each(HIREach),
    Switch(HIRSwitch),

    // Structure
    ComponentRef(HIRComponentRef),
    Spacer,
    Divider(ResolvedDesign),
    Slot,

    // Animation
    Animate(HIRAnimate),

    // Container (multiple children, no layout semantics)
    Group(Vec<HIRView>),
}

/// Layout properties shared by column, row, stack, wrap.
#[derive(Debug, Clone)]
pub struct HIRLayout {
    pub design: ResolvedDesign,
    pub children: Vec<HIRView>,
}

#[derive(Debug, Clone)]
pub struct HIRGridLayout {
    pub columns: Option<usize>,
    pub min_width: Option<f64>,
    pub design: ResolvedDesign,
    pub children: Vec<HIRView>,
}

#[derive(Debug, Clone)]
pub struct HIRScrollLayout {
    pub direction: ScrollDirection,
    pub design: ResolvedDesign,
    pub children: Vec<HIRView>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollDirection {
    Vertical,
    Horizontal,
    Both,
}

#[derive(Debug, Clone)]
pub struct HIRText {
    pub content: HIRExpr,
    pub design: ResolvedDesign,
}

#[derive(Debug, Clone)]
pub struct HIRHeading {
    pub content: HIRExpr,
    pub level: u8, // 1-6
    pub design: ResolvedDesign,
}

#[derive(Debug, Clone)]
pub struct HIRImage {
    pub source: HIRExpr,
    pub design: ResolvedDesign,
}

#[derive(Debug, Clone)]
pub struct HIRIcon {
    pub name: HIRExpr,
    pub design: ResolvedDesign,
}

#[derive(Debug, Clone)]
pub struct HIRBadge {
    pub content: HIRExpr,
    pub design: ResolvedDesign,
}

#[derive(Debug, Clone)]
pub struct HIRProgress {
    pub value: HIRExpr,
    pub design: ResolvedDesign,
}

#[derive(Debug, Clone)]
pub struct HIRAvatar {
    pub source: HIRExpr,
    pub design: ResolvedDesign,
}

#[derive(Debug, Clone)]
pub struct HIRTextField {
    pub binding: String,
    pub placeholder: Option<String>,
    pub action: Option<Box<HIRActionExpr>>,
    pub design: ResolvedDesign,
}

#[derive(Debug, Clone)]
pub struct HIRTextArea {
    pub binding: String,
    pub placeholder: Option<String>,
    pub design: ResolvedDesign,
}

#[derive(Debug, Clone)]
pub struct HIRCheckbox {
    pub binding: String,
    pub design: ResolvedDesign,
}

#[derive(Debug, Clone)]
pub struct HIRToggle {
    pub binding: String,
    pub label: Option<String>,
    pub design: ResolvedDesign,
}

#[derive(Debug, Clone)]
pub struct HIRSlider {
    pub binding: String,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub design: ResolvedDesign,
}

#[derive(Debug, Clone)]
pub struct HIRPicker {
    pub binding: String,
    pub options: HIRExpr,
    pub design: ResolvedDesign,
}

#[derive(Debug, Clone)]
pub struct HIRDatePicker {
    pub binding: String,
    pub label: Option<String>,
    pub design: ResolvedDesign,
}

#[derive(Debug, Clone)]
pub struct HIRSegmented {
    pub binding: String,
    pub options: HIRExpr,
    pub design: ResolvedDesign,
}

#[derive(Debug, Clone)]
pub struct HIRButton {
    pub label: HIRExpr,
    pub style: ButtonStyle,
    pub action: HIRActionExpr,
    pub design: ResolvedDesign,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonStyle {
    Default,
    Icon,
    Outline,
    Ghost,
    Link,
}

#[derive(Debug, Clone)]
pub struct HIRConditional {
    pub condition: HIRExpr,
    pub then_view: Box<HIRView>,
    pub else_view: Option<Box<HIRView>>,
}

#[derive(Debug, Clone)]
pub struct HIREach {
    pub iterable: HIRExpr,
    pub item_name: String,
    pub index_name: Option<String>,
    pub body: Box<HIRView>,
}

#[derive(Debug, Clone)]
pub struct HIRSwitch {
    pub expression: HIRExpr,
    pub cases: Vec<HIRSwitchCase>,
}

#[derive(Debug, Clone)]
pub struct HIRSwitchCase {
    pub pattern: HIRPattern,
    pub view: HIRView,
}

#[derive(Debug, Clone)]
pub enum HIRPattern {
    Literal(HIRExpr),
    EnumVariant(String),
    Some(String),
    Nil,
    Wildcard,
}

#[derive(Debug, Clone)]
pub struct HIRComponentRef {
    pub name: String,
    pub args: Vec<(String, HIRExpr)>,
    pub children: Vec<HIRView>,
}

#[derive(Debug, Clone)]
pub struct HIRAnimate {
    pub curve: AnimationCurve,
    pub duration: Option<f64>,
    pub child: Box<HIRView>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationCurve {
    Ease,
    Spring,
    Bounce,
    Linear,
}

/// An action definition (named state mutation).
#[derive(Debug, Clone)]
pub struct HIRAction {
    pub name: String,
    pub params: Vec<HIRParam>,
    pub body: Vec<HIRStmt>,
}

/// An action expression (used in buttons, inputs).
#[derive(Debug, Clone)]
pub enum HIRActionExpr {
    Call(String, Vec<HIRExpr>),
    Navigate(HIRNavigate),
    Sequence(Vec<HIRActionExpr>),
}

#[derive(Debug, Clone)]
pub enum HIRNavigate {
    To(HIRExpr),
    Back,
    Root,
    Replace(HIRExpr),
    Modal(HIRExpr),
    Dismiss,
}

/// A function definition (pure, no state mutation).
#[derive(Debug, Clone)]
pub struct HIRFunction {
    pub name: String,
    pub params: Vec<HIRParam>,
    pub return_type: AuraType,
    pub body: Vec<HIRStmt>,
}

/// A statement in an action or function body.
#[derive(Debug, Clone)]
pub enum HIRStmt {
    Assign(String, HIRExpr),
    Let(String, AuraType, HIRExpr),
    If(HIRExpr, Vec<HIRStmt>, Option<Vec<HIRStmt>>),
    When(HIRExpr, Vec<(HIRPattern, HIRStmtOrExpr)>),
    Navigate(HIRNavigate),
    Emit(String, Vec<HIRExpr>),
    Return(Option<HIRExpr>),
    Expr(HIRExpr),
}

#[derive(Debug, Clone)]
pub enum HIRStmtOrExpr {
    Stmts(Vec<HIRStmt>),
    Expr(HIRExpr),
}

/// A typed expression in HIR.
#[derive(Debug, Clone)]
pub enum HIRExpr {
    IntLit(i64),
    FloatLit(f64),
    StringLit(String),
    PercentLit(f64),
    BoolLit(bool),
    Nil,
    Var(String, AuraType),
    MemberAccess(Box<HIRExpr>, String, AuraType),
    Call(Box<HIRExpr>, Vec<HIRExpr>, AuraType),
    NamedCall(Box<HIRExpr>, Vec<(String, HIRExpr)>, AuraType),
    Index(Box<HIRExpr>, Box<HIRExpr>, AuraType),
    BinOp(Box<HIRExpr>, crate::ast::BinOp, Box<HIRExpr>, AuraType),
    UnaryOp(crate::ast::UnaryOp, Box<HIRExpr>, AuraType),
    Lambda(Vec<HIRParam>, Box<HIRExpr>, AuraType),
    Constructor(String, Vec<(String, HIRExpr)>, AuraType),
    Pipe(Box<HIRExpr>, Box<HIRExpr>, AuraType),
    Conditional(Box<HIRExpr>, Box<HIRExpr>, Box<HIRExpr>, AuraType),
    NilCoalesce(Box<HIRExpr>, Box<HIRExpr>, AuraType),
}

/// Enumerate all HIR node kinds (for BackendTrait::supported_hir_nodes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HIRNodeKind {
    Column,
    Row,
    Stack,
    Grid,
    Scroll,
    Wrap,
    Text,
    Heading,
    Image,
    Icon,
    Badge,
    Progress,
    Avatar,
    TextField,
    TextArea,
    Checkbox,
    Toggle,
    Slider,
    Picker,
    DatePicker,
    Segmented,
    Button,
    Conditional,
    Each,
    Switch,
    ComponentRef,
    Spacer,
    Divider,
    Slot,
    Animate,
}
