use std::path::PathBuf;
use std::sync::{Arc, Mutex, mpsc};

use super::state::VerilogALoadDialogState;
use super::types::{
    CompilationState, CompileErrorDisplay, CompileTaskResult, CompiledModuleInfo, ParameterInfo,
};

/// What a compile run reads its module from. Native builds compile the
/// picked file (preprocessing resolves `include against the disk); the
/// browser build compiles the pasted buffer.
enum CompileInput {
    #[cfg(not(target_arch = "wasm32"))]
    File(PathBuf),
    #[cfg(target_arch = "wasm32")]
    Source(String),
}

struct CompileArtifacts {
    model: rspice_veriloga::CompiledModel,
    canonical_ir: Option<rspice_veriloga::canonical_ir::CanonicalIrArtifact>,
    dependencies: Vec<PathBuf>,
}

/// Start async compilation using rspice-veriloga.
pub(super) fn start_compile(state: &mut VerilogALoadDialogState) {
    #[cfg(not(target_arch = "wasm32"))]
    let input = {
        let path = match &state.file_path {
            Some(p) => p.clone(),
            None => {
                state.errors = vec![CompileErrorDisplay::error("No file selected")];
                state.compilation_state = CompilationState::Failed;
                return;
            }
        };

        if !path.exists() {
            state.errors = vec![CompileErrorDisplay::error("File not found")];
            state.compilation_state = CompilationState::Failed;
            return;
        }

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        if ext != "va" && ext != "vams" {
            state.errors = vec![CompileErrorDisplay::error(
                "Invalid file extension. Expected .va or .vams",
            )];
            state.compilation_state = CompilationState::Failed;
            return;
        }
        CompileInput::File(path)
    };

    #[cfg(target_arch = "wasm32")]
    let input = {
        if state.source_text.trim().is_empty() {
            state.errors = vec![CompileErrorDisplay::error(
                "No source — paste the Verilog-A module text",
            )];
            state.compilation_state = CompilationState::Failed;
            return;
        }
        CompileInput::Source(state.source_text.clone())
    };

    let options = rspice_veriloga::CompilerOptions {
        enable_ams: state.options.enable_ams,
        include_paths: state.options.include_paths.clone(),
        defines: state
            .options
            .defines
            .iter()
            .map(|(n, v)| (n.clone(), if v.is_empty() { None } else { Some(v.clone()) }))
            .collect(),
        strict_mode: state.options.strict_mode,
        ..Default::default()
    };

    let (tx, rx) = mpsc::channel();

    crate::common::spawn_or_inline(move || {
        let compiler = rspice_veriloga::VerilogACompiler::new(options);
        let (result, source_path) = match input {
            #[cfg(not(target_arch = "wasm32"))]
            CompileInput::File(path) => {
                log::info!("Starting Verilog-A compilation: {}", path.display());
                (
                    compiler
                        .compile_file_runtime_with_metadata(&path, None)
                        .map(|compiled| CompileArtifacts {
                            model: compiled.model,
                            canonical_ir: Some(compiled.canonical_ir),
                            dependencies: compiled.dependencies,
                        }),
                    path,
                )
            }
            #[cfg(target_arch = "wasm32")]
            CompileInput::Source(text) => {
                log::info!(
                    "Starting Verilog-A compilation from pasted source ({} bytes)",
                    text.len()
                );
                // Compiling from memory has no on-disk dependencies; the
                // synthetic va:// path is minted from the module name once
                // it is known.
                let result = compiler.compile(&text).map(|model| CompileArtifacts {
                    model,
                    canonical_ir: None,
                    dependencies: Vec::new(),
                });
                (result, PathBuf::new())
            }
        };

        let task_result = match result {
            Ok(compiled) => {
                let model = compiled.model;
                let canonical_ir = compiled.canonical_ir;
                log::info!("Verilog-A compilation succeeded: module '{}'", model.name);
                let source_path = if source_path.as_os_str().is_empty() {
                    PathBuf::from(format!("va://{}.va", model.name))
                } else {
                    source_path
                };
                CompileTaskResult::Success {
                    module_info: CompiledModuleInfo {
                        name: model.name.to_string(),
                        ports: model.terminal_names.iter().map(|s| s.to_string()).collect(),
                        parameters: model
                            .parameters
                            .iter()
                            .map(|p| ParameterInfo {
                                name: p.name.to_string(),
                                default_value: format!("{}", p.default),
                                min: p.min,
                                max: p.max,
                                description: None,
                            })
                            .collect(),
                        source_path,
                        internal_nodes: model.internal_nodes,
                        num_variables: model.num_variables,
                    },
                    compiled_model: Box::new(model),
                    canonical_ir: canonical_ir.map(Box::new),
                    dependencies: compiled.dependencies,
                }
            }
            Err(e) => {
                log::error!("Verilog-A compilation failed: {}", e);
                CompileTaskResult::Failure(vec![CompileErrorDisplay::error(e.to_string())])
            }
        };

        let _ = tx.send(task_result);
    });

    state.compile_task_receiver = Some(Arc::new(Mutex::new(rx)));
    state.compilation_state = CompilationState::Compiling;
    state.errors.clear();
    state.compiled_module = None;
    state.compiled_artifact = None;
    state.compiled_canonical_ir = None;
    state.compiled_dependencies = None;
}

/// Poll for compilation result (non-blocking).
pub(super) fn poll_compile(state: &mut VerilogALoadDialogState) {
    if !matches!(state.compilation_state, CompilationState::Compiling) {
        return;
    }

    let receiver = match &state.compile_task_receiver {
        Some(rx) => rx.clone(),
        None => return,
    };

    let received = if let Ok(guard) = receiver.try_lock() {
        match guard.try_recv() {
            Ok(result) => Some(result),
            Err(mpsc::TryRecvError::Empty) => None,
            Err(mpsc::TryRecvError::Disconnected) => Some(CompileTaskResult::Failure(vec![
                CompileErrorDisplay::error("Compilation thread disconnected unexpectedly"),
            ])),
        }
    } else {
        None
    };

    if let Some(task_result) = received {
        match task_result {
            CompileTaskResult::Success {
                module_info,
                compiled_model,
                canonical_ir,
                dependencies,
            } => {
                state.compiled_module = Some(module_info);
                state.compiled_artifact = Some(*compiled_model);
                state.compiled_canonical_ir = canonical_ir.map(|artifact| *artifact);
                state.compiled_dependencies = Some(dependencies);
                state.compilation_state = CompilationState::Success;
            }
            CompileTaskResult::Failure(errors) => {
                state.errors = errors;
                state.compiled_artifact = None;
                state.compiled_canonical_ir = None;
                state.compiled_dependencies = None;
                state.compilation_state = CompilationState::Failed;
            }
        }
        state.compile_task_receiver = None;
    }
}
