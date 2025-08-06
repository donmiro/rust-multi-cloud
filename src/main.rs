mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand};
use utils::{detect_aws, detect_gcp, detect_azure};

#[derive(Parser)]
#[command(name = "rmc", about = "Rust Mutli-Cloud CLI")]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // Show which providers the system is already "connected" to
    GetProviders,

    // Managing contexts (set/get-contexts)
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::GetProviders => cmd_get_providers(),
    }

    Ok(())
}

fn cmd_get_providers() {
    let mut list = Vec::new();

    if detect_aws() { list.push("aws") };
    if detect_gcp() { list.push("gcp") };
    if detect_azure() { list.push("azure") };

    if list.is_empty() {
        println!("no cloud CLI/configs detected");
    } else {
        for p in list {
            println!("{}", p);
        }
    }
}
