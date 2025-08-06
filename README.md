# 🚀 rmc — Rust Multi-Cloud CLI

**`rmc`** (Rust Multi-Cloud) is a lightweight, cross-platform CLI tool for interacting with multiple cloud providers through a single interface.  
It currently supports:

- **Detection** of installed/configured cloud CLIs (AWS, GCP, Azure)
- **Context management** (profiles, regions, projects, credentials)
- Coming soon: bucket listings, file upload/download, log streaming, and more!

---

## 📦 Installation

### From GitHub Releases

1. Go to the [Releases page](https://github.com/donmiro/rust-multi-cloud/releases/latest).
2. Download the archive for your platform (e.g. `rmc-linux-x86_64.tar.gz`, `rmc-windows-x86_64.zip`, `rmc-macos-aarch64.tar.gz`).
3. Extract and move the `rmc` binary into your `PATH`:

   ```bash
   # Linux / macOS example:
   curl -LO https://github.com/donmiro/rmc/releases/latest/download/rmc-<os>-<arch>.tar.gz
   tar xzf rmc-<os>-<arch>.tar.gz
   sudo mv rmc /usr/local/bin/

   # Windows example (PowerShell):
   Invoke-WebRequest -Uri https://github.com/donmiro/rmc/releases/latest/download/rmc-windows-x86_64.zip -OutFile rmc.zip
   Expand-Archive rmc.zip -DestinationPath .
   Move-Item .\rmc.exe C:\Windows\System32\
