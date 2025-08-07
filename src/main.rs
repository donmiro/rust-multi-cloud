mod utils;
mod install;
mod models;

use anyhow::Result;
use clap::{Parser, Subcommand};
use utils::{detect_aws, detect_gcp, detect_azure};
use crate::install::install_provider;

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

    // Install missing cloud CLI
    InstallProvider {
        #[arg(long,value_parser=["aws", "azure", "gcp"])]
        provider: String,
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::GetProviders => cmd_get_providers(),
        Commands::InstallProvider { provider } => cmd_install_provider(&provider)?,
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

fn cmd_install_provider(provider: &str) -> anyhow::Result<()> {
    println!("installing {} CLI...", provider);
    install_provider(provider)?;
    println!("✅ {} successfully installed (or was already present)", provider);
    Ok(())
}
