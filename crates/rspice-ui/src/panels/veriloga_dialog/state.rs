use std::path::PathBuf;
use std::sync::{Arc, Mutex, mpsc};

use super::options::VerilogADialogOptions;
use super::types::{CompilationState, CompileErrorDisplay, CompileTaskResult, CompiledModuleInfo};

/// State for the Verilog-A model loading dialog.
pub struct VerilogALoadDialogState {
    /// Whether the dialog is open.
    pub open: bool,
    /// Selected file path.
    pub file_path: Option<PathBuf>,
    /// Path input text (for editing).
    pub file_path_text: String,
    /// Compiler options.
    pub options: VerilogADialogOptions,
    /// Compilation state.
    pub compilation_state: CompilationState,
    /// Error messages from last attempt.
    pub errors: Vec<CompileErrorDisplay>,
    /// Successfully compiled module info.
    pub compiled_module: Option<CompiledModuleInfo>,
    /// Successfully compiled model artifact for simulation cache registration.
    pub compiled_artifact: Option<rspice_veriloga::CompiledModel>,
    /// Canonical source/include dependencies captured during compilation.
    pub compiled_dependencies: Option<Vec<PathBuf>>,
    /// Whether to show advanced options.
    pub show_advanced_options: bool,
    /// egui clock (seconds) when the running compile started, for the
    /// elapsed readout while compiling.
    pub(super) compile_started_at: Option<f64>,
    /// Wall-clock seconds the last finished compile took.
    pub(super) last_compile_secs: Option<f64>,
    /// Verilog-A compile-cache totals (entry count, bytes) sampled when the
    /// dialog opens, for the idle footer hint.
    pub(super) cache_stats: Option<(usize, u64)>,
    /// Background compilation task receiver.
    pub(super) compile_task_receiver: Option<Arc<Mutex<mpsc::Receiver<CompileTaskResult>>>>,
}

impl Default for VerilogALoadDialogState {
    fn default() -> Self {
        Self {
            open: false,
            file_path: None,
            file_path_text: String::new(),
            options: VerilogADialogOptions::default(),
            compilation_state: CompilationState::Idle,
            errors: Vec::new(),
            compiled_module: None,
            compiled_artifact: None,
            compiled_dependencies: None,
            show_advanced_options: false,
            compile_started_at: None,
            last_compile_secs: None,
            cache_stats: None,
            compile_task_receiver: None,
        }
    }
}

impl Clone for VerilogALoadDialogState {
    fn clone(&self) -> Self {
        Self {
            open: self.open,
            file_path: self.file_path.clone(),
            file_path_text: self.file_path_text.clone(),
            options: self.options.clone(),
            compilation_state: self.compilation_state,
            errors: self.errors.clone(),
            compiled_module: self.compiled_module.clone(),
            compiled_artifact: self.compiled_artifact.clone(),
            compiled_dependencies: self.compiled_dependencies.clone(),
            show_advanced_options: self.show_advanced_options,
            compile_started_at: self.compile_started_at,
            last_compile_secs: self.last_compile_secs,
            cache_stats: self.cache_stats,
            compile_task_receiver: self.compile_task_receiver.clone(),
        }
    }
}

impl std::fmt::Debug for VerilogALoadDialogState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VerilogALoadDialogState")
            .field("open", &self.open)
            .field("file_path", &self.file_path)
            .field("file_path_text", &self.file_path_text)
            .field("compilation_state", &self.compilation_state)
            .field("errors", &self.errors.len())
            .field(
                "compiled_module",
                &self.compiled_module.as_ref().map(|m| &m.name),
            )
            .field(
                "compiled_artifact",
                &self.compiled_artifact.as_ref().map(|m| &m.name),
            )
            .field(
                "compiled_dependencies",
                &self.compiled_dependencies.as_ref().map(|d| d.len()),
            )
            .field("show_advanced_options", &self.show_advanced_options)
            .field("compile_started_at", &self.compile_started_at)
            .field("last_compile_secs", &self.last_compile_secs)
            .field("cache_stats", &self.cache_stats)
            .field(
                "compile_task_receiver",
                &self.compile_task_receiver.is_some(),
            )
            .finish()
    }
}

impl VerilogALoadDialogState {
    /// Create a new default state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Reset the dialog to initial state.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Open the dialog (clears previous state).
    pub fn open(&mut self) {
        self.reset();
        self.open = true;
        // Sample the simulator's Verilog-A compile cache once for the idle
        // footer hint; unavailable (e.g. browser build) simply omits it.
        self.cache_stats = rspice_core::veriloga_cache_stats()
            .ok()
            .map(|stats| (stats.entry_count, stats.total_bytes));
    }

    /// Close the dialog.
    pub fn close(&mut self) {
        self.open = false;
        self.compilation_state = CompilationState::Idle;
    }

    /// Set the file path from a PathBuf.
    pub fn set_file_path(&mut self, path: PathBuf) {
        self.file_path_text = path.to_string_lossy().to_string();
        self.file_path = Some(path);
        self.errors.clear();
        self.compiled_module = None;
        self.compiled_artifact = None;
        self.compiled_dependencies = None;
        self.compile_started_at = None;
        self.last_compile_secs = None;
        self.compilation_state = CompilationState::Idle;
    }

    /// Check if we can start compilation.
    pub fn can_compile(&self) -> bool {
        self.file_path.is_some()
            && matches!(
                self.compilation_state,
                CompilationState::Idle | CompilationState::Failed
            )
    }

    /// Check if compilation succeeded.
    pub fn is_success(&self) -> bool {
        matches!(self.compilation_state, CompilationState::Success)
    }
}
