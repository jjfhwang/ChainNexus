// src/main.rs
/*
 * Main executable for ChainNexus
 */

use clap::Parser;
use chainnexus::{Result, run};

#[derive(Parser)]
#[command(version, about = "ChainNexus - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
