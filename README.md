# Aura

**A programming language for AI coding agents with built-in design intelligence.**

Write once. Run native everywhere. Let AI do the coding.

```
app TodoApp
  theme: modern.dark

  model Todo
    title: text
    done: bool = false

  screen Main
    state todos: list[Todo] = []
    state input: text = ""

    view
      column gap.md padding.lg
        heading "My Tasks" size.xl
        row gap.sm
          textfield input placeholder: "Add task..."
          button "Add" accent -> todos.push(Todo(title: input))
        each todos as todo
          row gap.md align.center
            checkbox todo.done
            text todo.title strike: todo.done
            spacer
            button.icon "trash" danger -> todos.remove(todo)
```

This compiles to **native SwiftUI** (iOS/macOS), **Jetpack Compose** (Android), and **HTML/CSS/JS** (Web) — from a single source file.

## Quick Start

```bash
# Install
git clone https://github.com/360Labs-dev/aura.git
cd aura && cargo build --release

# Generate a prototype from English
aura sketch "todo app with dark mode and swipe to delete"

# Build for all platforms
aura build sketch.aura --target all

# Or scaffold a new project
aura init myapp
cd myapp
aura build src/main.aura --target web
```

## Why Aura?

AI coding agents generate millions of lines of code daily. But they're writing in languages designed for humans — verbose, ambiguous, full of boilerplate. Aura is different:

| Problem | Existing Languages | Aura |
|---|---|---|
| One typo causes 47 cascading errors | Standard compiler behavior | **Error poisoning** — one root cause = one error |
| Agent can't tell if its fix is right | Errors say what's wrong, not how to fix it | **Confidence-scored fixes** — 0.98 = auto-apply |
| Cross-platform means 3 codebases | React Native, Flutter | **Multi-backend codegen** — truly native output |
| Design requires separate tooling | CSS, Tailwind | **Design tokens in the grammar** — `.accent`, `.bold`, `.rounded` |
| Passwords stored as plaintext | Runtime checks, linters | **Security types** — `secret` auto-hashes, can't be logged |

## All Commands

| Command | Description |
|---|---|
| `aura build file.aura --target web` | Compile to HTML/CSS/JS |
| `aura build file.aura --target ios` | Compile to SwiftUI |
| `aura build file.aura --target android` | Compile to Jetpack Compose |
| `aura build file.aura --target all` | All platforms at once |
| `aura build file.aura --format json` | JSON errors for AI agents |
| `aura sketch "description"` | Generate app from English |
| `aura init myapp` | Scaffold new project |
| `aura fmt file.aura` | Format source code |
| `aura fmt file.aura --check` | Check formatting (CI mode) |
| `aura explain file.aura` | Code to plain English |
| `aura diff a.aura b.aura` | Semantic diff |
| `aura doctor` | Check environment |
| `aura agent serve` | Start Agent API (JSON-RPC) |
| `aura agent call method '{}'` | Test Agent API |

## Agent API

AI agents communicate with Aura through structured JSON-RPC — no text parsing needed:

```bash
# Get diagnostics with confidence-scored fix suggestions
aura agent call diagnostics.get '{"source": "app T\n  screen M\n    state todos: list[text] = []\n    view\n      text \"Hi\"\n    action test\n      todoos = []"}'
```

```json
{
  "diagnostics": [{
    "code": "E0103",
    "message": "Unknown variable 'todoos'. Did you mean 'todos'?",
    "fix": {
      "replacement": "todos",
      "confidence": 0.8
    }
  }],
  "summary": { "errors": 1, "warnings": 0 }
}
```

**Available methods:** `ping`, `ast.get`, `diagnostics.get`, `completions.get`, `hir.get`, `explain`, `sketch`

## Design Tokens

Design tokens are part of the grammar — not a library, not CSS classes:

```
column gap.md padding.lg
  heading "Title" size.2xl .bold
  text "Subtitle" .secondary
  button "Save" .accent .pill -> save()
```

Tokens resolve to platform-native values:

| Token | Web (CSS) | iOS (SwiftUI) | Android (Compose) |
|---|---|---|---|
| `gap.md` | `gap: 8px` | `spacing: 8` | `Arrangement.spacedBy(8.dp)` |
| `padding.lg` | `padding: 16px` | `.padding(16)` | `Modifier.padding(16.dp)` |
| `.bold` | `font-weight: 700` | `.fontWeight(.bold)` | `fontWeight = FontWeight.Bold` |
| `.accent` | `color: var(--color-accent)` | `.foregroundColor(.accentColor)` | `color = MaterialTheme.colorScheme.primary` |
| `.rounded` | `border-radius: 8px` | `.cornerRadius(8)` | `shape = RoundedCornerShape(8.dp)` |

## Security Types

Built-in types that enforce security at compile time:

```
model User
  name: text
  email: email          // format-validated
  password: secret      // auto-hashed, never in API response
  bio: sanitized        // XSS-safe
  api_key: token        // auto-expiring, never serialized
```

## Sketch Templates

`aura sketch` generates working prototypes from 10 built-in templates:

```bash
aura sketch "todo app with dark mode"
aura sketch "counter"
aura sketch "chat messenger"
aura sketch "weather forecast"
aura sketch "notes app"
aura sketch "user profile page"
aura sketch "countdown timer"
aura sketch "settings screen"
aura sketch "photo gallery"
aura sketch "login screen"
```

## Architecture

```
aura/
├── spec/language.md              # Formal language specification
├── crates/
│   ├── aura-core/                # Lexer, Parser, AST, Types, HIR, LIR
│   ├── aura-cli/                 # CLI binary
│   ├── aura-backend-web/         # → HTML/CSS/JS
│   ├── aura-backend-swift/       # → SwiftUI
│   ├── aura-backend-compose/     # → Jetpack Compose
│   ├── aura-agent/               # AI Agent API (JSON-RPC)
│   ├── aura-backend-win/         # → WinUI (planned)
│   ├── aura-backend-tui/         # → Terminal UI (planned)
│   ├── aura-lsp/                 # LSP server (planned)
│   └── aura-pkg/                 # Package manager (planned)
├── examples/                     # 5 example programs
├── tests/conformance/            # Cross-backend test suite
└── benchmarks/                   # AI agent benchmark definitions
```

**Compiler pipeline:**
```
.aura → Lexer → Parser → Semantic Analysis → HIR → Backend → Native Code
         │        │            │                │
         │        │      Type checking     Design token
      Indent/   Error     Security types    resolution
      Dedent   recovery   Error poisoning
```

## Project Status

| Component | Status | Tests |
|---|---|---|
| Language specification | Complete | — |
| Lexer (indentation-significant) | Complete | 22 |
| Parser (recursive descent) | Complete | 12 |
| Type system + security types | Complete | 15 |
| Semantic analysis + error poisoning | Complete | 12 |
| HIR builder + design tokens | Complete | 6 |
| Web backend (HTML/CSS/JS) | Complete | 8 |
| SwiftUI backend | Complete | 7 |
| Jetpack Compose backend | Complete | 6 |
| Agent API (JSON-RPC) | Complete | 12 |
| Sketch (prototype generator) | Complete | 6 |
| Formatter | Complete | 2 |
| Explain + Diff | Complete | 9 |
| Conformance tests | Complete | 7 |
| **Total** | | **120+** |

## Contributing

Aura is open source under the MIT license. We welcome contributions.

1. Read the [language specification](spec/language.md)
2. Look at the [examples](examples/)
3. Run `aura doctor` to check your environment
4. Run `cargo test` to verify everything works

## License

MIT License - see [LICENSE](LICENSE)
