//! # Aura CLI
//!
//! The main command-line interface for the Aura language.

use clap::{Parser, Subcommand};
use std::path::Path;

#[derive(Parser)]
#[command(name = "aura")]
#[command(about = "The Aura programming language — Design that radiates.")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile .aura files to target platforms
    Build {
        /// Target platform: web, ios, android, all
        #[arg(short, long, default_value = "web")]
        target: String,

        /// Source file or directory
        #[arg(default_value = "src/main.aura")]
        path: String,

        /// Output directory
        #[arg(short, long, default_value = "build")]
        output: String,

        /// Error format: text (default) or json (for AI agents)
        #[arg(long)]
        format: Option<String>,
    },

    /// Build and run with live preview
    Run {
        #[arg(short, long, default_value = "web")]
        target: String,
        #[arg(long)]
        preview: Option<String>,
        #[arg(short, long, default_value = "3000")]
        port: u16,
    },

    /// Format .aura source files
    Fmt {
        #[arg(default_value = "src")]
        path: String,
        #[arg(long)]
        check: bool,
    },

    /// Convert .aura code to plain English description
    Explain { file: String },

    /// Semantic diff between two .aura files
    Diff { a: String, b: String },

    /// Scaffold a new Aura project
    Init {
        name: String,
        #[arg(short, long, default_value = "app")]
        template: String,
    },

    /// Diagnose environment issues
    Doctor,

    /// Generate a running prototype from a description
    Sketch { description: String },

    /// Start the Agent API server (JSON-RPC over stdin/stdout)
    Agent {
        #[command(subcommand)]
        action: AgentCommands,
    },

    /// Package management
    Pkg {
        #[command(subcommand)]
        action: PkgCommands,
    },
}

#[derive(Subcommand)]
enum AgentCommands {
    /// Start JSON-RPC server on stdin/stdout
    Serve,
    /// Send a single request (for testing)
    Call {
        /// JSON-RPC method name
        method: String,
        /// JSON params
        #[arg(default_value = "{}")]
        params: String,
    },
}

#[derive(Subcommand)]
enum PkgCommands {
    Install { package: String },
    Update { package: Option<String> },
    Remove { package: String },
    Publish,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build {
            target,
            path,
            output,
            format,
        } => build_command(&target, &path, &output, format.as_deref()),
        Commands::Run { target, preview, port } => {
            eprintln!("  aura run not yet implemented (dev server coming in Phase 2)");
        }
        Commands::Fmt { path, check } => fmt_command(&path, check),
        Commands::Explain { file } => explain_command(&file),
        Commands::Diff { a, b } => diff_command(&a, &b),
        Commands::Init { name, template } => init_command(&name, &template),
        Commands::Doctor => doctor_command(),
        Commands::Sketch { description } => sketch_command(&description),
        Commands::Agent { action } => match action {
            AgentCommands::Serve => agent_serve(),
            AgentCommands::Call { method, params } => agent_call(&method, &params),
        },
        Commands::Pkg { action } => {
            eprintln!("  aura pkg not yet implemented");
        }
    }
}

fn build_command(target: &str, path: &str, output_dir: &str, format: Option<&str>) {
    let use_json = format == Some("json");

    eprintln!();
    eprintln!("  aura build v{}", env!("CARGO_PKG_VERSION"));
    eprintln!("  {} → {}", path, target);
    eprintln!();

    // Read source
    let source = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("  error: Cannot read '{}': {}", path, e);

            // Try to find .aura files in the path as a directory
            if Path::new(path).is_dir() {
                let main_file = Path::new(path).join("main.aura");
                if main_file.exists() {
                    match std::fs::read_to_string(&main_file) {
                        Ok(s) => {
                            eprintln!("  Found {}", main_file.display());
                            s
                        }
                        Err(e) => {
                            eprintln!("  error: Cannot read '{}': {}", main_file.display(), e);
                            std::process::exit(1);
                        }
                    }
                } else {
                    eprintln!("  hint: No main.aura found in '{}'", path);
                    std::process::exit(1);
                }
            } else {
                std::process::exit(1);
            }
        }
    };

    // Parse
    eprintln!("  [1/4] Parsing...");
    let parse_result = aura_core::parser::parse(&source);

    if !parse_result.errors.is_empty() {
        eprintln!("  {} error(s) found:", parse_result.errors.len());
        for err in &parse_result.errors {
            if use_json {
                print_error_json(err);
            } else {
                print_error_text(err, &source, path);
            }
        }
        if parse_result.program.is_none() {
            std::process::exit(1);
        }
    }

    let program = match parse_result.program {
        Some(p) => p,
        None => {
            eprintln!("  error: Failed to parse program");
            std::process::exit(1);
        }
    };

    // Semantic analysis
    eprintln!("  [2/4] Analyzing...");
    let analysis = aura_core::semantic::SemanticAnalyzer::new().analyze(&program);

    if !analysis.errors.is_empty() {
        let error_count = analysis.errors.iter().filter(|e| e.is_error()).count();
        let warning_count = analysis.errors.len() - error_count;
        if error_count > 0 {
            eprintln!("  {} error(s), {} warning(s):", error_count, warning_count);
        } else {
            eprintln!("  {} warning(s):", warning_count);
        }
        for err in &analysis.errors {
            if use_json {
                print_error_json(err);
            } else {
                print_error_text(err, &source, path);
            }
        }
    }

    // Build HIR
    eprintln!("  [3/4] Building IR...");
    let hir = aura_core::hir::build_hir(&program);

    // Codegen
    eprintln!("  [4/4] Generating {}...", target);
    match target {
        "web" => {
            let output = aura_backend_web::compile_to_web(&hir);

            // Write output files
            let out_path = Path::new(output_dir);
            std::fs::create_dir_all(out_path).expect("Failed to create output directory");

            std::fs::write(out_path.join("index.html"), &output.html)
                .expect("Failed to write index.html");
            std::fs::write(out_path.join("styles.css"), &output.css)
                .expect("Failed to write styles.css");
            std::fs::write(out_path.join("app.js"), &output.js)
                .expect("Failed to write app.js");

            eprintln!();
            eprintln!("  Build complete:");
            eprintln!("    {}/index.html  ({} bytes)", output_dir, output.html.len());
            eprintln!("    {}/styles.css  ({} bytes)", output_dir, output.css.len());
            eprintln!("    {}/app.js      ({} bytes)", output_dir, output.js.len());
            eprintln!();
            eprintln!("  Open {}/index.html in a browser to preview.", output_dir);
        }
        "ios" | "swift" => {
            let output = aura_backend_swift::compile_to_swift(&hir);

            let out_path = Path::new(output_dir);
            std::fs::create_dir_all(out_path).expect("Failed to create output directory");

            std::fs::write(out_path.join(&output.filename), &output.swift)
                .expect("Failed to write Swift file");

            eprintln!();
            eprintln!("  Build complete:");
            eprintln!("    {}/{}  ({} bytes)", output_dir, output.filename, output.swift.len());
        }
        "android" | "compose" => {
            let output = aura_backend_compose::compile_to_compose(&hir);

            let out_path = Path::new(output_dir);
            std::fs::create_dir_all(out_path).expect("Failed to create output directory");

            std::fs::write(out_path.join(&output.filename), &output.kotlin)
                .expect("Failed to write Kotlin file");

            eprintln!();
            eprintln!("  Build complete:");
            eprintln!("    {}/{}  ({} bytes)", output_dir, output.filename, output.kotlin.len());
        }
        "all" => {
            let out_base = Path::new(output_dir);

            // Web
            let web_out = out_base.join("web");
            std::fs::create_dir_all(&web_out).expect("Failed to create web output directory");
            let web = aura_backend_web::compile_to_web(&hir);
            std::fs::write(web_out.join("index.html"), &web.html).unwrap();
            std::fs::write(web_out.join("styles.css"), &web.css).unwrap();
            std::fs::write(web_out.join("app.js"), &web.js).unwrap();

            // iOS
            let ios_out = out_base.join("ios");
            std::fs::create_dir_all(&ios_out).expect("Failed to create ios output directory");
            let ios = aura_backend_swift::compile_to_swift(&hir);
            std::fs::write(ios_out.join(&ios.filename), &ios.swift).unwrap();

            // Android
            let android_out = out_base.join("android");
            std::fs::create_dir_all(&android_out).expect("Failed to create android output directory");
            let android = aura_backend_compose::compile_to_compose(&hir);
            std::fs::write(android_out.join(&android.filename), &android.kotlin).unwrap();

            eprintln!();
            eprintln!("  Build complete (all platforms):");
            eprintln!("    {}/web/         (HTML/CSS/JS)", output_dir);
            eprintln!("    {}/ios/         (SwiftUI)", output_dir);
            eprintln!("    {}/android/     (Jetpack Compose)", output_dir);
        }
        _ => {
            eprintln!("  error: Unknown target '{}'", target);
            eprintln!("  Available targets: web, ios, android, all");
            std::process::exit(1);
        }
    }
}

fn print_error_text(err: &aura_core::AuraError, source: &str, file: &str) {
    let severity = match err.severity {
        aura_core::Severity::Error => "error",
        aura_core::Severity::Warning => "warning",
        aura_core::Severity::Info => "info",
    };

    // Find line and column from byte offset
    let (line, col) = byte_to_line_col(source, err.span.start);

    eprintln!("  {}[{}]: {}", severity, err.code, err.message);
    eprintln!("    --> {}:{}:{}", file, line, col);

    if let Some(ref help) = err.help {
        eprintln!("    = help: {}", help);
    }

    if let Some(ref fix) = err.fix {
        eprintln!(
            "    = fix (confidence {:.0}%): replace with '{}'",
            fix.confidence * 100.0,
            fix.replacement
        );
    }
    eprintln!();
}

fn print_error_json(err: &aura_core::AuraError) {
    let json = serde_json::json!({
        "code": format!("{}", err.code),
        "severity": match err.severity {
            aura_core::Severity::Error => "error",
            aura_core::Severity::Warning => "warning",
            aura_core::Severity::Info => "info",
        },
        "message": err.message,
        "span": { "start": err.span.start, "end": err.span.end },
        "help": err.help,
        "fix": err.fix.as_ref().map(|f| serde_json::json!({
            "replacement": f.replacement,
            "confidence": f.confidence,
        })),
    });
    println!("{}", json);
}

fn byte_to_line_col(source: &str, byte_offset: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;
    for (i, ch) in source.char_indices() {
        if i >= byte_offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    (line, col)
}

fn fmt_command(path: &str, check: bool) {
    let source = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("  error: Cannot read '{}': {}", path, e);
            std::process::exit(1);
        }
    };

    let result = aura_core::parser::parse(&source);
    if let Some(ref program) = result.program {
        let formatted = aura_core::fmt::format(program);
        if check {
            if formatted == source {
                eprintln!("  {} is already formatted", path);
            } else {
                eprintln!("  {} needs formatting", path);
                std::process::exit(1);
            }
        } else {
            std::fs::write(path, &formatted).expect("Failed to write formatted file");
            eprintln!("  Formatted: {}", path);
        }
    } else {
        eprintln!("  error: Cannot format '{}' — parse errors:", path);
        for err in &result.errors {
            eprintln!("    {}", err.message);
        }
        std::process::exit(1);
    }
}

fn explain_command(file: &str) {
    let source = match std::fs::read_to_string(file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("  error: Cannot read '{}': {}", file, e);
            std::process::exit(1);
        }
    };

    let result = aura_core::parser::parse(&source);
    if let Some(ref program) = result.program {
        let hir = aura_core::hir::build_hir(program);
        let explanation = aura_core::explain::explain(&hir);
        println!("{}", explanation);
    } else {
        eprintln!("  error: Failed to parse '{}'", file);
        for err in &result.errors {
            eprintln!("    {}", err.message);
        }
        std::process::exit(1);
    }
}

fn diff_command(file_a: &str, file_b: &str) {
    let source_a = match std::fs::read_to_string(file_a) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("  error: Cannot read '{}': {}", file_a, e);
            std::process::exit(1);
        }
    };
    let source_b = match std::fs::read_to_string(file_b) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("  error: Cannot read '{}': {}", file_b, e);
            std::process::exit(1);
        }
    };

    let result_a = aura_core::parser::parse(&source_a);
    let result_b = aura_core::parser::parse(&source_b);

    let program_a = match result_a.program {
        Some(p) => p,
        None => {
            eprintln!("  error: Failed to parse '{}'", file_a);
            std::process::exit(1);
        }
    };
    let program_b = match result_b.program {
        Some(p) => p,
        None => {
            eprintln!("  error: Failed to parse '{}'", file_b);
            std::process::exit(1);
        }
    };

    let hir_a = aura_core::hir::build_hir(&program_a);
    let hir_b = aura_core::hir::build_hir(&program_b);

    let changes = aura_core::diff::diff(&hir_a, &hir_b);

    println!("  Aura Semantic Diff");
    println!("  {} → {}", file_a, file_b);
    println!();
    print!("{}", aura_core::diff::format_diff(&changes));
}

fn sketch_command(description: &str) {
    eprintln!();
    eprintln!("  aura sketch");
    eprintln!("  Description: \"{}\"", description);
    eprintln!();

    let code = aura_core::sketch::sketch(description);

    // Verify it parses
    let result = aura_core::parser::parse(&code);
    if result.program.is_none() {
        eprintln!("  warning: generated code has parse issues (template bug)");
    }

    // Write to file
    let filename = "sketch.aura";
    std::fs::write(filename, &code).expect("Failed to write sketch.aura");

    eprintln!("  Generated: {} ({} lines)", filename, code.lines().count());
    eprintln!();

    // Also print to stdout
    println!("{}", code);

    eprintln!("  Building preview...");

    // Auto-build for web
    let hir = aura_core::hir::build_hir(result.program.as_ref().unwrap());
    let output = aura_backend_web::compile_to_web(&hir);

    let out_dir = "build/sketch";
    std::fs::create_dir_all(out_dir).ok();
    std::fs::write(format!("{}/index.html", out_dir), &output.html).ok();
    std::fs::write(format!("{}/styles.css", out_dir), &output.css).ok();
    std::fs::write(format!("{}/app.js", out_dir), &output.js).ok();

    eprintln!("  Preview: {}/index.html", out_dir);
    eprintln!();
    eprintln!("  Open sketch.aura to customize, or run:");
    eprintln!("    aura build sketch.aura --target all");
}

fn init_command(name: &str, template: &str) {
    let dir = Path::new(name);
    if dir.exists() {
        eprintln!("  error: Directory '{}' already exists", name);
        std::process::exit(1);
    }

    let app_name = Path::new(name)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let app_name = if app_name.is_empty() { "MyApp".to_string() } else {
        let mut chars = app_name.chars();
        chars.next().map(|c| c.to_uppercase().to_string()).unwrap_or_default() + chars.as_str()
    };

    std::fs::create_dir_all(dir.join("src")).expect("Failed to create project directory");

    // aura.toml
    let toml = format!(r#"[app]
name = "{}"
version = "0.1.0"
aura-version = "0.1.0"

[targets]
web = true
ios = true
android = true

[theme]
default = "modern.light"
"#, app_name);
    std::fs::write(dir.join("aura.toml"), toml).expect("Failed to write aura.toml");

    // src/main.aura
    let main_aura = match template {
        "counter" => aura_core::sketch::sketch("counter app"),
        "todo" => aura_core::sketch::sketch("todo app with filter"),
        "chat" => aura_core::sketch::sketch("chat app"),
        _ => format!(r#"app {}
  theme: modern.light

  screen Main
    view
      column gap.lg padding.2xl align.center
        heading "{}" size.2xl .bold
        text "Welcome to your new Aura app!" .secondary
        button "Get Started" .accent .pill -> getStarted()

    action getStarted
      return
"#, app_name, app_name),
    };
    std::fs::write(dir.join("src/main.aura"), main_aura).expect("Failed to write main.aura");

    // .gitignore
    std::fs::write(dir.join(".gitignore"), "build/\n").ok();

    eprintln!();
    eprintln!("  Created project: {}/", name);
    eprintln!();
    eprintln!("  {}/aura.toml       Project configuration", name);
    eprintln!("  {}/src/main.aura   Entry point", name);
    eprintln!();
    eprintln!("  Next steps:");
    eprintln!("    cd {}", name);
    eprintln!("    aura build src/main.aura --target web");
    eprintln!("    aura build src/main.aura --target all");
}

fn agent_serve() {
    let server = aura_agent::AgentServer::new();
    eprintln!("  Aura Agent API v{}", env!("CARGO_PKG_VERSION"));
    eprintln!("  Listening on stdin/stdout (JSON-RPC 2.0)");
    eprintln!("  Send {{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"ping\",\"params\":{{}}}} to test");
    eprintln!();

    let stdin = std::io::stdin();
    let mut line = String::new();
    loop {
        line.clear();
        match stdin.read_line(&mut line) {
            Ok(0) => break, // EOF
            Ok(_) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                let response = server.handle_json(trimmed);
                println!("{}", response);
            }
            Err(e) => {
                eprintln!("  error reading stdin: {}", e);
                break;
            }
        }
    }
}

fn agent_call(method: &str, params_str: &str) {
    let params: serde_json::Value = serde_json::from_str(params_str).unwrap_or_else(|e| {
        eprintln!("  error: Invalid JSON params: {}", e);
        std::process::exit(1);
    });

    let server = aura_agent::AgentServer::new();
    let request = aura_agent::Request {
        jsonrpc: "2.0".to_string(),
        id: serde_json::json!(1),
        method: method.to_string(),
        params,
    };
    let response = server.handle_request(&request);
    println!("{}", serde_json::to_string_pretty(&response).unwrap());
}

fn doctor_command() {
    eprintln!();
    eprintln!("  Aura Doctor v{}", env!("CARGO_PKG_VERSION"));
    eprintln!("  Checking development environment...");
    eprintln!();

    let mut all_ok = true;

    // Check Rust
    let rust_ok = std::process::Command::new("rustc").arg("--version").output().is_ok();
    if rust_ok {
        let version = std::process::Command::new("rustc").arg("--version").output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_default();
        eprintln!("  [ok] Rust: {}", version);
    } else {
        eprintln!("  [!!] Rust: NOT FOUND — install from https://rustup.rs");
        all_ok = false;
    }

    // Check Cargo
    let cargo_ok = std::process::Command::new("cargo").arg("--version").output().is_ok();
    if cargo_ok {
        eprintln!("  [ok] Cargo: installed");
    } else {
        eprintln!("  [!!] Cargo: NOT FOUND");
        all_ok = false;
    }

    // Check for web target (Node.js — optional, for dev server)
    let node_ok = std::process::Command::new("node").arg("--version").output();
    match node_ok {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            eprintln!("  [ok] Node.js: {} (for web dev server)", version);
        }
        _ => {
            eprintln!("  [--] Node.js: not found (optional, for web dev server)");
        }
    }

    // Check for iOS target (Xcode)
    let xcode_ok = std::process::Command::new("xcodebuild").arg("-version").output();
    match xcode_ok {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout).lines().next().unwrap_or("").to_string();
            eprintln!("  [ok] Xcode: {} (for iOS/macOS target)", version);
        }
        _ => {
            eprintln!("  [--] Xcode: not found (needed for --target ios)");
        }
    }

    // Check for Android target
    let android_home = std::env::var("ANDROID_HOME").or_else(|_| std::env::var("ANDROID_SDK_ROOT"));
    match android_home {
        Ok(path) => {
            eprintln!("  [ok] Android SDK: {} (for Android target)", path);
        }
        Err(_) => {
            eprintln!("  [--] Android SDK: not found (needed for --target android)");
        }
    }

    // Check for iOS target (save result)
    let ios_ready = std::process::Command::new("xcodebuild")
        .arg("-version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    // Check for Android target (save result)
    let android_ready = std::env::var("ANDROID_HOME").is_ok() || std::env::var("ANDROID_SDK_ROOT").is_ok();

    // Check Aura itself
    eprintln!("  [ok] Aura: v{}", env!("CARGO_PKG_VERSION"));

    eprintln!();
    if all_ok {
        eprintln!("  All required tools are installed.");
    } else {
        eprintln!("  Some required tools are missing. Install them and run `aura doctor` again.");
    }

    // Target readiness
    eprintln!();
    eprintln!("  Target readiness:");
    eprintln!("    web:     Ready (no external dependencies)");
    eprintln!("    ios:     {}", if ios_ready { "Ready" } else { "Needs Xcode" });
    eprintln!("    android: {}", if android_ready { "Ready" } else { "Needs Android SDK" });
    eprintln!();
}
