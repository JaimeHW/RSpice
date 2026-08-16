//! Explicit maintenance command for ISCAS85 and Paranoia ngspice oracles.

use rspice_conformance::suites::execution::{
    ExecutionCorpus, capture_ngspice_oracles,
};
use std::path::PathBuf;

fn main() -> Result<(), String> {
    let mut corpus = None;
    let mut ngspice_exe = None;
    let mut case = None;
    let mut timeout_ms = 900_000_u128;
    let mut update = false;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--corpus" => {
                let value = args.next().ok_or("--corpus requires a value")?;
                corpus = Some(match value.as_str() {
                    "iscas85" => ExecutionCorpus::Iscas85,
                    "paranoia" => ExecutionCorpus::Paranoia,
                    _ => return Err(format!("unknown corpus '{value}'")),
                });
            }
            "--ngspice-exe" => {
                ngspice_exe = Some(PathBuf::from(
                    args.next().ok_or("--ngspice-exe requires a path")?,
                ));
            }
            "--case" => case = Some(args.next().ok_or("--case requires a path")?),
            "--timeout-ms" => {
                timeout_ms = args
                    .next()
                    .ok_or("--timeout-ms requires a value")?
                    .parse()
                    .map_err(|err| format!("invalid --timeout-ms: {err}"))?;
            }
            "--update" => update = true,
            "--help" | "-h" => {
                println!(
                    "Usage: rspice-ngspice-oracle-capture --corpus <iscas85|paranoia> \
                     --ngspice-exe <path> [--case <relative-path>] [--timeout-ms N] [--update]"
                );
                return Ok(());
            }
            _ => return Err(format!("unknown argument '{arg}'")),
        }
    }

    let corpus = corpus.ok_or("missing --corpus")?;
    let ngspice_exe = ngspice_exe.ok_or("missing --ngspice-exe")?;
    let tests_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .ok_or("cannot resolve workspace tests directory")?
        .join("tests");
    let stats = capture_ngspice_oracles(
        corpus,
        &tests_dir,
        &ngspice_exe,
        case.as_deref(),
        timeout_ms,
        update,
    )?;
    println!(
        "{} oracle capture: {} discovered, {} standalone, {} written, {} verified, \
         {} execution-only, {} failed",
        corpus.label(),
        stats.discovered,
        stats.eligible,
        stats.written,
        stats.verified,
        stats.without_outputs,
        stats.failed()
    );
    if !stats.failures.is_empty() {
        for failure in &stats.failures {
            eprintln!("  {failure}");
        }
        return Err(format!("{} oracle capture failure(s)", stats.failed()));
    }
    Ok(())
}
