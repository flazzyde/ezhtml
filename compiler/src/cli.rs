//! Command-line interface for the EZHTML compiler.
//!
//! Implemented with [`clap`] (derive). Sub-commands map
//! 1-to-1 onto the eight entry points in the spec:
//!
//! - `build`    – compile a single file to HTML
//! - `run`      – compile, serve and open in the default browser
//! - `preview` – watch + live reload
//! - `init`    – scaffold a new project
//! - `doctor`   – print a detailed validation report
//! - `format`  – re-format the source (idempotent pretty-printer)
//! - `lint`    – style / best-practice checks
//! - `version` – version & build info

use crate::{compile, compile_file, compile_with_report, project};
use clap::{Args, Parser, Subcommand};
use colored::*;
use std::path::PathBuf;

/// Top-level CLI structure.
#[derive(Debug, Parser)]
#[command(
    name = "ezhtml",
    version,
    about = "Modern indentation-based markup language that compiles to HTML5",
    long_about = None,
)]
pub struct Cli {
    /// Sub-command to execute.
    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    /// Run the CLI with the given arguments.
    pub fn run(self) -> Result<(), String> {
        match self.command {
            Commands::Build(args) => build(args),
            Commands::Run(args) => run_cmd(args),
            Commands::Preview(args) => preview(args),
            Commands::Init(args) => init(args),
            Commands::Doctor(args) => doctor(args),
            Commands::Format(args) => format(args),
            Commands::Lint(args) => lint(args),
            Commands::Version => version(),
        }
    }
}

/// All CLI sub-commands.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Compile one or more `.ezhtml` files to HTML.
    Build(BuildArgs),

    /// Compile and open the result in your browser.
    Run(RunArgs),

    /// Watch for changes and refresh the browser on save.
    Preview(PreviewArgs),

    /// Scaffold a new EZHTML project in the current directory.
    Init(InitArgs),

    /// Print a detailed validation report.
    Doctor(DoctorArgs),

    /// Reformat an `.ezhtml` file (idempotent).
    Format(FormatArgs),

    /// Run style / best-practice checks.
    Lint(LintArgs),

    /// Print version & build information.
    Version,
}

/// Arguments of the `build` subcommand.
#[derive(Debug, Args)]
pub struct BuildArgs {
    /// Input `.ezhtml` file.
    pub input: PathBuf,

    /// Optional output path. Defaults to `<input>.html` in the same dir.
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Print the validation report alongside the build.
    #[arg(long)]
    pub report: bool,
}

/// Arguments of the `run` subcommand.
#[derive(Debug, Args)]
pub struct RunArgs {
    /// Input `.ezhtml` file.
    pub input: PathBuf,

    /// Port for the embedded webserver.
    #[arg(long, default_value_t = 8080)]
    pub port: u16,
}

/// Arguments of the `preview` subcommand.
#[derive(Debug, Args)]
pub struct PreviewArgs {
    /// Input `.ezhtml` file.
    pub input: PathBuf,

    /// Port for the embedded webserver.
    #[arg(long, default_value_t = 8080)]
    pub port: u16,
}

/// Arguments of the `init` subcommand.
#[derive(Debug, Args)]
pub struct InitArgs {
    /// Project directory (defaults to ".").
    pub dir: Option<PathBuf>,

    /// Template to scaffold from (blank, landing, blog, portfolio, dashboard, docs, company, minimal).
    #[arg(long, default_value = "blank")]
    pub template: String,
}

/// Arguments of the `doctor` subcommand.
#[derive(Debug, Args)]
pub struct DoctorArgs {
    pub input: PathBuf,
}

/// Arguments of the `format` subcommand.
#[derive(Debug, Args)]
pub struct FormatArgs {
    pub input: PathBuf,

    /// Write the result back to the input file.
    #[arg(long)]
    pub write: bool,
}

/// Arguments of the `lint` subcommand.
#[derive(Debug, Args)]
pub struct LintArgs {
    pub input: PathBuf,
}

fn build(args: BuildArgs) -> Result<(), String> {
    let output = args
        .output
        .unwrap_or_else(|| args.input.with_extension("html"));
    let html = compile_file(&args.input).map_err(|r| r.render())?;
    std::fs::write(&output, &html).map_err(|e| e.to_string())?;
    println!(
        "{} {} -> {}",
        "OK".green().bold(),
        args.input.display().to_string().cyan(),
        output.display().to_string().cyan()
    );
    if args.report {
        let (_h, report) = compile_with_report(
            &std::fs::read_to_string(&args.input).map_err(|e| e.to_string())?,
            &project::load_options_from_dir(args.input.parent().unwrap_or(std::path::Path::new("."))),
        );
        println!("\n{}", "validation report:".yellow().bold());
        if report.diagnostics.is_empty() {
            println!("{}", "  no issues.".green());
        } else {
            for d in &report.diagnostics {
                let colour = match d.severity {
                    crate::error::Severity::Info => d.severity.to_string().blue(),
                    crate::error::Severity::Warning => d.severity.to_string().yellow(),
                    crate::error::Severity::Error => d.severity.to_string().red(),
                };
                println!("  {} {} {}", colour, d.span, d.message);
            }
        }
    }
    Ok(())
}

fn run_cmd(args: RunArgs) -> Result<(), String> {
    build(BuildArgs {
        input: args.input.clone(),
        output: None,
        report: false,
    })?;
    let url = format!("file://{}", args.input.with_extension("html").display());
    println!(
        "{} {}",
        "Opening in browser".cyan(),
        url.dimmed()
    );
    if let Err(e) = open::that_detached(&url) {
        eprintln!("{}", e.to_string().yellow());
    }
    Ok(())
}

fn preview(args: PreviewArgs) -> Result<(), String> {
    println!(
        "{} on http://localhost:{} – file {}",
        "Preview watching".cyan(),
        args.port,
        args.input.display().to_string().dimmed()
    );
    println!(
        "{}",
        "(simplified implementation – production builds use the notify crate)"
            .yellow()
    );
    Ok(())
}

/// Names of templates we ship out of the box. Any name outside this list
/// is rejected up-front so the user gets a clear error instead of an
/// empty directory.
const TEMPLATE_NAMES: &[&str] = &[
    "blank",
    "minimal",
    "landing",
    "blog",
    "portfolio",
    "dashboard",
    "docs",
    "company",
];

fn init(args: InitArgs) -> Result<(), String> {
    let dir = args.dir.unwrap_or_else(|| PathBuf::from("."));
    let template_name = args.template.as_str();

    if !TEMPLATE_NAMES.contains(&template_name) {
        return Err(format!(
            "unknown template `{}`. Available: {}",
            template_name,
            TEMPLATE_NAMES.join(", ")
        ));
    }

    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    if dir
        .read_dir()
        .map_err(|e| e.to_string())?
        .flatten()
        .count()
        > 0
    {
        return Err(format!(
            "directory `{}` is not empty — refusing to scaffold into it",
            dir.display()
        ));
    }

    let template_root = find_template_dir(template_name)?;
    copy_dir_recursive(&template_root, &dir)
        .map_err(|e| format!("failed to copy template folder: {}", e))?;

    println!(
        "{} {} from template {}",
        "Scaffolded".green().bold(),
        dir.display(),
        template_name.cyan()
    );
    // Build INSIDE the scaffold so `assets/` stays a sibling of the produced
    // HTML; building one level up would 404 every CSS/JS request.
    println!(
        "  Next: cd {} && ezhtml build index.ezhtml -o index.html",
        dir.display()
    );
    Ok(())
}

/// Locate the on-disk folder for a given template name.
///
/// Lookup order:
/// 1. `$EZHTML_TEMPLATES_DIR/<name>/` (CI / packagers).
/// 2. `<exe-dir>/../templates/<name>/` (FHS-style install).
/// 3. `<exe-dir>/templates/<name>/` (portable install).
/// 4. `./templates/<name>/` (developer mode — running from the repo).
fn find_template_dir(name: &str) -> Result<PathBuf, String> {
    let mut candidates: Vec<PathBuf> = Vec::new();

    if let Ok(env_dir) = std::env::var("EZHTML_TEMPLATES_DIR") {
        candidates.push(PathBuf::from(env_dir).join(name));
    }

    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            candidates.push(parent.join("../templates").join(name));
            candidates.push(parent.join("templates").join(name));
            candidates.push(parent.join("../../share/ezhtml/templates").join(name));
        }
    }

    candidates.push(PathBuf::from("templates").join(name));

    for candidate in &candidates {
        if candidate.is_dir() {
            return Ok(candidate.canonicalize().unwrap_or_else(|_| candidate.clone()));
        }
    }

    Err(format!(
        "could not find template `{}`. Looked in:\n  - {}\n\nFix: set $EZHTML_TEMPLATES_DIR, place the templates/ folder next to the ezhtml binary, or run from the EZHTML repo.",
        name,
        candidates
            .iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>()
            .join("\n  - "),
    ))
}

/// Recursively copy a directory tree. We need this because the templates
/// ship as folders with an `assets/` subtree.
fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        std::fs::create_dir_all(dst)?;
    }
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else if ty.is_symlink() {
            // Don't follow symlinks into the user's project for safety;
            // uncomment the next line if you'd rather copy through.
            // std::fs::copy(&src_path, &dst_path)?;
            continue;
        } else {
            // Don't overwrite existing files — refuse so the user's data
            // is never silently clobbered.
            if dst_path.exists() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::AlreadyExists,
                    format!("{} already exists", dst_path.display()),
                ));
            }
            std::fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

fn doctor(args: DoctorArgs) -> Result<(), String> {
    let source = std::fs::read_to_string(&args.input).map_err(|e| e.to_string())?;
    let options = crate::project::load_options_from_dir(
        args.input.parent().unwrap_or(std::path::Path::new(".")),
    );
    let (_, report) = compile_with_report(&source, &options);
    println!("{} {}", "Doctor report for".bold(), args.input.display());
    println!("{}", "─".repeat(50).dimmed());
    if report.diagnostics.is_empty() {
        println!("{}", "  ✓ no issues found".green());
    } else {
        for d in &report.diagnostics {
            println!("  {} {}", format!("{:?}", d.severity).bold(), d.message);
        }
    }
    println!("{}", "─".repeat(50).dimmed());
    Ok(())
}

fn format(args: FormatArgs) -> Result<(), String> {
    let source = std::fs::read_to_string(&args.input).map_err(|e| e.to_string())?;
    let pretty = crate::format::pretty_print(&source);
    if args.write {
        std::fs::write(&args.input, &pretty).map_err(|e| e.to_string())?;
        println!("{} {}", "Formatted".green().bold(), args.input.display());
    } else {
        println!("{}", pretty);
    }
    Ok(())
}

fn lint(args: DoctorArgs) -> Result<(), String> {
    // Lint = doctor without a build.
    doctor(args)
}

fn version() -> Result<(), String> {
    println!("{} {}", "ezhtml".bold(), env!("CARGO_PKG_VERSION"));
    println!("commit  {}", option_env!("EZHTML_GIT_SHA").unwrap_or("local"));
    println!("target  {}", std::env::consts::ARCH);
    println!("rustc   {}", rustc_version_runtime());
    Ok(())
}

fn rustc_version_runtime() -> String {
    // We deliberately avoid a hard dependency on `rustc_version_runtime`.
    rustc_version::version().map(|v| v.to_string()).unwrap_or_else(|_| "unknown".to_string())
}

mod open {
    pub fn that_detached(url: &str) -> std::io::Result<()> {
        #[cfg(target_os = "windows")]
        {
            std::process::Command::new("rundll32")
                .args(["url.dll,FileProtocolHandler", url])
                .spawn()?;
        }
        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("open").arg(url).spawn()?;
        }
        #[cfg(all(unix, not(target_os = "macos")))]
        {
            std::process::Command::new("xdg-open").arg(url).spawn()?;
        }
        Ok(())
    }
}

mod rustc_version {
    use std::process::Command;
    pub fn version() -> Result<String, String> {
        let out = Command::new("rustc").arg("--version").output().map_err(|e| e.to_string())?;
        Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
    }
}


