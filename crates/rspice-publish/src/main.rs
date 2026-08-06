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
use rspice_publish::{ViewerRuntime, render_bundle};
use sha2::{Digest as _, Sha256};

/// The viewer runtime staged by the component build (see `build.rs`).
mod viewer_embed {
    include!(concat!(env!("OUT_DIR"), "/viewer_embed.rs"));
}

/// Snapshot rejected by the contract (producer defect).
const EXIT_CONTRACT: u8 = 2;
/// Output directory already exists (operator defect).
const EXIT_OUTPUT_EXISTS: u8 = 3;
/// This build embeds no viewer runtime (component build defect). Checked
/// before any input is read so the cloud executor classifies it as a
/// renderer failure, never as a rejected snapshot.
const EXIT_RUNTIME_MISSING: u8 = 4;
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
    /// Print this build's identity and embedded viewer runtime digests.
    ComponentInfo,
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

/// The runtime this build seals into bundles, or the exact defect that
/// makes rendering impossible. Checked before any input is touched.
fn embedded_runtime() -> Result<ViewerRuntime, (u8, String)> {
    if !viewer_embed::EMBEDDED {
        return Err((
            EXIT_RUNTIME_MISSING,
            "this build embeds no viewer runtime; component builds set \
             RSPICE_VIEWER_RUNTIME_DIR before compiling"
                .to_string(),
        ));
    }
    ViewerRuntime::new(
        viewer_embed::VIEWER_WASM.to_vec(),
        viewer_embed::VIEWER_JS.to_vec(),
    )
    .map_err(|error| (EXIT_RUNTIME_MISSING, error.to_string()))
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut digest = String::with_capacity(64);
    for byte in Sha256::digest(bytes) {
        use std::fmt::Write as _;
        let _ = write!(digest, "{byte:02x}");
    }
    digest
}

/// Auditable identity for release tooling: version, embed status, and the
/// digests of the runtime this binary seals into every figure-bearing
/// bundle. Canonical single-line JSON on stdout.
fn component_info() -> String {
    let mut info = serde_json::json!({
        "version": env!("CARGO_PKG_VERSION"),
        "viewer_runtime_embedded": viewer_embed::EMBEDDED,
    });
    if viewer_embed::EMBEDDED {
        info["viewer_wasm_sha256"] = sha256_hex(viewer_embed::VIEWER_WASM).into();
        info["viewer_wasm_bytes"] = viewer_embed::VIEWER_WASM.len().into();
        info["viewer_js_sha256"] = sha256_hex(viewer_embed::VIEWER_JS).into();
        info["viewer_js_bytes"] = viewer_embed::VIEWER_JS.len().into();
    }
    info.to_string()
}

fn render(args: &RenderArgs) -> Result<(), (u8, String)> {
    let viewer = embedded_runtime()?;
    let bytes = std::fs::read(&args.snapshot).map_err(|error| {
        (
            EXIT_IO,
            format!("cannot read {}: {error}", args.snapshot.display()),
        )
    })?;
    let snapshot = PublicationSnapshot::from_canonical_bytes(&bytes)
        .map_err(|error| (EXIT_CONTRACT, format!("snapshot rejected: {error}")))?;

    let digest = sha256_hex(&bytes);

    if args.out.exists() {
        return Err((
            EXIT_OUTPUT_EXISTS,
            format!("output {} already exists", args.out.display()),
        ));
    }
    let bundle = render_bundle(&snapshot, &digest, &viewer)
        .map_err(|error| (EXIT_CONTRACT, error.to_string()))?;

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
        Command::ComponentInfo => {
            println!("{}", component_info());
            ExitCode::SUCCESS
        }
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

    #[test]
    fn component_info_states_the_embed_truthfully() {
        let info: serde_json::Value =
            serde_json::from_str(&component_info()).expect("component info is JSON");
        assert_eq!(info["version"], env!("CARGO_PKG_VERSION"));
        assert_eq!(
            info["viewer_runtime_embedded"],
            viewer_embed::EMBEDDED,
            "the embed claim must match the build"
        );
        if viewer_embed::EMBEDDED {
            assert_eq!(info["viewer_wasm_sha256"].as_str().map(str::len), Some(64));
        } else {
            assert!(info.get("viewer_wasm_sha256").is_none());
            let (code, _) = embedded_runtime().expect_err("unembedded builds refuse to render");
            assert_eq!(code, EXIT_RUNTIME_MISSING);
        }
    }
}
