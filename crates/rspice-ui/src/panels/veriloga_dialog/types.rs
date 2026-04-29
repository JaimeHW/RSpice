use std::path::PathBuf;
use std::sync::mpsc;

/// Compilation progress state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CompilationState {
    /// No compilation in progress.
    #[default]
    Idle,
    /// Compilation is running in background.
    Compiling,
    /// Compilation succeeded.
    Success,
    /// Compilation failed with errors.
    Failed,
}

/// Information about a successfully compiled module.
#[derive(Debug, Clone)]
pub struct CompiledModuleInfo {
    /// Module name from the Verilog-A source.
    pub name: String,
    /// Terminal (port) names.
    pub ports: Vec<String>,
    /// Parameter definitions.
    pub parameters: Vec<ParameterInfo>,
    /// Source file path.
    pub source_path: PathBuf,
    /// Number of internal nodes.
    pub internal_nodes: usize,
    /// Number of variables.
    pub num_variables: usize,
}

impl CompiledModuleInfo {
    /// Create from a compiled model.
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

    /// Create a mock for testing without the veriloga feature.
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

/// Parameter information for display.
#[derive(Debug, Clone)]
pub struct ParameterInfo {
    /// Parameter name.
    pub name: String,
    /// Default value formatted as string.
    pub default_value: String,
    /// Minimum value constraint.
    pub min: Option<f64>,
    /// Maximum value constraint.
    pub max: Option<f64>,
    /// Optional description.
    pub description: Option<String>,
}

impl ParameterInfo {
    /// Format the parameter range as a string.
    pub fn range_str(&self) -> String {
        match (self.min, self.max) {
            (Some(min), Some(max)) => format!("[{}, {}]", min, max),
            (Some(min), None) => format!("[{}, \u{221e})", min),
            (None, Some(max)) => format!("(-\u{221e}, {}]", max),
            (None, None) => String::new(),
        }
    }
}

/// Error information for display.
#[derive(Debug, Clone)]
pub struct CompileErrorDisplay {
    /// Error message.
    pub message: String,
    /// Source file (if known).
    pub file: Option<String>,
    /// Line number (if known).
    pub line: Option<usize>,
    /// Column number (if known).
    pub column: Option<usize>,
    /// Error severity.
    pub severity: ErrorSeverity,
}

impl CompileErrorDisplay {
    /// Create an error display.
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            file: None,
            line: None,
            column: None,
            severity: ErrorSeverity::Error,
        }
    }

    /// Create a warning display.
    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            file: None,
            line: None,
            column: None,
            severity: ErrorSeverity::Warning,
        }
    }

    /// Set the location information.
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

    /// Format the location string.
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

/// Error severity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    /// Error (compilation fails).
    Error,
    /// Warning (compilation may succeed).
    Warning,
    /// Note (informational).
    Note,
}

/// Background compile task handle.
pub struct CompileTask {
    /// Channel receiver for result.
    receiver: mpsc::Receiver<CompileTaskResult>,
}

impl CompileTask {
    /// Check if a result is available (non-blocking).
    pub fn try_recv(&self) -> Option<CompileTaskResult> {
        self.receiver.try_recv().ok()
    }
}

/// Result from background compilation.
pub enum CompileTaskResult {
    /// Compilation succeeded.
    Success {
        module_info: CompiledModuleInfo,
        compiled_model: Box<rspice_veriloga::CompiledModel>,
        dependencies: Vec<PathBuf>,
    },
    /// Compilation failed with errors.
    Failure(Vec<CompileErrorDisplay>),
}

/// Dialog result indicating what action the user took.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerilogADialogResult {
    /// Dialog is still open, no action yet.
    None,
    /// User cancelled the dialog.
    Cancelled,
    /// User requested to add the model to library.
    AddToLibrary,
}
