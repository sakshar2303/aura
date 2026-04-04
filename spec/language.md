# The Aura Language Specification

**Version:** 0.1.0-draft
**Status:** Draft
**Date:** 2026-04-04

> *"Design that radiates."*

Aura is a programming language designed for AI coding agents to build cross-platform applications with built-in design intelligence. It compiles to platform-native code: SwiftUI (iOS/macOS), Jetpack Compose (Android), HTML/CSS/JS (Web), WinUI (Windows), and Terminal UI.

---

## Table of Contents

1. [Philosophy & Design Goals](#1-philosophy--design-goals)
2. [Lexical Structure](#2-lexical-structure)
3. [Syntax & Grammar](#3-syntax--grammar)
4. [Type System](#4-type-system)
5. [Security Types](#5-security-types)
6. [Design Token System](#6-design-token-system)
7. [Component Model](#7-component-model)
8. [State Management](#8-state-management)
9. [Navigation & Routing](#9-navigation--routing)
10. [Platform Adaptation](#10-platform-adaptation)
11. [Intermediate Representations](#11-intermediate-representations)
12. [Error System](#12-error-system)
13. [Module System & Packages](#13-module-system--packages)
14. [Agent API Protocol](#14-agent-api-protocol)
15. [Standard Library](#15-standard-library)
16. [Backend Trait Interface](#16-backend-trait-interface)
17. [Appendix A: Complete EBNF Grammar](#appendix-a-complete-ebnf-grammar)
18. [Appendix B: Design Token Reference](#appendix-b-design-token-reference)
19. [Appendix C: Error Code Reference](#appendix-c-error-code-reference)

---

## 1. Philosophy & Design Goals

### 1.1 Core Principles

1. **AI-first, human-readable.** Every syntax decision optimizes for LLM token efficiency while remaining instantly readable by humans. A developer who has never seen Aura should understand an Aura program on first read.

2. **Zero ambiguity.** Every valid Aura program has exactly one parse tree. No automatic semicolon insertion, no operator precedence surprises, no implicit type coercion. AI agents never generate syntactically-valid-but-semantically-wrong code.

3. **Design is computation.** Layout, color, typography, and motion are not properties you set — they are first-class language constructs computed from constraints and themes. You express design intent; the compiler resolves it per platform.

4. **Secure by default.** The type system prevents common security vulnerabilities at compile time. Passwords cannot be stored as plaintext. User input cannot be rendered unsanitized. SQL queries cannot be built from raw strings.

5. **One source, native everywhere.** A single `.aura` file compiles to truly native code on every platform — not through a shared runtime, but through platform-native code generation.

### 1.2 Non-Goals

- Aura is NOT a general-purpose systems programming language. It does not expose raw memory, pointers, or manual memory management.
- Aura is NOT a server-side language (in v0.x). Backend/API generation is a future goal.
- Aura does NOT aim for Turing-completeness at the type level. The type system is practical, not academic.

### 1.3 Token Efficiency Targets

Compared to equivalent programs in TypeScript + React Native:
- **60-80% fewer tokens** for UI-heavy programs
- **40-60% fewer tokens** for data-model-heavy programs
- **Zero boilerplate** — no imports for built-in types, no file headers, no build configuration per file

---

## 2. Lexical Structure

### 2.1 Source Encoding

All Aura source files MUST be encoded in UTF-8. The compiler rejects files with invalid UTF-8 sequences with error `E0001`.

File extension: `.aura`

Maximum source file size: 10 MB. Files exceeding this limit produce error `E0002`.

### 2.2 Whitespace and Indentation

Aura uses **indentation-significant syntax**. Indentation defines block structure.

```
INDENT_UNIT = 2 spaces
```

Rules:
- The indent unit is exactly **2 spaces**.
- Tabs are **not permitted** and produce error `E0010`. The compiler suggests conversion.
- Blank lines are ignored (do not affect indentation).
- Trailing whitespace is ignored.
- A block is opened by increasing indentation by exactly one `INDENT_UNIT` (2 spaces).
- A block is closed by returning to the previous indentation level.
- Inconsistent indentation (e.g., 3 spaces) produces error `E0011` with an auto-fix suggestion.

```
// Valid: 2-space indentation
app MyApp
  screen Main
    view
      text "Hello"

// Invalid: 4-space indentation → E0011
app MyApp
    screen Main        // Error: expected 2 spaces, found 4
```

### 2.3 Comments

```
// Single-line comment (to end of line)

/* Multi-line comment
   can span multiple lines */

/// Documentation comment (attached to next declaration)
/// Supports markdown formatting.
```

Comments are stripped during lexing and do not appear in the AST. This is a security feature: AI agents operate on the AST, not on source text, preventing prompt injection via comments.

### 2.4 Identifiers

```ebnf
identifier     = letter , { letter | digit | "_" } ;
letter         = "a"-"z" | "A"-"Z" | unicode_letter ;
digit          = "0"-"9" ;
```

Rules:
- Identifiers are case-sensitive.
- Identifiers starting with uppercase are **type names** (e.g., `Todo`, `UserProfile`).
- Identifiers starting with lowercase are **value names** (e.g., `todos`, `userName`).
- The underscore `_` is a valid identifier meaning "discard" (unused binding).
- Reserved words cannot be used as identifiers.
- Unicode identifiers are normalized to NFC form. Confusable characters produce warning `W0001`.

### 2.5 Reserved Words

```
app       screen    view      model     state     
action    each      if        else      when
import    from      as        theme     style
true      false     nil       and       or
not       enum      fn        return    let
const     list      map       set       optional
api       route     get       post      put
delete    navigate  back      emit      on
animate   with      where     is        in
```

### 2.6 Literals

#### Numeric Literals
```
integer_literal  = digit , { digit | "_" } ;
float_literal    = digit , { digit | "_" } , "." , digit , { digit | "_" } ;
percent_literal  = ( integer_literal | float_literal ) , "%" ;

// Examples:
42
1_000_000
3.14
99.5%
```

#### String Literals
```
string_literal   = '"' , { string_char | escape_seq | interpolation } , '"' ;
string_char      = any UTF-8 character except '"', '\', '{' ;
escape_seq       = '\' , ( '"' | '\' | 'n' | 't' | 'r' | '{' ) ;
interpolation    = '{' , expression , '}' ;

// Examples:
"Hello, world"
"Hello, {user.name}"
"Line 1\nLine 2"
"Price: \{not interpolation\}"
```

#### Boolean Literals
```
true
false
```

#### Nil Literal
```
nil
```

`nil` is only valid for `optional` types. Using `nil` where a non-optional type is expected produces error `E0100`.

### 2.7 Operators

```
// Arithmetic
+   -   *   /   %

// Comparison
==  !=  <   >   <=  >=

// Logical
and   or   not

// Assignment
=

// Access
.       // member access
::      // namespace access

// Action
->      // action trigger (e.g., button "Go" -> doSomething())
=>      // lambda / mapping
|>      // pipe operator
```

### 2.8 Punctuation

```
(   )       // grouping, function calls
[   ]       // list/index access  
{   }       // string interpolation
:           // type annotation, key-value
,           // parameter separator (optional in many contexts)
..          // range operator
...         // spread operator
```

### 2.9 Design Token Syntax

Design tokens use a **dot-prefix notation** and are lexed as a distinct token type:

```
design_token = "." , identifier , { "." , identifier } ;

// Examples:
.xs  .sm  .md  .lg  .xl  .2xl            // spacing/sizing
.primary  .secondary  .accent  .danger    // color
.bold  .italic  .mono                     // typography
.ease  .spring  .bounce                   // motion
```

Design tokens are context-sensitive: `.md` in a spacing context means "medium spacing" while `.md` in a typography context means "medium font size". The semantic analyzer resolves this based on the parent element.

---

## 3. Syntax & Grammar

### 3.1 Program Structure

An Aura program is a single `.aura` file or a collection of `.aura` files in a project directory.

```ebnf
program        = { import_decl } , app_decl ;
app_decl       = "app" , identifier , NEWLINE , INDENT , app_body , DEDENT ;
app_body       = { app_member } ;
app_member     = theme_decl | model_decl | screen_decl | component_decl 
               | style_decl | api_decl | const_decl | fn_decl ;
```

Minimal valid program:
```
app Hello
  screen Main
    view
      text "Hello, Aura"
```

### 3.2 Import Declarations

```ebnf
import_decl    = "import" , import_spec , "from" , string_literal , NEWLINE ;
import_spec    = identifier | "{" , identifier , { "," , identifier } , "}" ;
```

```
import Button from "@aura/ui"
import { Chart, Graph } from "@aura/dataviz"
import MyTheme from "./themes/dark"
```

### 3.3 Model Declarations

Models define data structures with typed fields. They are immutable value types by default.

```ebnf
model_decl     = "model" , type_name , NEWLINE , INDENT , { field_decl } , DEDENT ;
field_decl     = identifier , ":" , type_expr , [ "=" , expression ] , NEWLINE ;
```

```
model Todo
  title: text
  done: bool = false
  created: timestamp = now()
  priority: enum[low, medium, high] = low

model User
  name: text
  email: email
  password: secret
  avatar: optional[url]
  todos: list[Todo] = []
```

### 3.4 Screen Declarations

Screens are top-level UI containers — analogous to Activities (Android), ViewControllers (iOS), or Pages (Web).

```ebnf
screen_decl    = "screen" , type_name , [ "(" , param_list , ")" ] , NEWLINE , 
                 INDENT , { screen_member } , DEDENT ;
screen_member  = state_decl | view_decl | action_decl | fn_decl | on_decl ;
```

```
screen TaskList
  state todos: list[Todo] = []
  state filter: enum[all, active, done] = all

  view
    column gap.md padding.lg
      heading "Tasks" size.xl
      segmented filter options: [all, active, done]
      each filteredTodos as todo
        TaskRow(todo)

  fn filteredTodos -> list[Todo]
    when filter
      is all -> todos
      is active -> todos.where(t => not t.done)
      is done -> todos.where(t => t.done)

  action addTodo(title: text)
    todos = todos.append(Todo(title: title))

  action toggleTodo(todo: Todo)
    todos = todos.map(t =>
      if t == todo then t.with(done: not t.done) else t)
```

### 3.5 View Declarations

The `view` block defines the UI tree. Child elements are nested via indentation.

```ebnf
view_decl      = "view" , NEWLINE , INDENT , view_body , DEDENT ;
view_body      = { view_element } ;
view_element   = layout_element | widget_element | control_flow | component_ref ;
```

#### Layout Elements
```ebnf
layout_element = layout_kind , { design_token | prop_assign } , NEWLINE , 
                 [ INDENT , { view_element } , DEDENT ] ;
layout_kind    = "column" | "row" | "stack" | "grid" | "scroll" | "wrap" ;
```

```
column gap.md padding.lg
  row gap.sm align.center
    text "Hello" .bold
    spacer
    badge count .accent
  divider
  scroll
    each items as item
      ItemCard(item)
```

#### Widget Elements
```ebnf
widget_element = widget_kind , { expression | design_token | prop_assign } , 
                 [ "->" , action_expr ] , NEWLINE ;
widget_kind    = "text" | "heading" | "image" | "icon" | "badge" | "divider" 
               | "spacer" | "progress" | "avatar" ;
```

```
heading "My App" size.2xl .bold
text user.name .secondary
text "Created {todo.created}" size.sm .muted
image user.avatar size.lg .rounded
icon "star" .accent
progress task.completion
divider .subtle
spacer
```

#### Input Elements
```ebnf
input_element  = input_kind , identifier , { design_token | prop_assign } , 
                 [ "->" , action_expr ] , NEWLINE ;
input_kind     = "textfield" | "textarea" | "checkbox" | "toggle" | "slider" 
               | "picker" | "datepicker" | "segmented" | "stepper" ;
```

```
textfield query placeholder: "Search..." .lg
  -> search(query)
checkbox todo.done
toggle darkMode label: "Dark Mode"
slider volume min: 0 max: 100 step: 1
picker category options: categories
datepicker dueDate label: "Due"
```

#### Button Elements
```ebnf
button_element = "button" , [ "." , button_style ] , expression , 
                 { design_token | prop_assign } , "->" , action_expr , NEWLINE ;
button_style   = "icon" | "outline" | "ghost" | "link" ;
```

```
button "Save" .accent -> saveTodo()
button "Delete" .danger -> deleteTodo(todo)
button.icon "plus" .accent .lg -> addItem()
button.outline "Cancel" -> navigate.back
button.ghost "Skip" .muted -> skip()
```

### 3.6 Control Flow in Views

```ebnf
if_view        = "if" , expression , NEWLINE , INDENT , { view_element } , DEDENT ,
                 [ "else" , NEWLINE , INDENT , { view_element } , DEDENT ] ;

each_view      = "each" , expression , "as" , identifier , [ "," , identifier ] , 
                 NEWLINE , INDENT , { view_element } , DEDENT ;

when_view      = "when" , expression , NEWLINE , INDENT , { when_branch } , DEDENT ;
when_branch    = "is" , pattern , NEWLINE , INDENT , { view_element } , DEDENT ;
```

```
// Conditional
if todos.isEmpty
  EmptyState(message: "No tasks yet")
else
  each todos as todo
    TodoRow(todo)

// Pattern matching
when connectionState
  is .connected
    text "Online" .success
  is .disconnected
    text "Offline" .danger
  is .connecting
    progress .indeterminate
```

### 3.7 Expression Syntax

```ebnf
expression     = literal | identifier | member_access | function_call 
               | binary_expr | unary_expr | lambda | constructor 
               | pipe_expr | conditional_expr ;

member_access  = expression , "." , identifier ;
function_call  = expression , "(" , [ arg_list ] , ")" ;
binary_expr    = expression , binary_op , expression ;
unary_expr     = unary_op , expression ;
lambda         = [ param_list ] , "=>" , expression ;
constructor    = type_name , "(" , [ named_arg_list ] , ")" ;
pipe_expr      = expression , "|>" , expression ;
conditional_expr = "if" , expression , "then" , expression , "else" , expression ;

named_arg_list = named_arg , { "," , named_arg } ;
named_arg      = identifier , ":" , expression ;
```

```
// Member access
user.name
todo.priority.label

// Function call
todos.count()
todos.where(t => t.done)

// Constructor
Todo(title: "Buy milk", priority: high)

// Pipe
todos |> filter(t => not t.done) |> sortBy(.created) |> take(10)

// Conditional expression
if user.isAdmin then "Admin Panel" else "Dashboard"
```

### 3.8 Action Declarations

Actions are named mutations that modify state. They are the only way to change `state` variables.

```ebnf
action_decl    = "action" , identifier , [ "(" , param_list , ")" ] , NEWLINE ,
                 INDENT , { statement } , DEDENT ;

statement      = assignment | action_call | if_stmt | when_stmt | emit_stmt 
               | navigate_stmt | let_stmt ;
assignment     = identifier , "=" , expression , NEWLINE ;
```

```
action addTodo(title: text)
  let todo = Todo(title: title)
  todos = todos.append(todo)
  input = ""

action deleteTodo(todo: Todo)
  todos = todos.remove(todo)

action reorder(from: int, to: int)
  let item = todos[from]
  todos = todos.remove(at: from).insert(item, at: to)
```

### 3.9 Function Declarations

Pure functions (no side effects, no state mutation).

```ebnf
fn_decl        = "fn" , identifier , [ "(" , param_list , ")" ] , 
                 [ "->" , type_expr ] , NEWLINE ,
                 INDENT , { statement | expression } , DEDENT ;
```

```
fn filteredTodos -> list[Todo]
  when filter
    is all -> todos
    is active -> todos.where(t => not t.done)
    is done -> todos.where(t => t.done)

fn formatDate(d: timestamp) -> text
  d.format("MMM d, yyyy")

fn greeting(name: text) -> text
  "Hello, {name}!"
```

### 3.10 Component Declarations

Reusable UI components with typed props.

```ebnf
component_decl = "component" , type_name , [ "(" , param_list , ")" ] , NEWLINE ,
                 INDENT , { component_member } , DEDENT ;
component_member = state_decl | view_decl | action_decl | fn_decl | style_decl ;
```

```
component TodoRow(todo: Todo, onToggle: action, onDelete: action)
  view
    row gap.md align.center padding.sm
      checkbox todo.done -> onToggle()
      column
        text todo.title strike: todo.done
        text todo.created |> formatDate size.xs .muted
      spacer
      button.icon "trash" .danger .sm -> onDelete()

component EmptyState(message: text, icon: text = "inbox")
  view
    column align.center padding.2xl gap.md
      icon icon size.2xl .muted
      text message .muted .center
```

### 3.11 Constant Declarations

```ebnf
const_decl     = "const" , identifier , [ ":" , type_expr ] , "=" , expression , NEWLINE ;
```

```
const MAX_TODOS = 100
const APP_NAME: text = "TaskMaster"
const PRIORITIES: list[text] = ["Low", "Medium", "High"]
```

---

## 4. Type System

### 4.1 Overview

Aura uses a **structural type system with inference**. Types are inferred where possible and annotated where required. The type checker runs as a single pass after parsing, producing typed HIR.

### 4.2 Primitive Types

| Type | Description | Default | Example |
|------|-------------|---------|---------|
| `text` | Unicode string (UTF-8) | `""` | `"Hello"` |
| `int` | 64-bit signed integer | `0` | `42` |
| `float` | 64-bit IEEE 754 | `0.0` | `3.14` |
| `bool` | Boolean | `false` | `true` |
| `timestamp` | UTC timestamp (millisecond precision) | — | `now()` |
| `duration` | Time duration | — | `5.minutes` |
| `percent` | 0.0-1.0 (displayed as 0%-100%) | `0%` | `75%` |

### 4.3 Collection Types

```
list[T]          // Ordered collection
map[K, V]        // Key-value mapping (K must be hashable)
set[T]           // Unique unordered collection (T must be hashable)
```

All collections are **immutable**. Mutation methods (`.append()`, `.remove()`, etc.) return new collections.

```
list[Todo]                    // List of Todos
map[text, int]                // String to integer mapping
set[text]                     // Set of unique strings
list[list[int]]               // Nested lists
```

### 4.4 Optional Types

```
optional[T]      // Value of type T or nil
```

Accessing an optional without checking produces error `E0110`. The compiler enforces nil-safety:

```
model User
  name: text
  bio: optional[text]

// Must check before use:
if user.bio is some(bio)
  text bio
else
  text "No bio" .muted

// Or use default:
text user.bio ?? "No bio"
```

### 4.5 Enum Types

```ebnf
enum_type      = "enum" , "[" , identifier , { "," , identifier } , "]" ;
```

Enums are lightweight union types:

```
// Inline enum (for fields)
priority: enum[low, medium, high]

// Named enum (top-level)
model ConnectionState
  enum[connected, disconnected, connecting, error(text)]
```

Enums with associated data:

```
model Result
  enum[
    success(data: any),
    error(message: text, code: int),
    loading,
    idle
  ]
```

### 4.6 Function Types

```
action                          // Action with no parameters
action(text)                    // Action taking text
fn(int, int) -> int             // Function from two ints to int
fn(Todo) -> bool                // Predicate on Todo
```

### 4.7 Type Inference Rules

1. **Variable initialization:** Type inferred from right-hand side.
   ```
   state count = 0              // inferred: int
   state name = "Aura"          // inferred: text
   state items = list[Todo]()   // explicit: list[Todo]
   ```

2. **Function return:** Type inferred from return expression if not annotated.
   ```
   fn double(x: int) -> int     // annotated
     x * 2
   
   fn double(x: int)            // inferred: -> int
     x * 2
   ```

3. **Lambda parameters:** Inferred from context.
   ```
   todos.where(t => not t.done)  // t inferred as Todo
   ```

4. **Constructor fields:** Must match model definition.
   ```
   Todo(title: "Buy milk")       // done inferred as false (default)
   ```

### 4.8 Type Compatibility

- No implicit type coercion. `int` does not auto-convert to `float`.
- Explicit conversion via methods: `42.toFloat()`, `"42".toInt()`.
- `text` interpolation auto-converts any type: `"Count: {count}"` works for any type with `.toText()`.

### 4.9 Immutability

All values in Aura are **immutable by default**. There is no mutable reference type.

- Model instances are immutable. Use `.with()` to create modified copies:
  ```
  let updated = todo.with(done: true)
  ```
- Collections are immutable. Mutation methods return new collections:
  ```
  let newList = todos.append(todo)   // todos is unchanged
  ```
- Only `state` variables can be reassigned (inside `action` blocks only):
  ```
  action toggle(todo: Todo)
    todos = todos.map(t =>
      if t == todo then t.with(done: not t.done) else t)
  ```

---

## 5. Security Types

Security types are **built-in primitive types** that enforce security constraints at compile time. They are not library types — they are part of the language grammar.

### 5.1 `secret`

A type for sensitive credentials (passwords, API keys, tokens).

```
model User
  password: secret
```

**Compile-time enforcement:**
- `secret` values CANNOT appear in:
  - API responses (error `E0200`)
  - Log output (error `E0201`)
  - String interpolation (error `E0202`)
  - Comparisons with `==` (error `E0203`; use `.verify()` instead)
  - Any serialization (error `E0204`)
- `secret` values CAN:
  - Be assigned from `text` input (auto-hashed on assignment)
  - Be verified with `.verify(input: text) -> bool`
  - Be checked for presence with `.isSet -> bool`

```
// Correct usage:
action login(email: email, pass: text)
  let user = findUser(email)
  if user.password.verify(pass)
    navigate(Dashboard)
  else
    showError("Invalid credentials")

// Compile error:
text "Password: {user.password}"     // E0202: secret in interpolation
api.respond(user)                    // E0200: model with secret in API response
```

### 5.2 `sanitized`

A type for user-provided text that has been validated and escaped.

```
model Comment
  body: sanitized
```

**Compile-time enforcement:**
- Raw `text` from user input CANNOT be assigned to view elements that render HTML/rich content without explicit sanitization (error `E0210`).
- `sanitized` is created via `text.sanitize()` or by receiving input through Aura's built-in `textfield`/`textarea` (which auto-sanitize).
- `sanitized` has a configurable maximum length (default: 10,000 characters).
- `sanitized` values have HTML entities escaped, script tags stripped, and Unicode normalized.

```
// Auto-sanitized (from Aura input):
textfield comment placeholder: "Write a comment..."
  -> addComment(comment)     // comment is already sanitized

// Manual sanitization:
action importComment(raw: text)
  let clean = raw.sanitize(maxLength: 5000)
  comments = comments.append(Comment(body: clean))
```

### 5.3 `email`

A type for email addresses with compile-time format validation.

```
model User
  email: email
```

**Compile-time enforcement:**
- String literals assigned to `email` are validated at compile time (error `E0220` if invalid format).
- Runtime values are validated on construction and produce a runtime error if invalid.
- `email` cannot be constructed from unsanitized input without validation.

```
const ADMIN: email = "admin@example.com"       // Validated at compile time
const BAD: email = "not-an-email"              // E0220: invalid email format

// Runtime validation:
action updateEmail(input: text)
  when email.parse(input)
    is some(addr) -> user = user.with(email: addr)
    is nil -> showError("Invalid email address")
```

### 5.4 `url`

A type for validated URLs.

```
model Link
  href: url
  label: text
```

**Compile-time enforcement:**
- String literals validated at compile time (error `E0230`).
- URLs are parsed and normalized (scheme required).
- `javascript:` and `data:` schemes are rejected by default (configurable).

### 5.5 `token`

A type for authentication/API tokens with expiration semantics.

```
model Session
  authToken: token
```

**Compile-time enforcement:**
- `token` values have an associated expiration timestamp.
- Using an expired token produces a runtime error (not silent failure).
- `token` values cannot be logged or serialized (same restrictions as `secret`).
- `token.isExpired -> bool` and `token.expiresAt -> timestamp` are the only read accessors.

### 5.6 Security Type Summary

```
  TYPE       │ AUTO-HASH │ NO-LOG │ NO-SERIALIZE │ VALIDATED │ EXPIRES
  ───────────┼───────────┼────────┼──────────────┼───────────┼────────
  secret     │ Yes       │ Yes    │ Yes          │ No        │ No
  sanitized  │ No        │ No     │ No           │ Yes (XSS) │ No
  email      │ No        │ No     │ No           │ Yes (fmt) │ No
  url        │ No        │ No     │ No           │ Yes (fmt) │ No
  token      │ No        │ Yes    │ Yes          │ No        │ Yes
```

---

## 6. Design Token System

Design tokens are first-class language constructs — not a library, not CSS classes, but part of Aura's grammar. They define the visual vocabulary of an application.

### 6.1 Token Categories

#### Spacing Tokens

Controls padding, margins, and gaps between elements.

```
  TOKEN │ MULTIPLIER │ iOS (pt) │ Android (dp) │ Web (px) │ Web (rem)
  ──────┼────────────┼──────────┼──────────────┼──────────┼──────────
  .xs   │ 0.25x      │ 2        │ 2            │ 2        │ 0.125
  .sm   │ 0.5x       │ 4        │ 4            │ 4        │ 0.25
  .md   │ 1x         │ 8        │ 8            │ 8        │ 0.5
  .lg   │ 2x         │ 16       │ 16           │ 16       │ 1.0
  .xl   │ 3x         │ 24       │ 24           │ 24       │ 1.5
  .2xl  │ 4x         │ 32       │ 32           │ 32       │ 2.0
  .3xl  │ 6x         │ 48       │ 48           │ 48       │ 3.0
  .4xl  │ 8x         │ 64       │ 64           │ 64       │ 4.0
```

The base multiplier (1x = 8px) is configurable in the theme definition. All tokens scale proportionally.

```
column gap.md padding.lg           // gap: 8px, padding: 16px
row padding.horizontal.lg          // horizontal padding only
text "Hello" margin.bottom.sm      // bottom margin: 4px
```

#### Typography Tokens

Controls font size, weight, and style.

```
  SIZE TOKEN │ iOS (pt) │ Android (sp) │ Web (rem)
  ───────────┼──────────┼──────────────┼──────────
  .xs        │ 11       │ 12           │ 0.75
  .sm        │ 13       │ 14           │ 0.875
  .md        │ 15       │ 16           │ 1.0
  .lg        │ 17       │ 18           │ 1.125
  .xl        │ 20       │ 20           │ 1.25
  .2xl       │ 24       │ 24           │ 1.5
  .3xl       │ 34       │ 34           │ 2.125
  .display   │ 48       │ 48           │ 3.0
```

```
  WEIGHT TOKEN │ Value
  ─────────────┼──────
  .thin        │ 100
  .light       │ 300
  .regular     │ 400
  .medium      │ 500
  .semibold    │ 600
  .bold        │ 700
  .heavy       │ 800
  .black       │ 900
```

```
  STYLE TOKEN  │ Effect
  ─────────────┼──────────────────
  .italic      │ Italic style
  .mono        │ Monospace font
  .underline   │ Underline decoration
  .strike      │ Strikethrough
  .center      │ Center-aligned text
  .leading     │ Left-aligned (default)
  .trailing    │ Right-aligned
  .uppercase   │ Transform to UPPERCASE
  .lowercase   │ Transform to lowercase
  .capitalize  │ Capitalize First Letters
```

```
heading "Title" size.2xl .bold
text "Subtitle" size.lg .medium .secondary
text "Caption" size.xs .mono .muted
text "Important" .bold .uppercase .accent
```

#### Color Tokens

Semantic colors that adapt to light/dark mode and platform conventions.

```
  TOKEN       │ Purpose                    │ Light Mode  │ Dark Mode
  ────────────┼──────────────────────────���─┼─────────────┼──────────
  .primary    │ Primary text               │ #000000     │ #FFFFFF
  .secondary  │ Secondary text             │ #666666     │ #AAAAAA
  .muted      │ Tertiary/disabled text     │ #999999     │ #666666
  .accent     │ Brand color / interactive  │ theme.accent│ theme.accent
  .danger     │ Destructive / error        │ #DC3545     │ #FF4757
  .warning    │ Warning / caution          │ #FFC107     │ #FFBE76
  .success    │ Success / positive         │ #28A745     │ #2ED573
  .info       │ Informational              │ #17A2B8     │ #1E90FF
  .surface    │ Card/container background  │ #FFFFFF     │ #1E1E1E
  .background │ Page background            │ #F5F5F5     │ #121212
  .divider    │ Lines and separators       │ #E0E0E0     │ #2C2C2C
```

```
text "Error!" .danger
button "Delete" .danger -> deleteItem()
row .surface padding.md radius.md
  text "Card content"
```

#### Shape Tokens

Controls border radius and shape.

```
  TOKEN      │ Value
  ───────────┼───────────────
  .sharp     │ 0px radius
  .subtle    │ 4px radius
  .rounded   │ 8px radius
  .smooth    │ 12px radius
  .pill      │ 9999px radius (fully rounded)
  .circle    │ 50% (circular)
```

```
image avatar .circle size.lg
button "Save" .pill .accent -> save()
row .surface .rounded padding.md
  text "Rounded card"
```

#### Motion Tokens

Controls animations and transitions.

```
  TOKEN         │ Description              │ Duration
  ──────────────┼─────────────────────────���┼─────────
  .instant      │ No animation             │ 0ms
  .fast         │ Quick micro-interaction  │ 150ms
  .normal       │ Standard transition      │ 300ms
  .slow         │ Deliberate animation     │ 500ms
  .ease         │ Ease in-out curve        │ —
  .spring       │ Spring physics           │ —
  .bounce       │ Bounce effect            │ —
```

```
animate .spring
  if expanded
    column gap.md
      text details

animate .fast .ease
  if visible
    row .surface padding.md
      text notification
```

### 6.2 Theme Definitions

Themes customize all design token values.

```
theme modern
  spacing.base: 8
  radius: smooth
  typography.family: "Inter"
  typography.scale: 1.25
  motion: spring(stiffness: 200, damping: 20)

  palette
    accent: "#6C5CE7"
    background.light: "#FAFAFA"
    background.dark: "#0D0D0D"
    surface.light: "#FFFFFF"
    surface.dark: "#1A1A1A"

  variants: [light, dark, high-contrast]
```

```
// Use in app:
app MyApp
  theme: modern.dark

  screen Main
    view
      text "Styled by theme" .accent
```

### 6.3 Responsive Design

Aura handles responsive layout through **constraint-based design**, not breakpoints.

```
// Automatic: grid adapts to screen width
grid min-width: 200 gap.md
  each items as item
    ItemCard(item)

// Explicit breakpoints (escape hatch):
row
  when screen.width
    is .compact     // < 600px
      column gap.sm
        SideBar()
        Content()
    is .regular     // 600-1024px
      row gap.md
        SideBar() width: 250
        Content()
    is .expanded    // > 1024px
      row gap.lg
        SideBar() width: 300
        Content()
        DetailPanel()
```

### 6.4 Accessibility

Design tokens enforce accessibility by default:

- **Contrast checking:** The compiler warns (W0050) when text/background combinations fail WCAG AA contrast ratios (4.5:1 for body text, 3:1 for large text).
- **Touch targets:** Interactive elements have a minimum touch target of 44x44pt (iOS) / 48x48dp (Android). Smaller targets produce warning W0051.
- **Motion reduction:** `animate` blocks respect the platform's "reduce motion" setting automatically. When motion is reduced, animations resolve to their end state instantly.
- **Screen reader:** All interactive elements require a label. Unlabeled buttons produce error E0060.

---

## 7. Component Model

### 7.1 Props

Components receive data through typed props. Props are immutable.

```
component UserCard(
  user: User,
  showEmail: bool = false,
  onTap: optional[action] = nil,
  size: enum[compact, regular, large] = regular
)
  view
    row gap.md align.center padding.md .surface .rounded
      avatar user.avatar size: avatarSize
      column
        text user.name .bold
        if showEmail
          text user.email .sm .secondary
      if onTap is some(tap)
        spacer
        button.icon "chevron.right" .muted -> tap()

  fn avatarSize -> design.size
    when size
      is compact -> .sm
      is regular -> .md
      is large -> .lg
```

### 7.2 Component Composition

Components compose by nesting:

```
screen UserList
  state users: list[User] = []

  view
    scroll padding.md
      column gap.sm
        each users as user
          UserCard(user, showEmail: true, onTap: -> viewUser(user))

  action viewUser(user: User)
    navigate(UserDetail(user: user))
```

### 7.3 Slots (Content Projection)

Components can accept child content via `slot`:

```
component Card(title: text)
  view
    column .surface .rounded padding.lg gap.md
      heading title size.lg
      slot                          // Children rendered here

// Usage:
Card(title: "Settings")
  toggle darkMode label: "Dark Mode"
  toggle notifications label: "Notifications"
  divider
  button "Reset" .danger -> reset()
```

### 7.4 Style Declarations

Named style groupings for reuse:

```
style cardStyle
  .surface .rounded padding.lg shadow.sm

style dangerButton
  .danger .bold .pill

// Usage:
row cardStyle
  text "Content"
button "Delete" dangerButton -> delete()
```

---

## 8. State Management

### 8.1 State Declaration

State is declared with the `state` keyword and is scoped to the enclosing screen or component.

```
state count: int = 0
state user: optional[User] = nil
state todos: list[Todo] = []
state loading: bool = false
```

### 8.2 State Mutation Rules

1. State can ONLY be mutated inside `action` blocks.
2. State mutation is ALWAYS explicit assignment (`=`).
3. There are no mutable references — mutation creates new values.
4. State changes trigger automatic view re-rendering.

```
// Valid: mutation inside action
action increment
  count = count + 1

// Invalid: mutation outside action → E0300
fn compute
  count = count + 1    // E0300: state mutation outside action block

// Invalid: mutation in view → E0301
view
  text "{count}"
  count = count + 1    // E0301: state mutation in view block
```

### 8.3 Derived State

Computed values that auto-update when dependencies change:

```
screen Dashboard
  state todos: list[Todo] = []

  // Derived (recomputed when todos changes):
  fn activeTodos -> list[Todo]
    todos.where(t => not t.done)

  fn completionRate -> percent
    if todos.isEmpty then 0%
    else (todos.count(t => t.done).toFloat() / todos.count().toFloat()) * 100%

  view
    text "Active: {activeTodos.count()}"
    progress completionRate .accent
```

### 8.4 Shared State

For state shared across screens, use `app`-level state:

```
app MyApp
  state currentUser: optional[User] = nil
  state theme: enum[light, dark] = light

  screen Login
    action login(email: email, pass: text)
      let user = authenticate(email, pass)
      currentUser = user             // Modifies app-level state
      navigate(Dashboard)

  screen Dashboard
    view
      if currentUser is some(user)
        text "Welcome, {user.name}"
```

---

## 9. Navigation & Routing

### 9.1 Navigation Model

Navigation in Aura is **declarative and type-safe**. Every navigation destination is a screen with typed parameters.

```
// Navigate to a screen:
navigate(UserDetail(user: selectedUser))

// Navigate back:
navigate.back

// Navigate to root:
navigate.root

// Replace current screen:
navigate.replace(Login)

// Present as modal:
navigate.modal(Settings)

// Dismiss modal:
navigate.dismiss
```

### 9.2 Navigation Stack

```
app MyApp
  navigation: stack            // Stack-based navigation (default)

  screen Home                  // Root screen
    view
      button "View Profile" -> navigate(Profile(user: currentUser))

  screen Profile(user: User)
    view
      heading user.name
      button "Back" -> navigate.back
```

### 9.3 Tab Navigation

```
app MyApp
  navigation: tabs

  screen Home tab: "house" label: "Home"
    view
      text "Home content"

  screen Search tab: "magnifyingglass" label: "Search"
    view
      text "Search content"

  screen Profile tab: "person" label: "Profile"
    view
      text "Profile content"
```

### 9.4 Deep Linking

```
app MyApp
  route "/" -> Home
  route "/user/{id}" -> UserDetail(id: id)
  route "/settings" -> Settings
```

---

## 10. Platform Adaptation

### 10.1 Conditional Compilation

Use `when platform` to include platform-specific code:

```
view
  column padding.lg
    text "Cross-platform content"

    when platform
      is ios
        // iOS-specific: haptic feedback button
        button "Save" .accent -> save() haptic: .success
      is android
        // Android-specific: FAB button
        fab "save" .accent -> save()
      is web
        // Web-specific: keyboard shortcut hint
        button "Save (Ctrl+S)" .accent -> save()
```

### 10.2 Platform Capability Queries

```
if platform.supports(.camera)
  button "Take Photo" -> openCamera()
else
  button "Upload Photo" -> openFilePicker()

if platform.supports(.haptics)
  on tap -> haptic(.light)

if platform.supports(.biometrics)
  button "Login with Face ID" -> authenticate()
```

### 10.3 Platform-Specific Styling

```
style navBar
  when platform
    is ios
      .translucent padding.top: safeArea.top
    is android
      .surface elevation.sm
    is web
      .surface shadow.sm position: fixed
```

### 10.4 Platform Constants

```
platform.name          // "ios" | "android" | "web" | "windows" | "terminal"
platform.version       // "17.0" (iOS), "14" (Android), etc.
platform.screenWidth   // Current screen width in platform units
platform.screenHeight  // Current screen height
platform.safeArea      // Safe area insets (top, bottom, left, right)
platform.isDarkMode    // Current system dark mode setting
platform.locale        // User's locale
platform.timezone      // User's timezone
```

---

## 11. Intermediate Representations

### 11.1 Overview

Aura uses a two-tier IR system:

```
  .aura Source → AST → Typed AST → HIR (High-level IR) → LIR (Low-level IR)
                                     │                      │
                               SwiftUI/Compose          HTML/CSS/WinUI
                               consume HIR              consume LIR
```

### 11.2 High-Level IR (HIR)

The HIR preserves semantic intent. It describes WHAT the UI should do, not HOW.

#### HIR Node Types

```
HIRModule
├── HIRApp(name, theme, navigation)
├── HIRModel(name, fields: [HIRField])
├── HIRScreen(name, params, state, view, actions, functions)
├── HIRComponent(name, props, state, view, actions, functions)
└── HIRTheme(name, tokens, palette, variants)

HIRView (tree)
├── HIRColumn(gap, padding, alignment, children)
├── HIRRow(gap, padding, alignment, children)
├── HIRStack(children)
├── HIRGrid(columns, gap, children)
├── HIRScroll(direction, children)
├── HIRText(content, size, weight, color)
├── HIRHeading(content, level, size)
├── HIRImage(source, size, shape)
├── HIRIcon(name, size, color)
├── HIRButton(label, style, action, design_tokens)
├── HIRTextField(binding, placeholder, design_tokens)
├── HIRCheckbox(binding)
├── HIRToggle(binding, label)
├── HIRSlider(binding, min, max, step)
├── HIRPicker(binding, options)
├── HIRList(items, template, key)
├── HIRConditional(condition, then_branch, else_branch)
├── HIRSwitch(expression, cases)
├── HIRSpacer
├── HIRDivider(style)
├── HIRNavigationLink(destination, children)
├── HIRAnimate(curve, children)
└── HIRSlot

HIRAction
├── HIRAssign(target, value)
├── HIRCall(function, arguments)
├── HIRNavigate(destination)
├── HIREmit(event)
├── HIRConditional(condition, then_action, else_action)
└── HIRSequence(actions)

HIRExpression
├── HIRLiteral(value, type)
├── HIRVariable(name, type)
├── HIRMemberAccess(object, member)
├── HIRFunctionCall(function, arguments)
├── HIRBinaryOp(op, left, right)
├── HIRUnaryOp(op, operand)
├── HIRLambda(params, body)
├── HIRConstructor(type, arguments)
├── HIRPipe(left, right)
└── HIRConditionalExpr(condition, then_expr, else_expr)
```

#### HIR Design Resolution

Every HIR node carries resolved design tokens:

```rust
struct HIRDesign {
    spacing: Option<ResolvedSpacing>,    // Computed from theme + token
    typography: Option<ResolvedTypography>,
    color: Option<ResolvedColor>,         // Semantic → actual RGBA
    shape: Option<ResolvedShape>,
    motion: Option<ResolvedMotion>,
}
```

### 11.3 Low-Level IR (LIR)

The LIR breaks semantic nodes into rendering primitives. Backends that cannot directly express HIR concepts (e.g., HTML doesn't have a native "List" widget) consume LIR instead.

#### LIR Node Types

```
LIRNode
├── LIRRect(x, y, width, height, fill, stroke, radius)
├── LIRText(content, x, y, font, size, color, align)
├── LIRImage(source, x, y, width, height, fit)
├── LIRGroup(transform, children)
├── LIRScroll(direction, content_size, children)
├── LIRInput(kind, x, y, width, height, value_binding)
├── LIRTouchTarget(x, y, width, height, action)
├── LIRAnimation(property, from, to, curve, duration)
└── LIRConditional(condition, then_nodes, else_nodes)

LIRLayout
├── LIRFlexColumn(gap, padding, align, justify, children)
├── LIRFlexRow(gap, padding, align, justify, children)
├── LIRAbsolute(children_with_positions)
└── LIRGrid(columns, rows, gap, children)
```

### 11.4 HIR → LIR Lowering

The lowering pass converts semantic HIR to primitive LIR:

```
HIRColumn(gap.md, padding.lg, [children])
  ↓ lowers to
LIRFlexColumn(gap: 8, padding: 16, align: stretch, justify: start, [lowered_children])

HIRButton("Save", .accent, saveAction)
  ↓ lowers to
LIRTouchTarget(width: auto, height: 44, action: saveAction,
  LIRRect(fill: theme.accent, radius: 8,
    LIRText("Save", color: white, font: system, size: 16, align: center)))

HIRList(todos, todoTemplate, key: .id)
  ↓ lowers to
LIRScroll(direction: vertical,
  LIRFlexColumn(gap: 0,
    [for each todo: lowered(todoTemplate(todo))]))
```

### 11.5 Backend Selection

Each backend declares which HIR nodes it can handle natively:

```rust
trait AuraBackend {
    fn supported_hir_nodes(&self) -> Vec<HIRNodeKind>;
    // If a node is NOT in this list, it is lowered to LIR first
}

// SwiftUI backend supports most HIR directly:
// HIRList → SwiftUI List (native)
// HIRNavigationLink → NavigationLink (native)
// HIRToggle → Toggle (native)

// Web backend may lower some:
// HIRList → LIR → <div> with overflow:scroll
// HIRNavigationLink → LIR → <a> with client-side routing
```

---

## 12. Error System

### 12.1 Error Philosophy

Aura's error system is designed for two audiences: **human developers** and **AI coding agents**. Every error includes both a human-readable message and a machine-readable structured payload.

### 12.2 Error Structure

```
AuraError {
  code: ErrorCode,          // E0123
  severity: Severity,       // Error | Warning | Info
  message: text,            // Human-readable description
  location: SourceSpan,     // File, line, column, span
  labels: [Label],          // Highlighted source regions
  help: optional[text],     // Human-readable fix suggestion
  fix: optional[Fix],       // Machine-applicable fix
  related: [AuraError],     // Related errors (for context)
  suppressed: int,          // Number of cascade errors suppressed
}

Fix {
  action: FixAction,        // Replace | Insert | Delete
  span: SourceSpan,         // Where to apply
  replacement: text,        // What to replace with
  confidence: float,        // 0.0 - 1.0
}

Severity {
  Error,      // Prevents compilation
  Warning,    // Compiles but likely wrong
  Info,       // Suggestion for improvement
}
```

### 12.3 Error Poisoning

When the compiler encounters an error, it marks the affected AST node as "poisoned." All downstream errors that depend on the poisoned node are **suppressed**.

Algorithm:
1. On error, mark the error's AST node with `poisoned = true`.
2. During type checking, if an expression references a poisoned node, skip type checking for that expression and mark it as poisoned too.
3. Poisoned nodes propagate transitively.
4. Only non-poisoned errors are reported.
5. The count of suppressed errors is included: `"1 error found (4 more suppressed — fix above first)"`

```
// Source with one typo:
screen Main
  state todoos: list[Todo] = []     // Typo: todoos

  view
    each todoos as todo              // References poisoned name
      text todoos.title              // References poisoned name
    text "Count: {todoos.count()}"   // References poisoned name

// WITHOUT poisoning: 4 errors (confusing)
// WITH poisoning: 1 error + "3 suppressed"
//   error[E0103]: unknown variable 'todoos'
//     help: did you mean 'todos'? (confidence: 0.97)
//     suppressed: 3 downstream errors
```

### 12.4 Confidence Scoring

Every `Fix` has a confidence score (0.0 - 1.0):

| Confidence | Meaning | AI Agent Behavior |
|---|---|---|
| 0.95 - 1.0 | Almost certainly correct | Auto-apply |
| 0.8 - 0.95 | Likely correct | Auto-apply with log |
| 0.5 - 0.8 | Plausible | Present to user for confirmation |
| 0.0 - 0.5 | Uncertain | Present multiple alternatives |

Confidence is computed from:
- **Edit distance** (for typos): "todoos" → "todos" = 0.97 (1 deletion)
- **Type compatibility** (for type errors): if only one type in scope matches = 0.95
- **Context frequency** (for ambiguity): most-used alternative in this file = higher confidence

### 12.5 Error Output Formats

#### Human Format (default)

```
error[E0103]: unknown variable 'todoos'
  --> src/main.aura:5:11
   |
 5 |   each todoos as todo
   |        ^^^^^^
   |
   = help: did you mean 'todos'?
   = note: 3 downstream errors suppressed (fix this first)
```

#### Agent Format (`--format=json`)

```json
{
  "errors": [{
    "code": "E0103",
    "severity": "error",
    "message": "unknown variable 'todoos'",
    "location": {
      "file": "src/main.aura",
      "line": 5,
      "column": 11,
      "span": [11, 17]
    },
    "fix": {
      "action": "replace",
      "span": [11, 17],
      "replacement": "todos",
      "confidence": 0.97
    },
    "suppressed": 3,
    "context": {
      "available_names": ["todos", "todo", "filter"],
      "enclosing_scope": "screen Main",
      "expected_type": "iterable"
    }
  }],
  "summary": {
    "errors": 1,
    "warnings": 0,
    "suppressed": 3
  }
}
```

### 12.6 Error Categories and Codes

```
  E0001-E0099:  Lexer errors
    E0001       Invalid UTF-8 encoding
    E0002       File exceeds maximum size (10 MB)
    E0010       Tab character found (use spaces)
    E0011       Inconsistent indentation
    E0020       Unterminated string literal
    E0021       Invalid escape sequence
    E0030       Invalid numeric literal

  E0100-E0199:  Type errors
    E0100       Nil used for non-optional type
    E0101       Type mismatch
    E0102       Cannot infer type
    E0103       Undefined variable
    E0104       Undefined type
    E0105       Duplicate field name
    E0106       Missing required field
    E0107       Circular type dependency
    E0108       Cannot call non-function
    E0109       Wrong number of arguments
    E0110       Optional access without nil check

  E0200-E0299:  Security type errors
    E0200       Secret value in API response
    E0201       Secret value in log output
    E0202       Secret value in string interpolation
    E0203       Secret value in equality comparison
    E0204       Secret value in serialization
    E0210       Unsanitized text in rendered view
    E0220       Invalid email format (compile-time literal)
    E0230       Invalid URL format (compile-time literal)
    E0240       Expired token access without check

  E0300-E0399:  State errors
    E0300       State mutation outside action block
    E0301       State mutation in view block
    E0302       State mutation in pure function
    E0310       Undeclared state variable

  E0400-E0499:  Design errors
    E0400       Unknown design token
    E0401       Incompatible design tokens
    E0410       Theme not found
    E0411       Theme missing required field

  E0500-E0599:  Platform errors
    E0500       Platform API not available on target
    E0501       Missing platform guard
    E0510       Platform SDK not installed

  E0600-E0699:  Navigation errors
    E0600       Unknown screen destination
    E0601       Missing required screen parameter
    E0602       Circular navigation reference

  E0700-E0799:  Parse errors
    E0700       Unexpected token
    E0701       Expected expression
    E0702       Expected identifier
    E0710       Maximum nesting depth exceeded (128)
    E0720       Unexpected end of file

  E0900-E0999:  Internal compiler errors
    E0900       Internal compiler error (bug in Aura)
    E0901       Codegen failure
    E0902       IR lowering failure

  W0001-W0099:  Warnings
    W0001       Confusable Unicode characters in identifier
    W0010       Unused variable
    W0011       Unused import
    W0020       Unreachable code
    W0050       Text/background contrast ratio below WCAG AA
    W0051       Touch target below minimum size
    W0060       Missing accessibility label on interactive element
```

---

## 13. Module System & Packages

### 13.1 File-Based Modules

Each `.aura` file is a module. The module name is the file name (without extension).

```
project/
├── aura.toml              # Project configuration
├── src/
│   ├── main.aura          # App entry point
│   ├── models/
│   │   ├── user.aura      # module: models.user
│   │   └── todo.aura      # module: models.todo
│   ├── screens/
│   │   ├── home.aura      # module: screens.home
│   │   └── settings.aura  # module: screens.settings
│   └── components/
│       ├── card.aura       # module: components.card
│       └── avatar.aura     # module: components.avatar
└── themes/
    └── custom.aura         # module: themes.custom
```

### 13.2 Imports

```
// Import from local module
import Todo from "./models/todo"
import { UserCard, UserAvatar } from "./components/user"

// Import from package
import Chart from "@aura/charts"
import { LineChart, BarChart } from "@aura/charts"

// Import everything
import * as Icons from "@aura/icons"
```

### 13.3 Project Configuration (aura.toml)

```toml
[app]
name = "TaskMaster"
version = "1.0.0"
aura-version = "0.1.0"

[targets]
web = true
ios = true
android = true
windows = false

[dependencies]
"@aura/charts" = "^1.0.0"
"@aura/icons" = "^2.0.0"

[theme]
default = "modern.dark"
```

### 13.4 Package Format

Packages are published to the Aura registry with:

```
aura pkg publish
```

Package requirements:
- `aura.toml` with name, version, description, author
- All source in `src/`
- Cryptographic signature (ed25519)
- No build scripts (packages are source-only, compiled by the consumer)
- Content-addressable storage (SHA-256 hash of contents)

Package naming: `@publisher/package-name` (scoped to prevent typosquatting).

---

## 14. Agent API Protocol

### 14.1 Overview

The Agent API allows AI coding agents to interact with Aura programs structurally — operating on the AST rather than editing text. This eliminates syntax errors from text manipulation and enables semantic operations.

### 14.2 Connection

```
// Start Agent API server:
aura agent serve --port 7432

// Connect via JSON-RPC over WebSocket or Unix socket
```

### 14.3 Operations

#### Read Operations

```json
// Get the full AST
{"method": "ast.get", "params": {"file": "src/main.aura"}}

// Query specific nodes
{"method": "ast.query", "params": {
  "file": "src/main.aura",
  "selector": "screen.Main.view"
}}

// Get type information
{"method": "types.get", "params": {
  "file": "src/main.aura",
  "location": {"line": 15, "column": 8}
}}

// Get completions
{"method": "completions.get", "params": {
  "file": "src/main.aura",
  "location": {"line": 15, "column": 8}
}}

// Get diagnostics (errors/warnings)
{"method": "diagnostics.get", "params": {"file": "src/main.aura"}}
```

#### Write Operations

```json
// Insert a new node
{"method": "ast.insert", "params": {
  "file": "src/main.aura",
  "parent": "screen.Main.view.column",
  "index": 2,
  "node": {
    "kind": "HIRButton",
    "label": "New Button",
    "style": "accent",
    "action": {"kind": "HIRCall", "function": "doSomething"}
  },
  "version": 42
}}

// Modify a node
{"method": "ast.modify", "params": {
  "file": "src/main.aura",
  "target": "screen.Main.view.column.children[0]",
  "changes": {"content": "Updated Text", "size": "xl"},
  "version": 42
}}

// Delete a node
{"method": "ast.delete", "params": {
  "file": "src/main.aura",
  "target": "screen.Main.view.column.children[3]",
  "version": 42
}}

// Batch mutations (atomic)
{"method": "ast.batch", "params": {
  "file": "src/main.aura",
  "operations": [...],
  "version": 42
}}
```

### 14.4 Concurrency Control

- Every file has a **version number** (monotonically increasing).
- All write operations include the expected version.
- If the version doesn't match (another agent modified the file), the operation is rejected with a conflict error containing both versions.
- Batch operations are atomic: all succeed or none do.

### 14.5 Validation

All AST mutations are validated before applying:
1. **Grammar check:** Is the resulting AST valid Aura?
2. **Type check:** Does the mutation preserve type safety?
3. **Design check:** Do design tokens resolve correctly?
4. **Security check:** Are security types still enforced?

If any check fails, the mutation is rejected with a structured error (same format as compiler errors, with confidence-scored fix suggestions).

### 14.6 Rate Limiting

- Default: 100 mutations per second per agent
- Configurable in `aura.toml`
- Exceeding the limit returns error with `retry-after` header

---

## 15. Standard Library

### 15.1 Collection Methods

#### list[T]

```
.count() -> int
.isEmpty -> bool
.first -> optional[T]
.last -> optional[T]
.append(item: T) -> list[T]
.prepend(item: T) -> list[T]
.insert(item: T, at: int) -> list[T]
.remove(item: T) -> list[T]
.remove(at: int) -> list[T]
.where(predicate: fn(T) -> bool) -> list[T]
.map(transform: fn(T) -> U) -> list[U]
.flatMap(transform: fn(T) -> list[U]) -> list[U]
.reduce(initial: U, fn(U, T) -> U) -> U
.sortBy(key: fn(T) -> Comparable) -> list[T]
.reversed() -> list[T]
.take(n: int) -> list[T]
.drop(n: int) -> list[T]
.contains(item: T) -> bool
.find(predicate: fn(T) -> bool) -> optional[T]
.enumerate() -> list[(int, T)]
.zip(other: list[U]) -> list[(T, U)]
.distinct() -> list[T]
.grouped(by: fn(T) -> K) -> map[K, list[T]]
```

#### map[K, V]

```
.count() -> int
.isEmpty -> bool
.keys -> list[K]
.values -> list[V]
.get(key: K) -> optional[V]
.set(key: K, value: V) -> map[K, V]
.remove(key: K) -> map[K, V]
.contains(key: K) -> bool
.merge(other: map[K, V]) -> map[K, V]
.map(transform: fn(K, V) -> (K2, V2)) -> map[K2, V2]
.where(predicate: fn(K, V) -> bool) -> map[K, V]
```

#### set[T]

```
.count() -> int
.isEmpty -> bool
.add(item: T) -> set[T]
.remove(item: T) -> set[T]
.contains(item: T) -> bool
.union(other: set[T]) -> set[T]
.intersection(other: set[T]) -> set[T]
.difference(other: set[T]) -> set[T]
```

### 15.2 Text Methods

```
.count() -> int
.isEmpty -> bool
.contains(substring: text) -> bool
.startsWith(prefix: text) -> bool
.endsWith(suffix: text) -> bool
.trim() -> text
.trimStart() -> text
.trimEnd() -> text
.uppercase() -> text
.lowercase() -> text
.capitalize() -> text
.replace(old: text, new: text) -> text
.split(separator: text) -> list[text]
.join(separator: text) -> text               // On list[text]
.slice(from: int, to: int) -> text
.padStart(length: int, char: text) -> text
.padEnd(length: int, char: text) -> text
.toInt() -> optional[int]
.toFloat() -> optional[float]
.sanitize(maxLength: int = 10000) -> sanitized
.matches(pattern: text) -> bool              // Regex
```

### 15.3 Numeric Methods

```
// int
.toFloat() -> float
.toText() -> text
.abs() -> int
.clamp(min: int, max: int) -> int

// float
.toInt() -> int
.toText(decimals: int = 2) -> text
.abs() -> float
.ceil() -> int
.floor() -> int
.round() -> int
.clamp(min: float, max: float) -> float

// Math constants and functions
math.pi -> float
math.e -> float
math.random() -> float              // 0.0 to 1.0
math.random(min: int, max: int) -> int
math.min(a: number, b: number) -> number
math.max(a: number, b: number) -> number
math.pow(base: float, exp: float) -> float
math.sqrt(x: float) -> float
```

### 15.4 Timestamp Methods

```
now() -> timestamp
today() -> timestamp                 // Midnight today UTC

// On timestamp:
.format(pattern: text) -> text       // "MMM d, yyyy" → "Apr 4, 2026"
.year -> int
.month -> int
.day -> int
.hour -> int
.minute -> int
.second -> int
.add(duration) -> timestamp
.subtract(duration) -> timestamp
.since(other: timestamp) -> duration
.isBefore(other: timestamp) -> bool
.isAfter(other: timestamp) -> bool

// Duration constructors
1.seconds
5.minutes
2.hours
1.days
1.weeks
```

### 15.5 Built-in Functions

```
print(value: any)                    // Debug output (dev only, stripped in release)
assert(condition: bool, message: text) // Debug assertion
now() -> timestamp
uuid() -> text                       // Generate UUID v4
```

---

## 16. Backend Trait Interface

### 16.1 Trait Definition

Every codegen backend implements this trait:

```rust
pub trait AuraBackend {
    /// Backend identifier
    fn name(&self) -> &str;

    /// Platform target
    fn platform(&self) -> Platform;

    /// Which HIR nodes this backend handles natively.
    /// Nodes NOT in this list are lowered to LIR first.
    fn supported_hir_nodes(&self) -> Vec<HIRNodeKind>;

    /// Compile a complete Aura module
    fn emit_module(&self, module: &HIRModule) -> Result<BackendOutput, CodegenError>;

    /// Compile individual HIR nodes (called by emit_module)
    fn emit_app(&self, app: &HIRApp) -> Result<String, CodegenError>;
    fn emit_screen(&self, screen: &HIRScreen) -> Result<String, CodegenError>;
    fn emit_component(&self, comp: &HIRComponent) -> Result<String, CodegenError>;
    fn emit_model(&self, model: &HIRModel) -> Result<String, CodegenError>;
    fn emit_view(&self, view: &HIRView) -> Result<String, CodegenError>;

    // Layout
    fn emit_column(&self, col: &HIRColumn) -> Result<String, CodegenError>;
    fn emit_row(&self, row: &HIRRow) -> Result<String, CodegenError>;
    fn emit_stack(&self, stack: &HIRStack) -> Result<String, CodegenError>;
    fn emit_grid(&self, grid: &HIRGrid) -> Result<String, CodegenError>;
    fn emit_scroll(&self, scroll: &HIRScroll) -> Result<String, CodegenError>;

    // Widgets
    fn emit_text(&self, text: &HIRText) -> Result<String, CodegenError>;
    fn emit_heading(&self, heading: &HIRHeading) -> Result<String, CodegenError>;
    fn emit_image(&self, image: &HIRImage) -> Result<String, CodegenError>;
    fn emit_icon(&self, icon: &HIRIcon) -> Result<String, CodegenError>;
    fn emit_button(&self, button: &HIRButton) -> Result<String, CodegenError>;

    // Inputs
    fn emit_textfield(&self, field: &HIRTextField) -> Result<String, CodegenError>;
    fn emit_checkbox(&self, cb: &HIRCheckbox) -> Result<String, CodegenError>;
    fn emit_toggle(&self, toggle: &HIRToggle) -> Result<String, CodegenError>;
    fn emit_slider(&self, slider: &HIRSlider) -> Result<String, CodegenError>;
    fn emit_picker(&self, picker: &HIRPicker) -> Result<String, CodegenError>;

    // Control flow
    fn emit_conditional(&self, cond: &HIRConditional) -> Result<String, CodegenError>;
    fn emit_each(&self, each: &HIREach) -> Result<String, CodegenError>;
    fn emit_switch(&self, switch: &HIRSwitch) -> Result<String, CodegenError>;

    // Navigation
    fn emit_navigation(&self, nav: &HIRNavigation) -> Result<String, CodegenError>;

    // Animation
    fn emit_animate(&self, anim: &HIRAnimate) -> Result<String, CodegenError>;

    // Design
    fn emit_theme(&self, theme: &HIRTheme) -> Result<String, CodegenError>;
    fn resolve_design_token(&self, token: &DesignToken, context: &TokenContext) -> String;

    // LIR fallback (for nodes not in supported_hir_nodes)
    fn emit_lir_node(&self, node: &LIRNode) -> Result<String, CodegenError>;
}

pub struct BackendOutput {
    pub files: Vec<OutputFile>,
    pub entry_point: String,
    pub build_command: Option<String>,
}

pub struct OutputFile {
    pub path: String,
    pub content: String,
    pub language: String,    // "swift", "kotlin", "html", "css", "js"
}
```

### 16.2 Backend Conformance Tests

Every backend is tested against the same suite of `.aura` programs:

```
tests/conformance/
├── 001_hello_world.aura
├── 002_counter.aura
├── 003_todo_list.aura
├── 004_navigation.aura
├── 005_theme_switching.aura
├── 006_responsive_layout.aura
├── 007_forms_and_input.aura
├── 008_lists_and_data.aura
├── 009_conditional_views.aura
├── 010_animations.aura
├── 011_nested_components.aura
├── 012_slots.aura
├── 013_security_types.aura
├── 014_platform_adaptation.aura
├── 015_accessibility.aura
... (50+ test files)
```

For each test file and each backend:
1. Compile succeeds (no errors)
2. Output is valid syntax for the target language
3. Output structure matches expected pattern (snapshot test)
4. Design tokens resolve to correct platform values

---

## Appendix A: Complete EBNF Grammar

```ebnf
(* Aura Language Grammar — EBNF *)

program            = { import_decl } , app_decl ;

(* === Declarations === *)

app_decl           = "app" , IDENT , NL , INDENT , app_body , DEDENT ;
app_body           = { app_member } ;
app_member         = theme_ref | model_decl | screen_decl | component_decl
                   | style_decl | const_decl | fn_decl | state_decl
                   | navigation_decl | route_decl ;

theme_ref          = "theme" , ":" , expression , NL ;
navigation_decl    = "navigation" , ":" , IDENT , NL ;
route_decl         = "route" , STRING , "->" , IDENT , [ "(" , named_args , ")" ] , NL ;

import_decl        = "import" , import_spec , "from" , STRING , NL ;
import_spec        = IDENT
                   | "{" , IDENT , { "," , IDENT } , "}"
                   | "*" , "as" , IDENT ;

model_decl         = "model" , TYPE_IDENT , NL , INDENT , { field_decl } , DEDENT ;
field_decl         = IDENT , ":" , type_expr , [ "=" , expression ] , NL ;

screen_decl        = "screen" , TYPE_IDENT , [ "(" , param_list , ")" ] ,
                     [ screen_modifiers ] , NL , INDENT , { screen_member } , DEDENT ;
screen_modifiers   = { screen_modifier } ;
screen_modifier    = "tab" , ":" , STRING
                   | "label" , ":" , STRING ;
screen_member      = state_decl | view_decl | action_decl | fn_decl | on_decl | style_decl ;

component_decl     = "component" , TYPE_IDENT , [ "(" , param_list , ")" ] , NL ,
                     INDENT , { component_member } , DEDENT ;
component_member   = state_decl | view_decl | action_decl | fn_decl | style_decl ;

theme_decl         = "theme" , IDENT , NL , INDENT , { theme_member } , DEDENT ;
theme_member       = theme_prop | palette_decl | "variants" , ":" , list_literal , NL ;
theme_prop         = IDENT , { "." , IDENT } , ":" , expression , NL ;
palette_decl       = "palette" , NL , INDENT , { palette_entry } , DEDENT ;
palette_entry      = IDENT , { "." , IDENT } , ":" , STRING , NL ;

style_decl         = "style" , IDENT , NL , INDENT , { design_token | prop_assign } , NL , DEDENT ;

const_decl         = "const" , IDENT , [ ":" , type_expr ] , "=" , expression , NL ;

state_decl         = "state" , IDENT , ":" , type_expr , [ "=" , expression ] , NL ;

action_decl        = "action" , IDENT , [ "(" , param_list , ")" ] , NL ,
                     INDENT , { statement } , DEDENT ;

fn_decl            = "fn" , IDENT , [ "(" , param_list , ")" ] ,
                     [ "->" , type_expr ] , NL ,
                     INDENT , fn_body , DEDENT ;
fn_body            = { statement } | expression ;

on_decl            = "on" , IDENT , [ "(" , param_list , ")" ] , NL ,
                     INDENT , { statement } , DEDENT ;

(* === Types === *)

type_expr          = primitive_type | collection_type | optional_type | enum_type
                   | function_type | TYPE_IDENT ;
primitive_type     = "text" | "int" | "float" | "bool" | "timestamp" | "duration"
                   | "percent" | "secret" | "sanitized" | "email" | "url" | "token" ;
collection_type    = ( "list" | "set" ) , "[" , type_expr , "]"
                   | "map" , "[" , type_expr , "," , type_expr , "]" ;
optional_type      = "optional" , "[" , type_expr , "]" ;
enum_type          = "enum" , "[" , enum_variant , { "," , enum_variant } , "]" ;
enum_variant       = IDENT , [ "(" , param_list , ")" ] ;
function_type      = "fn" , "(" , [ type_list ] , ")" , [ "->" , type_expr ]
                   | "action" , [ "(" , [ type_list ] , ")" ] ;
type_list          = type_expr , { "," , type_expr } ;

param_list         = param , { "," , param } ;
param              = IDENT , ":" , type_expr , [ "=" , expression ] ;

(* === View === *)

view_decl          = "view" , NL , INDENT , { view_element } , DEDENT ;

view_element       = layout_elem | widget_elem | input_elem | button_elem
                   | control_flow_elem | component_ref | "spacer" , NL
                   | "divider" , { design_token } , NL
                   | "slot" , NL ;

layout_elem        = layout_kind , { design_token | prop_assign } , NL ,
                     [ INDENT , { view_element } , DEDENT ] ;
layout_kind        = "column" | "row" | "stack" | "grid" | "scroll" | "wrap" ;

widget_elem        = widget_kind , { expression | design_token | prop_assign } , NL ;
widget_kind        = "text" | "heading" | "image" | "icon" | "badge"
                   | "progress" | "avatar" ;

input_elem         = input_kind , IDENT , { design_token | prop_assign } ,
                     [ "->" , action_expr ] , NL ;
input_kind         = "textfield" | "textarea" | "checkbox" | "toggle"
                   | "slider" | "picker" | "datepicker" | "segmented" | "stepper" ;

button_elem        = "button" , [ "." , IDENT ] , expression ,
                     { design_token | prop_assign } , "->" , action_expr , NL ;

control_flow_elem  = if_elem | each_elem | when_elem ;

if_elem            = "if" , expression , NL , INDENT , { view_element } , DEDENT ,
                     [ "else" , NL , INDENT , { view_element } , DEDENT ] ;

each_elem          = "each" , expression , "as" , IDENT , [ "," , IDENT ] , NL ,
                     INDENT , { view_element } , DEDENT ;

when_elem          = "when" , expression , NL , INDENT , { when_branch } , DEDENT ;
when_branch        = "is" , pattern , NL , INDENT , { view_element } , DEDENT ;

component_ref      = TYPE_IDENT , [ "(" , [ named_args ] , ")" ] , NL ,
                     [ INDENT , { view_element } , DEDENT ] ;

(* === Expressions === *)

expression         = pipe_expr ;
pipe_expr          = conditional_expr , { "|>" , conditional_expr } ;
conditional_expr   = logic_or , [ "if" , logic_or , "then" , logic_or , "else" , logic_or ]
                   | "if" , logic_or , "then" , logic_or , "else" , logic_or ;
logic_or           = logic_and , { "or" , logic_and } ;
logic_and          = equality , { "and" , equality } ;
equality           = comparison , { ( "==" | "!=" ) , comparison } ;
comparison         = addition , { ( "<" | ">" | "<=" | ">=" ) , addition } ;
addition           = multiplication , { ( "+" | "-" ) , multiplication } ;
multiplication     = unary , { ( "*" | "/" | "%" ) , unary } ;
unary              = [ "not" | "-" ] , postfix ;
postfix            = primary , { "." , IDENT | "(" , [ args ] , ")" | "[" , expression , "]" } ;
primary            = literal | IDENT | TYPE_IDENT | "(" , expression , ")"
                   | lambda | constructor | "nil" ;

literal            = INTEGER | FLOAT | STRING | PERCENT | "true" | "false" ;
lambda             = [ IDENT | "(" , param_list , ")" ] , "=>" , expression ;
constructor        = TYPE_IDENT , "(" , [ named_args ] , ")" ;

args               = expression , { "," , expression } ;
named_args         = named_arg , { "," , named_arg } ;
named_arg          = IDENT , ":" , expression ;

pattern            = literal | IDENT | "." , IDENT | "some" , "(" , IDENT , ")"
                   | "nil" | TYPE_IDENT , [ "(" , { IDENT , ":" , IDENT } , ")" ] ;

(* === Statements === *)

statement          = assignment | let_stmt | action_call | if_stmt | when_stmt
                   | navigate_stmt | emit_stmt | return_stmt ;

assignment         = IDENT , { "." , IDENT } , "=" , expression , NL ;
let_stmt           = "let" , IDENT , [ ":" , type_expr ] , "=" , expression , NL ;
action_call        = expression , NL ;
if_stmt            = "if" , expression , NL , INDENT , { statement } , DEDENT ,
                     [ "else" , NL , INDENT , { statement } , DEDENT ] ;
when_stmt          = "when" , expression , NL , INDENT , { when_stmt_branch } , DEDENT ;
when_stmt_branch   = "is" , pattern , "->" , ( statement | expression , NL ) ;
navigate_stmt      = "navigate" , ( "(" , expression , ")" | "." , IDENT ) , NL ;
emit_stmt          = "emit" , IDENT , [ "(" , [ args ] , ")" ] , NL ;
return_stmt        = "return" , [ expression ] , NL ;

action_expr        = IDENT , [ "(" , [ args ] , ")" ]
                   | navigate_stmt_inline
                   | "->" , IDENT , [ "(" , [ args ] , ")" ] ;
navigate_stmt_inline = "navigate" , ( "(" , expression , ")" | "." , IDENT ) ;

(* === Design Tokens === *)

design_token       = "." , IDENT , { "." , IDENT } ;
prop_assign        = IDENT , ":" , expression ;

(* === Terminals === *)

IDENT              = lowercase_letter , { letter | digit | "_" } ;
TYPE_IDENT         = uppercase_letter , { letter | digit } ;
INTEGER            = digit , { digit | "_" } ;
FLOAT              = digit , { digit | "_" } , "." , digit , { digit | "_" } ;
PERCENT            = ( INTEGER | FLOAT ) , "%" ;
STRING             = '"' , { string_char } , '"' ;
NL                 = newline ;
INDENT             = increase in indentation (2 spaces) ;
DEDENT             = decrease in indentation ;
```

---

## Appendix B: Design Token Reference

### B.1 Complete Token Vocabulary

```
SPACING:       .xs .sm .md .lg .xl .2xl .3xl .4xl
DIRECTION:     .horizontal .vertical .top .bottom .left .right
               .leading .trailing .start .end
COMPOSITION:   gap.{size} padding.{size} padding.{direction}.{size}
               margin.{size} margin.{direction}.{size}

TYPOGRAPHY:
  Size:        .xs .sm .md .lg .xl .2xl .3xl .display
  Weight:      .thin .light .regular .medium .semibold .bold .heavy .black
  Style:       .italic .mono .underline .strike
  Alignment:   .center .leading .trailing
  Transform:   .uppercase .lowercase .capitalize

COLOR:         .primary .secondary .muted .accent .danger .warning .success .info
               .surface .background .divider
               .onPrimary .onAccent .onDanger .onSurface

SHAPE:         .sharp .subtle .rounded .smooth .pill .circle
               radius.{size}

SHADOW:        shadow.none shadow.sm shadow.md shadow.lg shadow.xl

MOTION:        .instant .fast .normal .slow
               .ease .spring .bounce

SIZING:        size.{token}  width.{token}  height.{token}
               .fill (100% of parent)  .fit (size to content)

ALIGNMENT:     align.center align.start align.end align.stretch
               align.top align.bottom align.leading align.trailing
               justify.center justify.start justify.end justify.between justify.around

OPACITY:       opacity.{0-100}

ELEVATION:     elevation.none elevation.sm elevation.md elevation.lg

INTERACTION:   .disabled .loading .selected .focused .hovered .pressed
```

### B.2 Token Resolution Order

1. Explicit inline value (e.g., `padding: 20`)
2. Component-level style override
3. Screen-level style override
4. Theme token value
5. Default token value (built into language)

---

## Appendix C: Error Code Reference

See Section 12.6 for the complete error code listing.

### Quick Lookup by Symptom

| Symptom | Likely Error | Fix |
|---|---|---|
| "Expected 2 spaces, found 4" | E0011 | Use 2-space indentation |
| "Unknown variable" | E0103 | Check spelling; see `help` for suggestions |
| "Type mismatch" | E0101 | Check expected vs actual type |
| "Secret in interpolation" | E0202 | Use `.verify()` instead of reading secret |
| "State mutation outside action" | E0300 | Move mutation into an `action` block |
| "Unknown design token" | E0400 | Check Appendix B for valid tokens |
| "Contrast ratio below threshold" | W0050 | Choose higher-contrast colors |
| "Internal compiler error" | E0900 | Report bug to Aura team |

---

## Document History

| Version | Date | Changes |
|---|---|---|
| 0.1.0-draft | 2026-04-04 | Initial specification |

---

*End of Aura Language Specification v0.1.0-draft*
