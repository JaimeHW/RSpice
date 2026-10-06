//! Compile-VA Command - Compile Verilog-A models
//!
//! Compiles Verilog-A model files and displays model information.

use crate::cli::{CliError, CompileVaArgs, Config, map_atomic_output_error};
use crate::commands::publish;
use rspice_veriloga::{CompilerOptions, VerilogACompiler};

/// Execute the compile-va command
pub fn execute(
    args: CompileVaArgs,
    config: &Config,
    verbose: bool,
    quiet: bool,
) -> Result<(), CliError> {
    if args.strict {
        return Err(rspice_core::SimulationError::unsupported_capability(
            "veriloga.strict_lrm", "strict LRM compliance checking is not implemented; omit --strict to use the supported compiler checks",
        ).into());
    }
    crate::abort::install_interrupt_handler();
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

    if verbose && !quiet {
        println!("Verilog-A Compiler Options:");
        println!("  Strict mode: {}", options.strict_mode);
        println!("  Include paths: {:?}", options.include_paths);
        println!();
    }

    // Compile the model
    if !quiet {
        println!("Compiling: {}", args.input.display());
    }
    let compiler = VerilogACompiler::new(options);
    let limits = config.resources.limits();
    let provider = rspice_veriloga::preprocessor::BoundedFileSystemSourceProvider::new(
        rspice_veriloga::SourceProviderLimits {
            max_dependencies: usize::MAX,
            max_total_source_bytes: limits.max_dependency_source_bytes,
            max_include_depth: limits.max_include_depth,
            max_expanded_bytes: limits.max_expanded_source_bytes,
        },
        limits.max_netlist_bytes,
        limits.max_netlist_lines,
        &ProcessControl,
    );
    let compiled = compiler
        .compile_provider_module_with_metadata_and_control(
            &provider,
            &args.input,
            args.module.as_deref(),
            &ProcessControl,
        )
        .map_err(compile_error)?;
    if let Some(output) = args.output.as_deref() {
        publish::destinations::protect_sources(
            output,
            std::iter::once(args.input.as_path()).chain(
                compiled
                    .dependencies
                    .iter()
                    .map(std::path::PathBuf::as_path),
            ),
        )?;
    }
    let model = compiled.model;

    // Quiet suppresses all text; an explicitly requested file still publishes.
    if !quiet {
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
    }

    // Machine-readable interface summary
    if let Some(ref output_path) = args.output {
        let json = crate::observability::envelope(
            "rspice.compile_va",
            serde_json::json!({
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
            }),
        );
        let text = serde_json::to_string_pretty(&json)
            .map_err(|e| CliError::output_json_error(output_path, e))?;
        let document = text + "\n";
        publish::artifact(output_path, |writer| {
            writer
                .write_all(document.as_bytes())
                .map_err(|error| CliError::output_error(output_path, error))
        })
        .map_err(|error| map_atomic_output_error(output_path, error))?;
        if !quiet {
            println!("Interface summary written to: {}", output_path.display());
        }
    }

    // Usage example
    //
    // A Verilog-A module is declared by the source directive and instantiated
    // on an X card that names the module; there is no `.MODEL name VERILOGA`
    // card in RSpice, and no device family binds a model of that type, so the
    // example writes instance parameters where the parser reads them.
    if args.show_usage && !quiet {
        let source = args
            .input
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| args.input.display().to_string());
        let terminal_list: String = model
            .terminal_names
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join(" ");
        let instance_parameters = model
            .parameters
            .first()
            .map(|parameter| format!(" {}={}", parameter.name, parameter.default))
            .unwrap_or_default();

        println!();
        println!("Usage in SPICE netlist:");
        println!("  .va \"{}\"", source);
        println!(
            "  X1 {} {}{}",
            terminal_list, model.name, instance_parameters
        );
        println!();
        println!(
            "The .va card compiles the source and the X card instantiates the module by name, \
             carrying any instance parameters."
        );
    }

    Ok(())
}

struct ProcessControl;
impl rspice_veriloga::PipelineControl for ProcessControl {
    fn is_cancelled(&self) -> bool {
        crate::abort::reason().is_some()
    }
}

fn compile_error(error: rspice_veriloga::ProviderCompileError) -> CliError {
    use rspice_veriloga::preprocessor::SourceResource;
    use rspice_veriloga::{CompileError, ProviderCompileError};
    match error {
        ProviderCompileError::Compile(CompileError::Cancelled(_)) => CliError::Interrupted,
        ProviderCompileError::Source(error) if error.cancelled => CliError::Interrupted,
        ProviderCompileError::Source(error) if error.resource_limit.is_some() => {
            let limit = error.resource_limit.expect("matched resource failure");
            let resource = match limit.resource {
                SourceResource::RootBytes => rspice_core::ResourceKind::NetlistBytes,
                SourceResource::RootLines => rspice_core::ResourceKind::NetlistLines,
                SourceResource::TotalSourceBytes | SourceResource::Dependencies => {
                    rspice_core::ResourceKind::DependencySourceBytes
                }
                SourceResource::IncludeDepth => rspice_core::ResourceKind::IncludeDepth,
                SourceResource::ExpandedBytes => rspice_core::ResourceKind::ExpandedSourceBytes,
            };
            rspice_core::SimulationError::ResourceLimit(rspice_core::ResourceLimitError {
                resource,
                requested: limit.requested,
                limit: limit.limit,
            })
            .into()
        }
        error => CliError::VerilogAError {
            message: error.to_string(),
        },
    }
}
