//! Binary entry point.

#![deny(unsafe_code)]

use clap::Parser;
use ezhtml::cli::Cli;

fn main() {
    let cli = Cli::parse();
    if let Err(e) = cli.run() {
        eprintln!("ezhtml error: {}", e);
        std::process::exit(1);
    }
}
