//! Verilog-A Model Loading Dialog
//!
//! Professional interface for loading, compiling, and registering Verilog-A
//! compact device models in the component library.
//!
//! ## Features
//!
//! - File browser for `.va` files
//! - Compiler options configuration (include paths, defines)
//! - Background compilation with progress indicator
//! - Error display with line numbers
//! - Module preview (ports, parameters)
//! - Registration in hierarchy

use std::path::PathBuf;
use std::sync::{mpsc, Arc, Mutex};

/// State for the Verilog-A model loading dialog
pub struct VerilogALoadDialogState {
    /// Whether the dialog is open
    pub open: bool,
    /// Selected file path
    pub file_path: Option<PathBuf>,
    /// Path input text (for editing)
    pub file_path_text: String,
    /// Compiler options
    pub options: VerilogADialogOptions,
    /// Compilation state
    pub compilation_state: CompilationState,
    /// Error messages from last attempt
    pub errors: Vec<CompileErrorDisplay>,
    /// Successfully compiled module info
    pub compiled_module: Option<CompiledModuleInfo>,
    /// Successfully compiled model artifact for simulation cache registration
    pub compiled_artifact: Option<rspice_veriloga::CompiledModel>,
    /// Canonical source/include dependencies captured during compilation
    pub compiled_dependencies: Option<Vec<PathBuf>>,
    /// Whether to show advanced options
    pub show_advanced_options: bool,
    /// Background compilation task receiver
    compile_task_receiver: Option<Arc<Mutex<mpsc::Receiver<CompileTaskResult>>>>,
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
            // Clone the Arc, not the receiver itself
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
            .field(
                "compile_task_receiver",
                &self.compile_task_receiver.is_some(),
            )
            .finish()
    }
}

impl VerilogALoadDialogState {
    /// Create a new default state
    pub fn new() -> Self {
        Self::default()
    }

    /// Reset the dialog to initial state
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Open the dialog (clears previous state)
    pub fn open(&mut self) {
        self.reset();
        self.open = true;
    }

    /// Close the dialog
    pub fn close(&mut self) {
        self.open = false;
        self.compilation_state = CompilationState::Idle;
    }

    /// Set the file path from a PathBuf
    pub fn set_file_path(&mut self, path: PathBuf) {
        self.file_path_text = path.to_string_lossy().to_string();
        self.file_path = Some(path);
        // Clear previous results when path changes
        self.errors.clear();
        self.compiled_module = None;
        self.compiled_artifact = None;
        self.compiled_dependencies = None;
        self.compilation_state = CompilationState::Idle;
    }

    /// Check if we can start compilation
    pub fn can_compile(&self) -> bool {
        self.file_path.is_some()
            && matches!(
                self.compilation_state,
                CompilationState::Idle | CompilationState::Failed
            )
    }

    /// Check if compilation succeeded
    pub fn is_success(&self) -> bool {
        matches!(self.compilation_state, CompilationState::Success)
    }
}

/// Compiler options configuration
#[derive(Debug, Clone, Default)]
pub struct VerilogADialogOptions {
    /// Include paths for `include directives
    pub include_paths: Vec<PathBuf>,
    /// Input field for new include path
    pub new_include_path: String,
    /// Preprocessor defines (name, value)
    pub defines: Vec<(String, String)>,
    /// Input field for new define name
    pub new_define_name: String,
    /// Input field for new define value
    pub new_define_value: String,
    /// Enable strict LRM compliance
    pub strict_mode: bool,
    /// Enable Verilog-AMS mixed-signal support
    pub enable_ams: bool,
}

impl VerilogADialogOptions {
    /// Add an include path
    pub fn add_include_path(&mut self, path: PathBuf) {
        if !path.as_os_str().is_empty() && !self.include_paths.contains(&path) {
            self.include_paths.push(path);
        }
    }

    /// Remove an include path by index
    pub fn remove_include_path(&mut self, index: usize) {
        if index < self.include_paths.len() {
            self.include_paths.remove(index);
        }
    }

    /// Add a preprocessor define
    pub fn add_define(&mut self, name: String, value: String) {
        if !name.is_empty() {
            // Remove existing define with same name
            self.defines.retain(|(n, _)| n != &name);
            self.defines.push((name, value));
        }
    }

    /// Remove a define by index
    pub fn remove_define(&mut self, index: usize) {
        if index < self.defines.len() {
            self.defines.remove(index);
        }
    }

    /// Convert to rspice-veriloga CompilerOptions
    #[cfg(feature = "veriloga")]
    pub fn to_compiler_options(&self) -> rspice_veriloga::CompilerOptions {
        rspice_veriloga::CompilerOptions {
            enable_ams: self.enable_ams,
            include_paths: self.include_paths.clone(),
            defines: self
                .defines
                .iter()
                .map(|(n, v)| (n.clone(), Some(v.clone())))
                .collect(),
            strict_mode: self.strict_mode,
            ..Default::default()
        }
    }
}

/// Compilation progress state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CompilationState {
    /// No compilation in progress
    #[default]
    Idle,
    /// Compilation is running in background
    Compiling,
    /// Compilation succeeded
    Success,
    /// Compilation failed with errors
    Failed,
}

/// Information about a successfully compiled module
#[derive(Debug, Clone)]
pub struct CompiledModuleInfo {
    /// Module name from the Verilog-A source
    pub name: String,
    /// Terminal (port) names
    pub ports: Vec<String>,
    /// Parameter definitions
    pub parameters: Vec<ParameterInfo>,
    /// Source file path
    pub source_path: PathBuf,
    /// Number of internal nodes
    pub internal_nodes: usize,
    /// Number of variables
    pub num_variables: usize,
}

impl CompiledModuleInfo {
    /// Create from a compiled model
    #[cfg(feature = "veriloga")]
    pub fn from_compiled_model(
        model: &rspice_veriloga::CompiledModel,
        source_path: PathBuf,
    ) -> Self {
        Self {
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
        }
    }

    /// Create a mock for testing without the veriloga feature
    #[cfg(not(feature = "veriloga"))]
    pub fn mock(name: &str, ports: &[&str], params: &[(&str, f64)]) -> Self {
        Self {
            name: name.to_string(),
            ports: ports.iter().map(|s| s.to_string()).collect(),
            parameters: params
                .iter()
                .map(|(n, v)| ParameterInfo {
                    name: n.to_string(),
                    default_value: format!("{}", v),
                    min: None,
                    max: None,
                    description: None,
                })
                .collect(),
            source_path: PathBuf::new(),
            internal_nodes: 0,
            num_variables: 0,
        }
    }
}

/// Parameter information for display
#[derive(Debug, Clone)]
pub struct ParameterInfo {
    /// Parameter name
    pub name: String,
    /// Default value formatted as string
    pub default_value: String,
    /// Minimum value constraint
    pub min: Option<f64>,
    /// Maximum value constraint
    pub max: Option<f64>,
    /// Optional description
    pub description: Option<String>,
}

impl ParameterInfo {
    /// Format the parameter range as a string
    pub fn range_str(&self) -> String {
        match (self.min, self.max) {
            (Some(min), Some(max)) => format!("[{}, {}]", min, max),
            (Some(min), None) => format!("[{}, ∞)", min),
            (None, Some(max)) => format!("(-∞, {}]", max),
            (None, None) => String::new(),
        }
    }
}

/// Error information for display
#[derive(Debug, Clone)]
pub struct CompileErrorDisplay {
    /// Error message
    pub message: String,
    /// Source file (if known)
    pub file: Option<String>,
    /// Line number (if known)
    pub line: Option<usize>,
    /// Column number (if known)
    pub column: Option<usize>,
    /// Error severity
    pub severity: ErrorSeverity,
}

impl CompileErrorDisplay {
    /// Create an error display
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            file: None,
            line: None,
            column: None,
            severity: ErrorSeverity::Error,
        }
    }

    /// Create a warning display
    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            file: None,
            line: None,
            column: None,
            severity: ErrorSeverity::Warning,
        }
    }

    /// Set the location information
    pub fn with_location(
        mut self,
        file: Option<String>,
        line: Option<usize>,
        column: Option<usize>,
    ) -> Self {
        self.file = file;
        self.line = line;
        self.column = column;
        self
    }

    /// Format the location string
    pub fn location_str(&self) -> String {
        match (&self.file, self.line, self.column) {
            (Some(f), Some(l), Some(c)) => format!("{}:{}:{}", f, l, c),
            (Some(f), Some(l), None) => format!("{}:{}", f, l),
            (Some(f), None, None) => f.clone(),
            (None, Some(l), Some(c)) => format!("line {}:{}", l, c),
            (None, Some(l), None) => format!("line {}", l),
            _ => String::new(),
        }
    }
}

/// Error severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    /// Error (compilation fails)
    Error,
    /// Warning (compilation may succeed)
    Warning,
    /// Note (informational)
    Note,
}

/// Background compile task handle
pub struct CompileTask {
    /// Channel receiver for result
    receiver: mpsc::Receiver<CompileTaskResult>,
}

impl CompileTask {
    /// Check if a result is available (non-blocking)
    pub fn try_recv(&self) -> Option<CompileTaskResult> {
        self.receiver.try_recv().ok()
    }
}

/// Result from background compilation
pub enum CompileTaskResult {
    /// Compilation succeeded
    Success {
        module_info: CompiledModuleInfo,
        compiled_model: Box<rspice_veriloga::CompiledModel>,
        dependencies: Vec<PathBuf>,
    },
    /// Compilation failed with errors
    Failure(Vec<CompileErrorDisplay>),
}

/// Dialog result indicating what action the user took
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerilogADialogResult {
    /// Dialog is still open, no action yet
    None,
    /// User cancelled the dialog
    Cancelled,
    /// User requested to add the model to library
    AddToLibrary,
}

// =============================================================================
// Rendering
// =============================================================================

use egui::{Context, RichText, Ui, Window};

/// Render the Verilog-A model loading dialog
///
/// Returns the dialog result indicating user action.
pub fn render_veriloga_load_dialog(
    ctx: &Context,
    state: &mut VerilogALoadDialogState,
) -> VerilogADialogResult {
    if !state.open {
        return VerilogADialogResult::None;
    }

    // Poll for async compilation result
    poll_compile(state);

    let mut result = VerilogADialogResult::None;
    let mut should_close = false;

    Window::new("Load Verilog-A Model")
        .resizable(true)
        .collapsible(false)
        .default_width(500.0)
        .default_height(400.0)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.spacing_mut().item_spacing.y = 8.0;

            // =========================================================
            // File Selection Section
            // =========================================================
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.strong("File:");
                    ui.add_space(8.0);

                    let text_edit = egui::TextEdit::singleline(&mut state.file_path_text)
                        .desired_width(350.0)
                        .hint_text("Path to .va file...");
                    if ui.add(text_edit).changed() {
                        // Update file_path when text changes
                        let path = PathBuf::from(&state.file_path_text);
                        if !state.file_path_text.is_empty() {
                            state.file_path = Some(path);
                        } else {
                            state.file_path = None;
                        }
                        // Clear previous results
                        state.errors.clear();
                        state.compiled_module = None;
                        state.compiled_artifact = None;
                        state.compilation_state = CompilationState::Idle;
                    }

                    if ui.button("Browse...").clicked() {
                        // Open native file dialog
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("Verilog-A", &["va", "vams"])
                            .pick_file()
                        {
                            state.set_file_path(path);
                        }
                    }
                });

                // Show file info if path is set
                if let Some(path) = &state.file_path {
                    if path.exists() {
                        let file_name = path
                            .file_name()
                            .map(|s| s.to_string_lossy())
                            .unwrap_or_default();
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("File: ").color(egui::Color32::GRAY));
                            ui.label(RichText::new(file_name).strong());
                        });
                    } else {
                        ui.horizontal(|ui| {
                            ui.colored_label(
                                egui::Color32::from_rgb(255, 100, 100),
                                "File not found",
                            );
                        });
                    }
                }
            });

            // =========================================================
            // Compiler Options Section (Collapsible)
            // =========================================================
            ui.add_space(4.0);
            egui::CollapsingHeader::new("Compiler Options")
                .default_open(state.show_advanced_options)
                .show(ui, |ui| {
                    state.show_advanced_options = true;
                    render_compiler_options(ui, &mut state.options);
                });

            // =========================================================
            // Compilation Status / Results Section
            // =========================================================
            ui.add_space(4.0);

            match state.compilation_state {
                CompilationState::Idle => {
                    // Show compile button
                    ui.horizontal(|ui| {
                        let can_compile = state.can_compile();
                        if ui
                            .add_enabled(can_compile, egui::Button::new("Compile"))
                            .clicked()
                        {
                            // Start async compilation
                            start_compile(state);
                        }

                        if !can_compile && state.file_path.is_none() {
                            ui.label(
                                RichText::new("Select a file first")
                                    .italics()
                                    .color(egui::Color32::GRAY),
                            );
                        }
                    });
                }
                CompilationState::Compiling => {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label("Compiling...");
                    });
                }
                CompilationState::Success => {
                    render_success_section(ui, state);
                }
                CompilationState::Failed => {
                    render_error_section(ui, state);
                }
            }

            // =========================================================
            // Action Buttons
            // =========================================================
            ui.add_space(8.0);
            ui.separator();
            ui.horizontal(|ui| {
                // Right-align buttons
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Cancel").clicked() {
                        should_close = true;
                        result = VerilogADialogResult::Cancelled;
                    }

                    if state.is_success() {
                        if ui.button("Add to Library").clicked() {
                            result = VerilogADialogResult::AddToLibrary;
                            should_close = true;
                        }
                    }
                });
            });
        });

    if should_close {
        state.close();
    }

    result
}

/// Render compiler options section
fn render_compiler_options(ui: &mut Ui, options: &mut VerilogADialogOptions) {
    ui.horizontal(|ui| {
        ui.checkbox(&mut options.strict_mode, "Strict LRM mode");
        ui.add_space(16.0);
        ui.checkbox(&mut options.enable_ams, "Enable Verilog-AMS");
    });

    ui.add_space(4.0);

    // Include paths
    ui.label(RichText::new("Include Paths:").strong().size(11.0));

    let mut remove_idx = None;
    for (idx, path) in options.include_paths.iter().enumerate() {
        ui.horizontal(|ui| {
            ui.add_space(8.0);
            ui.label(path.to_string_lossy().to_string());
            if ui.small_button("X").clicked() {
                remove_idx = Some(idx);
            }
        });
    }
    if let Some(idx) = remove_idx {
        options.remove_include_path(idx);
    }

    ui.horizontal(|ui| {
        ui.add_space(8.0);
        ui.add(
            egui::TextEdit::singleline(&mut options.new_include_path)
                .desired_width(250.0)
                .hint_text("Add include path..."),
        );
        if ui.button("+").clicked() && !options.new_include_path.is_empty() {
            options.add_include_path(PathBuf::from(&options.new_include_path));
            options.new_include_path.clear();
        }
    });

    ui.add_space(4.0);

    // Defines
    ui.label(RichText::new("Preprocessor Defines:").strong().size(11.0));

    let mut remove_def_idx = None;
    for (idx, (name, value)) in options.defines.iter().enumerate() {
        ui.horizontal(|ui| {
            ui.add_space(8.0);
            ui.label(format!("{} = {}", name, value));
            if ui.small_button("X").clicked() {
                remove_def_idx = Some(idx);
            }
        });
    }
    if let Some(idx) = remove_def_idx {
        options.remove_define(idx);
    }

    ui.horizontal(|ui| {
        ui.add_space(8.0);
        ui.add(
            egui::TextEdit::singleline(&mut options.new_define_name)
                .desired_width(100.0)
                .hint_text("Name"),
        );
        ui.label("=");
        ui.add(
            egui::TextEdit::singleline(&mut options.new_define_value)
                .desired_width(100.0)
                .hint_text("Value"),
        );
        if ui.button("+").clicked() && !options.new_define_name.is_empty() {
            let name = options.new_define_name.clone();
            let value = options.new_define_value.clone();
            options.add_define(name, value);
            options.new_define_name.clear();
            options.new_define_value.clear();
        }
    });
}

/// Render the success section with module preview
fn render_success_section(ui: &mut Ui, state: &VerilogALoadDialogState) {
    ui.colored_label(
        egui::Color32::from_rgb(100, 200, 100),
        "Compilation successful!",
    );

    if let Some(module) = &state.compiled_module {
        ui.add_space(4.0);
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.strong("Module:");
                ui.label(&module.name);
            });

            ui.horizontal(|ui| {
                ui.strong("Ports:");
                ui.label(module.ports.join(", "));
            });

            if !module.parameters.is_empty() {
                ui.add_space(4.0);
                ui.label(RichText::new("Parameters:").strong().size(11.0));

                egui::ScrollArea::vertical()
                    .max_height(120.0)
                    .show(ui, |ui| {
                        for param in &module.parameters {
                            ui.horizontal(|ui| {
                                ui.add_space(8.0);
                                ui.label(&param.name);
                                ui.label("=");
                                ui.label(&param.default_value);
                                let range = param.range_str();
                                if !range.is_empty() {
                                    ui.label(RichText::new(range).color(egui::Color32::GRAY));
                                }
                            });
                        }
                    });
            }

            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!(
                        "{} internal nodes, {} variables",
                        module.internal_nodes, module.num_variables
                    ))
                    .color(egui::Color32::GRAY)
                    .size(10.0),
                );
            });
            if let Some(deps) = &state.compiled_dependencies {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(format!(
                            "{} dependency file(s) captured for compile cache",
                            deps.len()
                        ))
                        .color(egui::Color32::GRAY)
                        .size(10.0),
                    );
                });
            }
        });
    }
}

/// Render error section with compile errors
fn render_error_section(ui: &mut Ui, state: &mut VerilogALoadDialogState) {
    ui.colored_label(
        egui::Color32::from_rgb(255, 100, 100),
        "Compilation failed!",
    );

    if !state.errors.is_empty() {
        ui.add_space(4.0);
        egui::ScrollArea::vertical()
            .max_height(150.0)
            .show(ui, |ui| {
                for err in &state.errors {
                    ui.horizontal(|ui| {
                        let icon = match err.severity {
                            ErrorSeverity::Error => RichText::new("[E]").color(egui::Color32::RED),
                            ErrorSeverity::Warning => {
                                RichText::new("[W]").color(egui::Color32::YELLOW)
                            }
                            ErrorSeverity::Note => RichText::new("[N]").color(egui::Color32::GRAY),
                        };
                        ui.label(icon);

                        let loc = err.location_str();
                        if !loc.is_empty() {
                            ui.label(RichText::new(format!("{}:", loc)).color(egui::Color32::GRAY));
                        }
                        ui.label(&err.message);
                    });
                }
            });
    }

    ui.add_space(4.0);
    if ui.button("Retry Compilation").clicked() {
        start_compile(state);
    }
}

/// Start async compilation using rspice-veriloga
fn start_compile(state: &mut VerilogALoadDialogState) {
    // Validate file path first
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

    // Check extension
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

    // Build compiler options
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

    // Create channel for result
    let (tx, rx) = mpsc::channel();
    let source_path = path.clone();

    // Spawn compilation thread
    std::thread::spawn(move || {
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
                let errors = vec![CompileErrorDisplay::error(e.to_string())];
                CompileTaskResult::Failure(errors)
            }
        };

        let _ = tx.send(task_result);
    });

    // Store receiver and set state to compiling
    state.compile_task_receiver = Some(Arc::new(Mutex::new(rx)));
    state.compilation_state = CompilationState::Compiling;
    state.errors.clear();
    state.compiled_module = None;
    state.compiled_artifact = None;
    state.compiled_dependencies = None;
}

/// Poll for compilation result (non-blocking)
fn poll_compile(state: &mut VerilogALoadDialogState) {
    if !matches!(state.compilation_state, CompilationState::Compiling) {
        return;
    }

    // Check if we have a task receiver
    let receiver = match &state.compile_task_receiver {
        Some(rx) => rx.clone(),
        None => return,
    };

    // Try to get result (non-blocking) - separate scope for the lock
    let received = if let Ok(guard) = receiver.try_lock() {
        match guard.try_recv() {
            Ok(result) => Some(result),
            Err(mpsc::TryRecvError::Empty) => None, // Still compiling
            Err(mpsc::TryRecvError::Disconnected) => Some(CompileTaskResult::Failure(vec![
                CompileErrorDisplay::error("Compilation thread disconnected unexpectedly"),
            ])),
        }
    } else {
        None // Lock not available, try again later
    };

    // Now update state based on received result
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

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dialog_state_default() {
        let state = VerilogALoadDialogState::default();
        assert!(!state.open);
        assert!(state.file_path.is_none());
        assert!(state.errors.is_empty());
        assert!(state.compiled_module.is_none());
        assert!(state.compiled_dependencies.is_none());
        assert_eq!(state.compilation_state, CompilationState::Idle);
    }

    #[test]
    fn test_dialog_state_open_and_close() {
        let mut state = VerilogALoadDialogState::default();

        state.open();
        assert!(state.open);

        state.close();
        assert!(!state.open);
    }

    #[test]
    fn test_dialog_state_set_file_path() {
        let mut state = VerilogALoadDialogState::default();
        let path = PathBuf::from("/test/model.va");

        state.set_file_path(path.clone());

        assert_eq!(state.file_path, Some(path.clone()));
        assert_eq!(state.file_path_text, "/test/model.va");
    }

    #[test]
    fn test_dialog_state_can_compile() {
        let mut state = VerilogALoadDialogState::default();

        // No path - cannot compile
        assert!(!state.can_compile());

        // With path - can compile
        state.file_path = Some(PathBuf::from("test.va"));
        assert!(state.can_compile());

        // While compiling - cannot compile
        state.compilation_state = CompilationState::Compiling;
        assert!(!state.can_compile());

        // After failure - can retry
        state.compilation_state = CompilationState::Failed;
        assert!(state.can_compile());

        // After success - need to reset first
        state.compilation_state = CompilationState::Success;
        assert!(!state.can_compile());
    }

    #[test]
    fn test_options_add_include_path() {
        let mut opts = VerilogADialogOptions::default();

        opts.add_include_path(PathBuf::from("/include1"));
        opts.add_include_path(PathBuf::from("/include2"));

        assert_eq!(opts.include_paths.len(), 2);

        // Adding duplicate should not add
        opts.add_include_path(PathBuf::from("/include1"));
        assert_eq!(opts.include_paths.len(), 2);

        // Empty path should not add
        opts.add_include_path(PathBuf::new());
        assert_eq!(opts.include_paths.len(), 2);
    }

    #[test]
    fn test_options_remove_include_path() {
        let mut opts = VerilogADialogOptions::default();
        opts.include_paths = vec![
            PathBuf::from("/a"),
            PathBuf::from("/b"),
            PathBuf::from("/c"),
        ];

        opts.remove_include_path(1);

        assert_eq!(opts.include_paths.len(), 2);
        assert_eq!(opts.include_paths[0], PathBuf::from("/a"));
        assert_eq!(opts.include_paths[1], PathBuf::from("/c"));
    }

    #[test]
    fn test_options_add_define() {
        let mut opts = VerilogADialogOptions::default();

        opts.add_define("FOO".to_string(), "1".to_string());
        opts.add_define("BAR".to_string(), "".to_string());

        assert_eq!(opts.defines.len(), 2);

        // Updating existing define replaces it
        opts.add_define("FOO".to_string(), "2".to_string());
        assert_eq!(opts.defines.len(), 2);
        assert_eq!(
            opts.defines.iter().find(|(n, _)| n == "FOO").unwrap().1,
            "2"
        );

        // Empty name should not add
        opts.add_define("".to_string(), "x".to_string());
        assert_eq!(opts.defines.len(), 2);
    }

    #[test]
    fn test_options_remove_define() {
        let mut opts = VerilogADialogOptions::default();
        opts.defines = vec![
            ("A".to_string(), "1".to_string()),
            ("B".to_string(), "2".to_string()),
        ];

        opts.remove_define(0);

        assert_eq!(opts.defines.len(), 1);
        assert_eq!(opts.defines[0].0, "B");
    }

    #[test]
    fn test_compile_error_display_formatting() {
        let err = CompileErrorDisplay::error("Syntax error").with_location(
            Some("model.va".to_string()),
            Some(42),
            Some(10),
        );

        assert_eq!(err.message, "Syntax error");
        assert_eq!(err.location_str(), "model.va:42:10");
        assert_eq!(err.severity, ErrorSeverity::Error);
    }

    #[test]
    fn test_parameter_info_range_str() {
        let p1 = ParameterInfo {
            name: "r".to_string(),
            default_value: "1000".to_string(),
            min: Some(0.0),
            max: Some(1e12),
            description: None,
        };
        assert_eq!(p1.range_str(), "[0, 1000000000000]");

        let p2 = ParameterInfo {
            name: "x".to_string(),
            default_value: "0".to_string(),
            min: Some(-1.0),
            max: None,
            description: None,
        };
        assert_eq!(p2.range_str(), "[-1, ∞)");

        let p3 = ParameterInfo {
            name: "y".to_string(),
            default_value: "0".to_string(),
            min: None,
            max: None,
            description: None,
        };
        assert_eq!(p3.range_str(), "");
    }

    #[test]
    fn test_file_path_validation_va_extension() {
        let valid_paths = [
            "/path/to/model.va",
            "relative/model.va",
            "C:\\Windows\\model.va",
        ];

        for path_str in valid_paths {
            let path = PathBuf::from(path_str);
            assert!(
                path.extension().map(|e| e == "va").unwrap_or(false),
                "Path {} should have .va extension",
                path_str
            );
        }
    }

    #[test]
    fn test_file_path_validation_nonexistent() {
        let path = PathBuf::from("/nonexistent/fake/model.va");
        // In production, we'd check existence before compilation
        assert!(!path.exists());
    }
}
