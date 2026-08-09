//! Generator CLI for the built-in Verilog-A devices.
//!
//! Drives [`rspice_veriloga::rust_backend`] over a tree of Verilog-A sources
//! and writes the Rust that `rspice-core` compiles under its
//! `veriloga-builtins` feature.
//!
//! ```text
//! regenerate-builtins       rewrite every device, registry.rs and manifest.txt
//! generate-builtins-subset  --filter one model into target/, for inspection
//! check-builtins            verify the checked-in output is current; no writes
//! ```
//!
//! Build it under `--profile generator`: the model corpus is large enough that
//! an unoptimized generator is impractically slow.
//!
//! `--filter` is accepted only by the subset command. A partial rewrite would
//! leave `registry.rs` and the manifest describing devices that are no longer
//! there, so full regeneration rejects the flag rather than producing an
//! inconsistent tree, and the subset command writes neither file.

use std::env;
use std::path::PathBuf;

use rspice_veriloga::rust_backend::{
    common_kernel_coverage, generate_generated_builtin_subset_with_progress_and_jobs,
    regenerate_generated_builtins_with_progress_and_jobs, validate_generated_builtins,
};

type CommandResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Command {
    RegenerateBuiltins,
    GenerateBuiltinsSubset,
    CheckBuiltins,
}

#[derive(Debug)]
struct Options {
    command: Command,
    model_root: PathBuf,
    generated_root: PathBuf,
    generator_root: PathBuf,
    filter: Option<String>,
    jobs: Option<usize>,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn run() -> CommandResult<()> {
    let Some(options) = parse_args(env::args().skip(1))? else {
        println!("{}", usage());
        return Ok(());
    };
    match options.command {
        Command::RegenerateBuiltins => {
            if options.filter.is_some() {
                return Err(
                    "--filter is only supported by generate-builtins-subset; full regeneration must update every generated built-in"
                        .into(),
                );
            }
            let report = regenerate_generated_builtins_with_progress_and_jobs(
                &options.model_root,
                &options.generated_root,
                &options.generator_root,
                true,
                options.jobs,
            )?;
            println!(
                "regenerated {} Verilog-A built-ins at {}",
                report.manifest.device_count,
                options.generated_root.display()
            );
            println!(
                "manifest: source_tree_digest={}, generator_digest={}",
                report.manifest.source_tree_digest, report.manifest.generator_digest
            );
        }
        Command::GenerateBuiltinsSubset => {
            let Some(filter) = options.filter.as_deref() else {
                return Err("generate-builtins-subset requires --filter FILTER".into());
            };
            let report = generate_generated_builtin_subset_with_progress_and_jobs(
                &options.model_root,
                &options.generated_root,
                filter,
                true,
                options.jobs,
            )?;
            println!(
                "generated {} filtered Verilog-A built-ins at {}",
                report.device_count,
                options.generated_root.display()
            );
            let coverage = common_kernel_coverage(&report.kernel_structures);
            println!(
                "kernel-structure common-newton-instructions={}",
                coverage.common_newton_instruction_count
            );
            for (model, instructions, percent) in coverage.models {
                println!(
                    "kernel-structure model={model} newton-instructions={instructions} common-coverage={percent:.2}%"
                );
            }
            let noise_census =
                report
                    .kernel_structures
                    .iter()
                    .fold([0u64; 5], |mut total, report| {
                        for (total, count) in
                            total.iter_mut().zip(report.noise_invalidation_value_count)
                        {
                            *total = total.saturating_add(count);
                        }
                        total
                    });
            println!(
                "noise-structure values={} model={} instance={} temperature={} timestep={} newton={}",
                noise_census.iter().sum::<u64>(),
                noise_census[0],
                noise_census[1],
                noise_census[2],
                noise_census[3],
                noise_census[4],
            );
            let shared_preprocess = report
                .kernel_structures
                .iter()
                .map(|report| report.noise_shared_preprocess_value_count)
                .sum::<u64>();
            println!("noise-structure exact-shared-preprocess-values={shared_preprocess}");
            println!("[partial output: registry.rs and manifest.txt were not rewritten]");
        }
        Command::CheckBuiltins => {
            if options.filter.is_some() {
                return Err("--filter is not supported by check-builtins".into());
            }
            if options.jobs.is_some() {
                return Err("--jobs is not supported by check-builtins".into());
            }
            let manifest = validate_generated_builtins(
                &options.model_root,
                &options.generated_root,
                &options.generator_root,
                false,
            )?;
            println!(
                "generated Verilog-A built-ins are current: device_count={}, source_tree_digest={}, generator_digest={}",
                manifest.device_count, manifest.source_tree_digest, manifest.generator_digest
            );
        }
    }
    Ok(())
}

fn parse_args(args: impl IntoIterator<Item = String>) -> CommandResult<Option<Options>> {
    let mut args = args.into_iter();
    let Some(command) = args.next() else {
        return Err(usage().into());
    };
    if command == "-h" || command == "--help" {
        return Ok(None);
    }
    let command = match command.as_str() {
        "regenerate-builtins" => Command::RegenerateBuiltins,
        "generate-builtins-subset" => Command::GenerateBuiltinsSubset,
        "check-builtins" => Command::CheckBuiltins,
        other => return Err(format!("unknown command '{other}'\n\n{}", usage()).into()),
    };

    let workspace_root = workspace_root();
    let mut model_root = workspace_root.join("models/veriloga");
    let mut generated_root = match command {
        Command::GenerateBuiltinsSubset => workspace_root.join("target/veriloga-generated-subset"),
        Command::RegenerateBuiltins | Command::CheckBuiltins => {
            workspace_root.join("crates/rspice-veriloga-models")
        }
    };
    let mut generator_root = workspace_root.join("crates/rspice-veriloga");
    let mut filter = None;
    let mut jobs = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--models" => {
                model_root = path_arg("--models", args.next())?;
            }
            "--out" => {
                generated_root = path_arg("--out", args.next())?;
            }
            "--generator-root" => {
                generator_root = path_arg("--generator-root", args.next())?;
            }
            "--filter" => {
                filter = Some(string_arg("--filter", args.next())?);
            }
            "--jobs" => {
                jobs = Some(usize_arg("--jobs", args.next())?);
            }
            "-h" | "--help" => {
                return Ok(None);
            }
            other => return Err(format!("unknown argument '{other}'\n\n{}", usage()).into()),
        }
    }

    Ok(Some(Options {
        command,
        model_root,
        generated_root,
        generator_root,
        filter,
        jobs,
    }))
}

fn path_arg(flag: &str, value: Option<String>) -> CommandResult<PathBuf> {
    value
        .map(PathBuf::from)
        .ok_or_else(|| format!("{flag} requires a path argument").into())
}

fn string_arg(flag: &str, value: Option<String>) -> CommandResult<String> {
    value.ok_or_else(|| format!("{flag} requires an argument").into())
}

fn usize_arg(flag: &str, value: Option<String>) -> CommandResult<usize> {
    let value = string_arg(flag, value)?;
    value
        .parse::<usize>()
        .map_err(|error| format!("{flag} requires a positive integer: {error}").into())
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("rspice-veriloga must live under workspace crates directory")
        .to_path_buf()
}

fn usage() -> String {
    [
        "usage:",
        "  cargo run -p rspice-veriloga --profile generator --bin rspice-veriloga-gen -- regenerate-builtins [--models PATH] [--out PATH] [--jobs N]",
        "  cargo run -p rspice-veriloga --profile generator --bin rspice-veriloga-gen -- generate-builtins-subset --filter FILTER [--models PATH] [--out PATH] [--jobs N]",
        "  cargo run -p rspice-veriloga --bin rspice-veriloga-gen -- check-builtins [--models PATH] [--out PATH]",
    ]
    .join("\n")
}
