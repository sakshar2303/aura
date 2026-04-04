//! Token definitions for the Aura lexer.

use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \r]+")]
pub enum Token {
    // === Reserved Words ===
    #[token("app")]
    App,
    #[token("screen")]
    Screen,
    #[token("view")]
    View,
    #[token("model")]
    Model,
    #[token("state")]
    State,
    #[token("action")]
    Action,
    #[token("each")]
    Each,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("when")]
    When,
    #[token("is")]
    Is,
    #[token("import")]
    Import,
    #[token("from")]
    From,
    #[token("as")]
    As,
    #[token("theme")]
    Theme,
    #[token("style")]
    Style,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("nil")]
    Nil,
    #[token("and")]
    And,
    #[token("or")]
    Or,
    #[token("not")]
    Not,
    #[token("enum")]
    Enum,
    #[token("fn")]
    Fn,
    #[token("return")]
    Return,
    #[token("let")]
    Let,
    #[token("const")]
    Const,
    #[token("list")]
    List,
    #[token("map")]
    Map,
    #[token("set")]
    Set,
    #[token("optional")]
    Optional,
    #[token("component")]
    Component,
    #[token("navigate")]
    Navigate,
    #[token("back")]
    Back,
    #[token("emit")]
    Emit,
    #[token("on")]
    On,
    #[token("animate")]
    Animate,
    #[token("with")]
    With,
    #[token("where")]
    Where,
    #[token("in")]
    In,
    #[token("then")]
    Then,
    #[token("some")]
    Some_,
    #[token("slot")]
    Slot,

    // === Layout Keywords ===
    #[token("column")]
    Column,
    #[token("row")]
    Row,
    #[token("stack")]
    Stack,
    #[token("grid")]
    Grid,
    #[token("scroll")]
    Scroll,
    #[token("wrap")]
    Wrap,

    // === Widget Keywords ===
    #[token("text")]
    Text,
    #[token("heading")]
    Heading,
    #[token("image")]
    Image,
    #[token("icon")]
    Icon,
    #[token("badge")]
    Badge,
    #[token("divider")]
    Divider,
    #[token("spacer")]
    Spacer,
    #[token("progress")]
    Progress,
    #[token("avatar")]
    Avatar,
    #[token("button")]
    Button,

    // === Input Keywords ===
    #[token("textfield")]
    TextField,
    #[token("textarea")]
    TextArea,
    #[token("checkbox")]
    Checkbox,
    #[token("toggle")]
    Toggle,
    #[token("slider")]
    Slider,
    #[token("picker")]
    Picker,
    #[token("datepicker")]
    DatePicker,
    #[token("segmented")]
    Segmented,
    #[token("stepper")]
    Stepper,

    // === Type Keywords (Security) ===
    #[token("secret")]
    Secret,
    #[token("sanitized")]
    Sanitized,
    #[token("email")]
    Email,
    #[token("url")]
    Url,
    #[token("token")]
    TokenType,

    // === Type Keywords (Primitive) ===
    #[token("int")]
    Int,
    #[token("float")]
    Float,
    #[token("bool")]
    Bool,
    #[token("timestamp")]
    Timestamp,
    #[token("duration")]
    Duration,
    #[token("percent")]
    Percent,

    // === Operators ===
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Modulo,
    #[token("==")]
    EqEq,
    #[token("!=")]
    NotEq,
    #[token("<")]
    Lt,
    #[token(">")]
    Gt,
    #[token("<=")]
    LtEq,
    #[token(">=")]
    GtEq,
    #[token("=")]
    Eq,
    #[token("->")]
    Arrow,
    #[token("=>")]
    FatArrow,
    #[token("|>")]
    Pipe,
    #[token("::")]
    ColonColon,
    #[token("..")]
    DotDot,
    #[token("...")]
    Spread,
    #[token("??")]
    NilCoalesce,

    // === Punctuation ===
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,

    // === Literals ===
    #[regex(r"[0-9][0-9_]*", |lex| lex.slice().replace('_', "").parse::<i64>().ok())]
    Integer(i64),

    #[regex(r"[0-9][0-9_]*\.[0-9][0-9_]*", |lex| lex.slice().replace('_', "").parse::<f64>().ok())]
    FloatLit(f64),

    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let s = lex.slice();
        Some(s[1..s.len()-1].to_string())
    })]
    StringLit(String),

    // === Identifiers ===
    #[regex(r"[a-z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),

    #[regex(r"[A-Z][a-zA-Z0-9]*", |lex| lex.slice().to_string())]
    TypeIdent(String),

    // === Whitespace & Structure ===
    #[token("\n")]
    Newline,

    // === Comments (skipped) ===
    #[regex(r"//[^\n]*", logos::skip)]
    SingleLineComment,

    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip)]
    MultiLineComment,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::App => write!(f, "app"),
            Token::Screen => write!(f, "screen"),
            Token::View => write!(f, "view"),
            Token::Model => write!(f, "model"),
            Token::Ident(s) => write!(f, "{}", s),
            Token::TypeIdent(s) => write!(f, "{}", s),
            Token::StringLit(s) => write!(f, "\"{}\"", s),
            Token::Integer(n) => write!(f, "{}", n),
            Token::FloatLit(n) => write!(f, "{}", n),
            Token::Arrow => write!(f, "->"),
            Token::FatArrow => write!(f, "=>"),
            Token::Dot => write!(f, "."),
            Token::Colon => write!(f, ":"),
            Token::Eq => write!(f, "="),
            Token::Newline => write!(f, "\\n"),
            _ => write!(f, "{:?}", self),
        }
    }
}
