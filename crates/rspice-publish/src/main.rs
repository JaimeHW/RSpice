//! CLI for the deterministic publication renderer.
//!
//! `rspice-publish render --snapshot <file> --out <dir>` turns one sealed
//! snapshot into a complete page bundle. The output directory must not
//! already exist: a bundle is written exactly once and never amended, the
//! same append-only discipline the cloud pipeline seals it under.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Args, Parser, Subcommand};
use rspice_publication_contract::PublicationSnapshot;
use rspice_publish::render_bundle;
use sha2::{Digest as _, Sha256};

/// Snapshot rejected by the contract (producer defect).
const EXIT_CONTRACT: u8 = 2;
/// Output directory already exists (operator defect).
const EXIT_OUTPUT_EXISTS: u8 = 3;
/// Filesystem or usage failure.
const EXIT_IO: u8 = 1;

#[derive(Debug, Parser)]
#[command(
    name = "rspice-publish",
    version,
    about = "Render a sealed RSpice publication snapshot into an immutable page bundle"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Render one snapshot into a new bundle directory.
    Render(RenderArgs),
}

#[derive(Debug, Args)]
struct RenderArgs {
    /// Canonical publication snapshot produced by the RSpice client.
    #[arg(long, value_name = "FILE")]
    snapshot: PathBuf,
    /// New bundle directory. It must not already exist.
    #[arg(long, value_name = "DIR")]
    out: PathBuf,
}

fn render(args: &RenderArgs) -> Result<(), (u8, String)> {
    let bytes = std::fs::read(&args.snapshot).map_err(|error| {
        (
            EXIT_IO,
            format!("cannot read {}: {error}", args.snapshot.display()),
        )
    })?;
    let snapshot = PublicationSnapshot::from_canonical_bytes(&bytes)
        .map_err(|error| (EXIT_CONTRACT, format!("snapshot rejected: {error}")))?;

    let mut digest = String::with_capacity(64);
    for byte in Sha256::digest(&bytes) {
        use std::fmt::Write as _;
        let _ = write!(digest, "{byte:02x}");
    }

    if args.out.exists() {
        return Err((
            EXIT_OUTPUT_EXISTS,
            format!("output {} already exists", args.out.display()),
        ));
    }
    let bundle =
        render_bundle(&snapshot, &digest).map_err(|error| (EXIT_CONTRACT, error.to_string()))?;

    for (path, bytes) in &bundle {
        let target = args.out.join(Path::new(path));
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent).map_err(|error| {
                (
                    EXIT_IO,
                    format!("cannot create {}: {error}", parent.display()),
                )
            })?;
        }
        std::fs::write(&target, bytes).map_err(|error| {
            (
                EXIT_IO,
                format!("cannot write {}: {error}", target.display()),
            )
        })?;
    }
    println!(
        "rendered {} assets to {} from snapshot sha256:{digest}",
        bundle.len(),
        args.out.display(),
    );
    Ok(())
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match &cli.command {
        Command::Render(args) => match render(args) {
            Ok(()) => ExitCode::SUCCESS,
            Err((code, message)) => {
                eprintln!("rspice-publish: {message}");
                ExitCode::from(code)
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory as _;

    #[test]
    fn cli_definition_is_consistent() {
        Cli::command().debug_assert();
    }
}
