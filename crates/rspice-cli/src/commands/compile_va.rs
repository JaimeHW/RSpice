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
    if let Some(output) = args.output.as_deref() {
        publish::destinations::protect_sources(output, config.source_paths())?;
    }
    let usage_source = (args.show_usage && !quiet)
        .then(|| usage_source_path(&args.input))
        .transpose()?;
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
        crate::console::line(format_args!("Verilog-A Compiler Options:"))?;
        crate::console::line(format_args!("  Strict mode: {}", options.strict_mode))?;
        crate::console::line(format_args!("  Include paths: {:?}", options.include_paths))?;
        crate::console::line(format_args!(""))?;
    }

    // Compile the model
    if !quiet {
        crate::console::line(format_args!("Compiling: {}", args.input.display()))?;
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
        .map_err(|error| compile_error(error, &args.input))?;
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
        for diagnostic in &compiled.diagnostics {
            crate::observability::compiler_diagnostic(diagnostic);
        }
        // Display model information
        crate::console::line(format_args!(""))?;
        crate::console::line(format_args!(
            "===================================================================="
        ))?;
        crate::console::line(format_args!("Verilog-A Model: {}", model.name))?;
        crate::console::line(format_args!(
            "===================================================================="
        ))?;
        crate::console::line(format_args!(""))?;

        // Terminals
        crate::console::line(format_args!("Terminals ({}):", model.num_terminals))?;
        for (i, name) in model.terminal_names.iter().enumerate() {
            crate::console::line(format_args!("  [{:2}] {}", i, name))?;
        }
        crate::console::line(format_args!(""))?;

        // Internal nodes
        if model.internal_nodes > 0 {
            crate::console::line(format_args!("Internal Nodes: {}", model.internal_nodes))?;
            crate::console::line(format_args!(""))?;
        }

        // Parameters
        if !model.parameters.is_empty() {
            crate::console::line(format_args!("Parameters ({}):", model.parameters.len()))?;
            crate::console::line(format_args!(
                "  {:<20} {:>15} {:>12} {:>12}",
                "Name", "Default", "Min", "Max"
            ))?;
            crate::console::line(format_args!(
                "  {:-<20} {:-^15} {:-^12} {:-^12}",
                "", "", "", ""
            ))?;

            for param in &model.parameters {
                let min_str = param.min.map_or("-".to_string(), |v| format!("{:.4e}", v));
                let max_str = param.max.map_or("-".to_string(), |v| format!("{:.4e}", v));
                crate::console::line(format_args!(
                    "  {:<20} {:>15.6e} {:>12} {:>12}",
                    param.name, param.default, min_str, max_str
                ))?;
            }
            crate::console::line(format_args!(""))?;
        }

        // Branch equations (stamp programs)
        if args.detailed {
            crate::console::line(format_args!(
                "Branch Equations: {}",
                model.stamp_programs.len()
            ))?;
            for (i, program) in model.stamp_programs.iter().enumerate() {
                crate::console::line(format_args!(
                    "  [{}] {} stamp locations, {} jacobian entries",
                    i,
                    program.stamp_locations.len(),
                    program.jacobian_programs.len()
                ))?;
            }
            crate::console::line(format_args!(""))?;
        }

        // Summary
        crate::console::line(format_args!("Compilation successful"))?;
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
                "diagnostics": compiled.diagnostics,
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
            crate::console::line(format_args!(
                "Interface summary written to: {}",
                output_path.display()
            ))?;
        }
    }

    // Usage example
    //
    // A Verilog-A module is declared by the source directive and instantiated
    // on an X card that names the module; there is no `.MODEL name VERILOGA`
    // card in RSpice, and no device family binds a model of that type, so the
    // example writes instance parameters where the parser reads them.
    if let Some(source) = usage_source {
        let terminal_list = model.terminal_names.join(" ");
        let instance_parameters = model
            .parameters
            .first()
            .map(|parameter| format!(" {}={}", parameter.name, parameter.default))
            .unwrap_or_default();

        crate::console::line(format_args!(""))?;
        crate::console::line(format_args!("Usage in SPICE netlist:"))?;
        crate::console::line(format_args!(
            "  .va {} module={}",
            source,
            quote_netlist_token(&model.name)
        ))?;
        crate::console::line(format_args!(
            "  X1 {} {}{}",
            terminal_list, model.name, instance_parameters
        ))?;
        crate::console::line(format_args!(""))?;
        crate::console::line(format_args!(
            "The .va card compiles the source and the X card instantiates the module by name, \
             carrying any instance parameters."
        ))?;
    }

    Ok(())
}

fn usage_source_path(input: &std::path::Path) -> Result<String, CliError> {
    let absolute = std::path::absolute(input).map_err(|source| CliError::InputReadError {
        path: input.to_path_buf(),
        source,
    })?;
    let source = absolute
        .to_str()
        .filter(|source| !source.contains(['\r', '\n']))
        .ok_or_else(|| CliError::InvalidArgument {
            message: "--show-usage requires a UTF-8 source path without line breaks".into(),
            suggestion: Some("rename the source file or omit --show-usage".into()),
        })?;
    Ok(quote_netlist_token(source))
}

// The netlist source-directive grammar treats backslashes as escapes inside
// quotes, including Windows separators. Preserve the literal path and name.
fn quote_netlist_token(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}

struct ProcessControl;
impl rspice_veriloga::PipelineControl for ProcessControl {
    fn is_cancelled(&self) -> bool {
        crate::abort::reason().is_some()
    }
}

fn compile_error(
    error: rspice_veriloga::ProviderCompileError,
    input: &std::path::Path,
) -> CliError {
    use rspice_veriloga::preprocessor::SourceResource;
    use rspice_veriloga::{CompileError, ProviderCompileError};
    match error {
        ProviderCompileError::Compile {
            source: CompileError::Cancelled(_),
            ..
        } => CliError::Interrupted,
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
            CliError::ResourceLimit {
                path: error.file.unwrap_or_else(|| input.to_path_buf()),
                source: rspice_core::ResourceLimitError {
                    resource,
                    requested: limit.requested,
                    limit: limit.limit,
                },
            }
        }
        ProviderCompileError::Source(rspice_veriloga::PreprocessorError {
            io_error: Some(source),
            file,
            ..
        }) => {
            let path = file.unwrap_or_else(|| input.to_path_buf());
            let source = std::sync::Arc::try_unwrap(source)
                .unwrap_or_else(|source| std::io::Error::new(source.kind(), source));
            if source.kind() == std::io::ErrorKind::NotFound {
                CliError::InputNotFound { path, source }
            } else {
                CliError::InputReadError { path, source }
            }
        }
        ProviderCompileError::Source(source) => {
            let diagnostic = rspice_veriloga::SourceCompileDiagnostic::from(&source);
            let error = CliError::VerilogAError {
                message: source.message,
            };
            let mut details = error.details();
            details.path = source.file.map(|path| path.display().to_string());
            details.line = (source.line > 0).then_some(source.line);
            details.diagnostics.push(diagnostic);
            CliError::reported(error.to_string(), Some(details))
        }
        ProviderCompileError::Compile {
            source,
            diagnostics,
        } => {
            let error = CliError::VerilogAError {
                message: if diagnostics.is_empty() {
                    source.to_string()
                } else {
                    diagnostics
                        .iter()
                        .map(|diagnostic| diagnostic.message.as_str())
                        .collect::<Vec<_>>()
                        .join("\n")
                },
            };
            let mut details = error.details();
            if let Some(primary) = diagnostics.first() {
                details.path = primary.path.clone();
                details.line = primary.line;
            }
            details.diagnostics = diagnostics;
            CliError::reported(error.to_string(), Some(details))
        }
    }
}
