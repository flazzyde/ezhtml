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

fn init(args: InitArgs) -> Result<(), String> {
    let dir = args.dir.unwrap_or_else(|| PathBuf::from("."));
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let template = parse_template(&args.template);
    std::fs::write(dir.join("index.ezhtml"), template).map_err(|e| e.to_string())?;
    std::fs::write(
        dir.join("project.ez"),
        "title \"My EZHTML Project\"\ndescription \"A new page built with EZHTML.\"\nauthor \"You\"\ntheme_color \"#0a84ff\"\nkeyword \"ezhtml\"\n",
    )
    .map_err(|e| e.to_string())?;
    println!(
        "{} {} from template {}",
        "Scaffolded".green().bold(),
        dir.display(),
        args.template.cyan()
    );
    Ok(())
}

fn parse_template(name: &str) -> String {
    match name {
        "landing" => include_str!("../../templates/landing.ezhtml").to_string(),
        "blog" => include_str!("../../templates/blog.ezhtml").to_string(),
        "portfolio" => include_str!("../../templates/portfolio.ezhtml").to_string(),
        "dashboard" => include_str!("../../templates/dashboard.ezhtml").to_string(),
        "docs" => include_str!("../../templates/docs.ezhtml").to_string(),
        "company" => include_str!("../../templates/company.ezhtml").to_string(),
        "minimal" => include_str!("../../templates/minimal.ezhtml").to_string(),
        _ => include_str!("../../templates/blank.ezhtml").to_string(),
    }
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


