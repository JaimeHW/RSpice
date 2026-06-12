//! Compile-VA Command - Compile Verilog-A models
//!
//! Compiles Verilog-A model files and displays model information.

use crate::cli::{CliError, CompileVaArgs, Config};
use rspice_veriloga::{CompilerOptions, VerilogACompiler};

/// Execute the compile-va command
pub fn execute(
    args: CompileVaArgs,
    config: &Config,
    verbose: bool,
    _quiet: bool,
) -> Result<(), CliError> {
    // Validate input file exists
    if !args.input.exists() {
        return Err(CliError::InputNotFound {
            path: args.input.clone(),
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"),
        });
    }

    // Configure compiler options
    let mut options = CompilerOptions {
        strict_mode: args.strict,
        ..CompilerOptions::default()
    };

    // Add include paths (CLI flags first, then config paths.veriloga_includes)
    for include_dir in &args.includes {
        options.include_paths.push(include_dir.clone());
    }
    for include_dir in &config.paths.veriloga_includes {
        options.include_paths.push(include_dir.clone());
    }

    // Add the source file's directory as an include path
    if let Some(parent) = args.input.parent() {
        options.include_paths.push(parent.to_path_buf());
    }

    if verbose {
        println!("Verilog-A Compiler Options:");
        println!("  Strict mode: {}", options.strict_mode);
        println!("  Include paths: {:?}", options.include_paths);
        println!();
    }

    // Compile the model
    println!("Compiling: {}", args.input.display());
    let compiler = VerilogACompiler::new(options);
    let model = compiler
        .compile_file(&args.input)
        .map_err(|e| CliError::VerilogAError {
            message: e.to_string(),
        })?;

    // Display model information
    println!();
    println!("====================================================================");
    println!("Verilog-A Model: {}", model.name);
    println!("====================================================================");
    println!();

    // Terminals
    println!("Terminals ({}):", model.num_terminals);
    for (i, name) in model.terminal_names.iter().enumerate() {
        println!("  [{:2}] {}", i, name);
    }
    println!();

    // Internal nodes
    if model.internal_nodes > 0 {
        println!("Internal Nodes: {}", model.internal_nodes);
        println!();
    }

    // Parameters
    if !model.parameters.is_empty() {
        println!("Parameters ({}):", model.parameters.len());
        println!(
            "  {:<20} {:>15} {:>12} {:>12}",
            "Name", "Default", "Min", "Max"
        );
        println!("  {:-<20} {:-^15} {:-^12} {:-^12}", "", "", "", "");

        for param in &model.parameters {
            let min_str = param.min.map_or("-".to_string(), |v| format!("{:.4e}", v));
            let max_str = param.max.map_or("-".to_string(), |v| format!("{:.4e}", v));
            println!(
                "  {:<20} {:>15.6e} {:>12} {:>12}",
                param.name, param.default, min_str, max_str
            );
        }
        println!();
    }

    // Branch equations (stamp programs)
    if args.detailed {
        println!("Branch Equations: {}", model.stamp_programs.len());
        for (i, program) in model.stamp_programs.iter().enumerate() {
            println!(
                "  [{}] {} stamp locations, {} jacobian entries",
                i,
                program.stamp_locations.len(),
                program.jacobian_programs.len()
            );
        }
        println!();
    }

    // Summary
    println!("Compilation successful");

    // Machine-readable interface summary
    if let Some(ref output_path) = args.output {
        let json = serde_json::json!({
            "source": args.input.display().to_string(),
            "model": model.name,
            "terminals": model.terminal_names,
            "internal_nodes": model.internal_nodes,
            "parameters": model.parameters.iter().map(|p| {
                serde_json::json!({
                    "name": p.name,
                    "default": p.default,
                    "min": p.min,
                    "max": p.max,
                })
            }).collect::<Vec<_>>(),
        });
        let text = serde_json::to_string_pretty(&json)
            .map_err(|e| CliError::output_json_error(output_path, e))?;
        std::fs::write(output_path, text + "\n")
            .map_err(|e| CliError::output_error(output_path, e))?;
        println!("Interface summary written to: {}", output_path.display());
    }

    // Usage example
    if args.show_usage {
        println!();
        println!("Usage in SPICE netlist:");
        println!(
            "  .VERILOGA {}",
            args.input
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_else(|| args.input.display().to_string())
        );

        // Generate example device instantiation
        let terminal_list: String = model
            .terminal_names
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join(" ");
        println!("  X1 {} {}", terminal_list, model.name);

        if !model.parameters.is_empty() {
            let param_example = model
                .parameters
                .first()
                .map(|p| format!(".MODEL {} VERILOGA({}={})", model.name, p.name, p.default))
                .unwrap_or_default();
            println!("  {}", param_example);
        }
    }

    Ok(())
}
