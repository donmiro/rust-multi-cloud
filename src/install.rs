use anyhow::{anyhow,Context,Result};
use which::which;
use std::process::Command;
use crate::models::CloudProvider;

pub fn install_provider(provider: &str) -> Result<()> {
    let cp = match provider {
        "aws" => CloudProvider::Aws,
        "gcp" => CloudProvider::Gcp,
        "azure" => CloudProvider::Azure,
        other => return Err(anyhow!("{} is not a supported provider", other)),
    };

    install(cp)?;
    Ok(())
}

fn install(cp: CloudProvider) -> Result<()> {
    use crate::models::PackageManager;

    #[cfg(target_os = "linux")]
    {
        if which(PackageManager::Apt.pkg_name()).is_ok() {
            sudo(&["apt-get", "update"])?;
            sudo(&["apt-get", "install", "-y", cp.cli_name(PackageManager::Apt)])?;
            return Ok(());
        }
        if which(PackageManager::Yum.pkg_name()).is_ok() {
            sudo(&["yum", "install", "-y", cp.cli_name(PackageManager::Yum)])?;
            return Ok(());
        }
        return Err(anyhow!("no supported package manager found (apt-get/yum)"));
    }

    #[cfg(target_os = "macos")]
    {
        if which(PackageManager::Brew.pkg_name()).is_ok() {
            run("brew", &["install", cp.cli_name(PackageManager::Brew)])?;
            return Ok(());
        }
        return Err(anyhow!("homebrew not found"));
    }

    #[cfg(target_os = "windows")]
    {
        if which(PackageManager::Choco.pkg_name()).is_ok() {
            run("choco", &["install", cp.cli_name(PackageManager::Choco), "-y"])?;
            return Ok(());
        }
        return Err(anyhow!("chocolatey not found"));
    }
}

fn run(cmd: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(cmd)
        .args(args)
        .status()
        .with_context(|| format!("failed to run {}", cmd))?;
    if !status.success() {
        Err(anyhow!("'{}' exited with {} ", cmd, status))?
    }

    Ok(())
}

#[cfg(any(target_os = "linux"))]
fn sudo(args: &[&str]) -> Result<()> {
    run("sudo", args)
}

#[cfg(any(target_os = "linux"))]
fn sudo(_: &[&str]) -> Result<()> {
    Err(anyhow!("sudo is only supported on Linux"))
}