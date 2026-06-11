use std::sync::{Arc, Mutex, mpsc};

use super::state::VerilogALoadDialogState;
use super::types::{
    CompilationState, CompileErrorDisplay, CompileTaskResult, CompiledModuleInfo, ParameterInfo,
};

/// Start async compilation using rspice-veriloga.
pub(super) fn start_compile(state: &mut VerilogALoadDialogState) {
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
    let source_path = path.clone();

    crate::common::spawn_or_inline(move || {
        log::info!("Starting Verilog-A compilation: {}", source_path.display());

        let compiler = rspice_veriloga::VerilogACompiler::new(options);
        let result = compiler.compile_file_with_metadata(&source_path);

        let task_result = match result {
            Ok(compiled) => {
                let model = compiled.model;
                log::info!("Verilog-A compilation succeeded: module '{}'", model.name);
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
                dependencies,
            } => {
                state.compiled_module = Some(module_info);
                state.compiled_artifact = Some(*compiled_model);
                state.compiled_dependencies = Some(dependencies);
                state.compilation_state = CompilationState::Success;
            }
            CompileTaskResult::Failure(errors) => {
                state.errors = errors;
                state.compiled_artifact = None;
                state.compiled_dependencies = None;
                state.compilation_state = CompilationState::Failed;
            }
        }
        state.compile_task_receiver = None;
    }
}
