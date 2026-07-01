use std::env;
use std::path::PathBuf;

use rspice_veriloga::rust_backend::{
    regenerate_generated_builtins_with_progress, validate_generated_builtins,
};

type CommandResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Command {
    RegenerateBuiltins,
    CheckBuiltins,
}

#[derive(Debug)]
struct Options {
    command: Command,
    model_root: PathBuf,
    generated_root: PathBuf,
    generator_root: PathBuf,
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
            let report = regenerate_generated_builtins_with_progress(
                &options.model_root,
                &options.generated_root,
                &options.generator_root,
                true,
            )?;
            println!(
                "regenerated {} Verilog-A built-ins at {}",
                report.manifest.device_count,
                options.generated_root.display()
            );
            println!(
                "backend selection: scalar={}, scalar-hybrid={}, legacy-device={}",
                report.backend_counts.scalar,
                report.backend_counts.hybrid,
                report.backend_counts.legacy_device
            );
            println!(
                "manifest: source_tree_digest={}, generator_digest={}",
                report.manifest.source_tree_digest, report.manifest.generator_digest
            );
        }
        Command::CheckBuiltins => {
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
        "check-builtins" => Command::CheckBuiltins,
        other => return Err(format!("unknown command '{other}'\n\n{}", usage()).into()),
    };

    let workspace_root = workspace_root();
    let mut model_root = workspace_root.join("models/veriloga");
    let mut generated_root =
        workspace_root.join("crates/rspice-core/src/device/veriloga_generated");
    let mut generator_root = workspace_root.join("crates/rspice-veriloga");

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
    }))
}

fn path_arg(flag: &str, value: Option<String>) -> CommandResult<PathBuf> {
    value
        .map(PathBuf::from)
        .ok_or_else(|| format!("{flag} requires a path argument").into())
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
        "  cargo run -p rspice-veriloga --profile generator --bin rspice-veriloga-gen -- regenerate-builtins [--models PATH] [--out PATH]",
        "  cargo run -p rspice-veriloga --bin rspice-veriloga-gen -- check-builtins [--models PATH] [--out PATH]",
    ]
    .join("\n")
}
